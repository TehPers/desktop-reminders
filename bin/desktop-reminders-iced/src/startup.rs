use std::{error::Error, sync::Arc};

use iced_native::{renderer::Style, Color, Debug, Theme};
use iced_wgpu::Settings as WgpuSettings;
use iced_winit::{
    program::State,
    winit::{
        dpi::{LogicalSize, PhysicalPosition},
        event::{Event, ModifiersState, WindowEvent},
        event_loop::EventLoop,
        window::WindowBuilder,
    },
    Clipboard,
};
use tracing::{error, info, metadata::LevelFilter};
use tracing_error::ErrorLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Registry};

use crate::{backend::AppPainter, ui::app::App};

pub fn start() -> color_eyre::Result<()> {
    Registry::default()
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .with(ErrorLayer::default())
        .with(tracing_subscriber::fmt::layer().compact())
        .try_init()?;

    run()
}

fn run() -> color_eyre::Result<()> {
    let event_loop = EventLoop::default();
    let window = WindowBuilder::new()
        .with_title("Reminders")
        .with_inner_size(LogicalSize::new(300, 300))
        .with_decorations(true)
        .with_resizable(true)
        // .with_skip_taskbar(true)
        .build(&event_loop)?;
    let window = Arc::new(window);

    info!("Creating painter");
    let settings = WgpuSettings::from_env();
    let mut painter =
        futures::executor::block_on(unsafe { AppPainter::init(window.clone(), settings) })?;

    // Create program
    let program = App::default();
    let mut clipboard = Clipboard::connect(window.as_ref());
    let mut debug = Debug::new();

    // Setup event loop state
    let mut resize_requested = false;
    let mut cursor_pos = PhysicalPosition::default();
    let mut modifiers = ModifiersState::default();
    let mut state = State::new(
        program,
        painter.viewport().logical_size(),
        painter.renderer_mut(),
        &mut debug,
    );

    info!("Starting event loop");
    event_loop.run(move |event, _, control_flow| {
        control_flow.set_wait();

        match event {
            Event::WindowEvent { window_id, event } if window_id == window.id() => {
                // Handle the event
                match event {
                    WindowEvent::CloseRequested => control_flow.set_exit(),
                    WindowEvent::Resized(_) | WindowEvent::ScaleFactorChanged { .. } => {
                        resize_requested = true
                    }
                    WindowEvent::CursorMoved { position, .. } => cursor_pos = position,
                    WindowEvent::ModifiersChanged(new_modifiers) => modifiers = new_modifiers,
                    _ => {}
                }

                // Convert the winit event to an iced event
                let scale_factor = window.scale_factor();
                if let Some(event) =
                    iced_winit::conversion::window_event(&event, scale_factor, modifiers)
                {
                    state.queue_event(event);
                }
            }
            Event::MainEventsCleared if !state.is_queue_empty() => {
                // Update iced
                let (_remaining_events, _command) = state.update(
                    painter.viewport().logical_size(),
                    iced_winit::conversion::cursor_position(cursor_pos, window.scale_factor()),
                    painter.renderer_mut(),
                    &Theme::Dark,
                    &Style {
                        text_color: Color::WHITE,
                    },
                    &mut clipboard,
                    &mut debug,
                );

                // Redraw the window
                window.request_redraw();
            }
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                // Resize if needed
                if resize_requested {
                    painter.resize();
                    resize_requested = false;
                }

                // Redraw
                let clear_color = state.program().background_color();
                let overlay = debug.overlay();
                if let Err(error) = painter.redraw(clear_color, &overlay) {
                    let error: &dyn Error = error.as_ref();
                    error!(error, "aborting due to unrecoverable error");
                    control_flow.set_exit_with_code(1);
                    return;
                }

                // Update cursor icon
                window.set_cursor_icon(iced_winit::conversion::mouse_interaction(
                    state.mouse_interaction(),
                ));
            }
            _ => {}
        }
    })
}
