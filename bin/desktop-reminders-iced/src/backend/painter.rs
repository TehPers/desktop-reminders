use std::{error::Error, sync::Arc};

use color_eyre::eyre::{bail, ContextCompat};
use iced_native::{Color, Size};
use iced_wgpu::{
    wgpu::{
        util::StagingBelt, Adapter, Backends, CommandEncoderDescriptor, CompositeAlphaMode, Device,
        DeviceDescriptor, Features, Instance, InstanceDescriptor, Limits, LoadOp, Operations,
        PresentMode, Queue, RenderPassColorAttachment, RenderPassDescriptor, Surface,
        SurfaceConfiguration, SurfaceError, TextureFormat, TextureUsages,
    },
    Backend, Renderer, Settings as WgpuSettings, Viewport,
};
use iced_winit::winit::window::Window;
use tracing::error;

macro_rules! label {
    ($name:expr) => {
        concat!(
            env!("CARGO_PKG_NAME"),
            "@",
            env!("CARGO_PKG_VERSION"),
            "-",
            $name
        )
    };
}

/// A painter for the application.
pub struct AppPainter {
    surface: Surface,
    _adapter: Adapter,
    device: Device,
    queue: Queue,
    renderer: Renderer,
    staging_belt: StagingBelt,

    format: TextureFormat,
    viewport: Viewport,

    // SAFETY: this must be dropped after the surface
    window: Arc<Window>,
}

impl AppPainter {
    /// Create a new painter.
    ///
    /// # Safety
    ///
    /// - `window` must be a valid object to create a surface from.
    pub async unsafe fn init(
        window: Arc<Window>,
        settings: WgpuSettings,
    ) -> color_eyre::Result<Self> {
        // Create the instance
        let backends = iced_wgpu::wgpu::util::backend_bits_from_env().unwrap_or(Backends::PRIMARY);
        let instance = Instance::new(InstanceDescriptor {
            backends,
            ..Default::default()
        });

        // Create the surface
        // SAFETY: window outlives this painter (and thus the surface)
        let surface = unsafe { instance.create_surface(window.as_ref()) }?;

        // Create the adapter and device
        let adapter = iced_wgpu::wgpu::util::initialize_adapter_from_env_or_default(
            &instance,
            backends,
            Some(&surface),
        )
        .await
        .wrap_err("failed to initialize adapter")?;
        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    label: Some(label!("device")),
                    features: adapter.features() & Features::default(),
                    limits: Limits::downlevel_defaults(),
                },
                None,
            )
            .await?;

        // Get a supported texture format
        let capabilities = surface.get_capabilities(&adapter);
        let format = capabilities
            .formats
            .iter()
            .copied()
            .find(|&format| format != format.remove_srgb_suffix())
            .or_else(|| capabilities.formats.first().copied())
            .wrap_err("failed to find supported texture format")?;

        // Create renderer for iced
        let renderer = Renderer::new(Backend::new(&device, settings, format));

        // Create the painter
        let mut painter = Self {
            surface,
            _adapter: adapter,
            device,
            queue,
            renderer,
            staging_belt: StagingBelt::new(10 * 1024),

            format,
            viewport: Viewport::with_physical_size(Size::new(0, 0), 1.0),

            window,
        };

        // Configure the surface
        painter.resize();

        Ok(painter)
    }

    /// Gets a reference to the current viewport.
    pub fn viewport(&self) -> &Viewport {
        &self.viewport
    }

    /// Gets a mutable reference to the renderer.
    pub fn renderer_mut(&mut self) -> &mut Renderer {
        &mut self.renderer
    }

    /// Resizes the painter.
    pub fn resize(&mut self) {
        let size = self.window.inner_size();
        self.viewport = Viewport::with_physical_size(
            Size::new(size.width, size.height),
            self.window.scale_factor(),
        );
        self.surface.configure(
            &self.device,
            &SurfaceConfiguration {
                usage: TextureUsages::RENDER_ATTACHMENT,
                format: self.format,
                width: size.width,
                height: size.height,
                present_mode: PresentMode::AutoVsync,
                alpha_mode: CompositeAlphaMode::Auto,
                view_formats: vec![],
            },
        );
    }

    /// Redraws to the window.
    pub fn redraw<D>(&mut self, clear_color: Color, overlay: &[D]) -> color_eyre::Result<()>
    where
        D: AsRef<str>,
    {
        let frame = match self.surface.get_current_texture() {
            Ok(texture) => texture,
            Err(error @ SurfaceError::OutOfMemory) => {
                bail!(error);
            }
            Err(error) => {
                let error = &error as &dyn Error;
                error!(error);

                self.window.request_redraw();
                return Ok(());
            }
        };

        // Create the command encoder
        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some(label!("encoder")),
            });
        let view = frame.texture.create_view(&Default::default());

        // Clear the frame
        encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some(label!("pass-main")),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear({
                        let [r, g, b, a] = clear_color.into_linear();
                        iced_wgpu::wgpu::Color {
                            r: r.into(),
                            g: g.into(),
                            b: b.into(),
                            a: a.into(),
                        }
                    }),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });

        self.renderer.with_primitives(|backend, primitives| {
            backend.present(
                &self.device,
                &mut self.staging_belt,
                &mut encoder,
                &view,
                primitives,
                &self.viewport,
                overlay,
            );
        });

        // Submit work
        self.staging_belt.finish();
        self.queue.submit(Some(encoder.finish()));
        frame.present();

        // Recall staging buffers
        self.staging_belt.recall();

        Ok(())
    }
}
