use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use egui::Context;
use egui_winit::winit::{
    error::OsError,
    event::Event,
    event_loop::{EventLoop, EventLoopBuilder, EventLoopProxy},
    platform::windows::{WindowBuilderExtWindows, WindowExtWindows},
    window::{WindowBuilder, WindowButtons, WindowLevel},
};
use once_cell::sync::OnceCell;
use thiserror::Error;
use tracing::{error, info, info_span, instrument, trace};
use windows::{
    core::PWSTR,
    Win32::{
        Foundation::{HWND, MAX_PATH},
        UI::{
            Accessibility::{SetWinEventHook, HWINEVENTHOOK},
            WindowsAndMessaging::{
                GetClassNameW, GetWindow, SetWindowPos, EVENT_SYSTEM_FOREGROUND, GW_HWNDPREV,
                HWND_BOTTOM, HWND_TOPMOST, SWP_ASYNCWINDOWPOS, SWP_NOACTIVATE, SWP_NOMOVE,
                SWP_NOOWNERZORDER, SWP_NOSENDCHANGING, SWP_NOSIZE, WINEVENT_OUTOFCONTEXT,
                WINEVENT_SKIPOWNPROCESS,
            },
        },
    },
};

use super::{BackendState, BackendUserEvent};

/// A proxy for sending events to the event loop.
static EVENT_LOOP_PROXY: OnceCell<Arc<Mutex<EventLoopProxy<BackendUserEvent>>>> = OnceCell::new();

#[derive(Debug)]
pub struct Backend {
    event_loop: EventLoop<BackendUserEvent>,
}

impl Backend {
    pub fn new() -> Self {
        let event_loop = EventLoopBuilder::with_user_event().build();
        Self { event_loop }
    }

    pub fn run<F>(self, mut ui: F) -> Result<(), BackendError>
    where
        F: FnMut(&Context) + 'static,
    {
        let proxy = Arc::new(Mutex::new(self.event_loop.create_proxy()));
        EVENT_LOOP_PROXY
            .set(proxy)
            .map_err(|_| BackendError::EventLoopAlreadyCreated)?;

        let window = WindowBuilder::new()
            .with_window_level(WindowLevel::AlwaysOnBottom)
            .with_decorations(false)
            .with_enabled_buttons(WindowButtons::empty())
            .with_resizable(false)
            .with_skip_taskbar(true)
            .build(&self.event_loop)?;
        window.set_enable(false);

        // Create context
        let egui_ctx = Context::default();
        let proxy = Arc::new(Mutex::new(self.event_loop.create_proxy()));
        egui_ctx.set_request_repaint_callback(move || {
            let _ = proxy
                .clone()
                .lock()
                .unwrap()
                .send_event(BackendUserEvent::RequestRepaint);
        });

        // Track custom window events
        // SAFETY: win_event_hook should be safe, and parameters are valid
        let _hook = unsafe {
            SetWinEventHook(
                EVENT_SYSTEM_FOREGROUND,
                EVENT_SYSTEM_FOREGROUND,
                None,
                Some(win_event_hook),
                0,
                0,
                WINEVENT_OUTOFCONTEXT | WINEVENT_SKIPOWNPROCESS,
            )
        };

        // Run the event loop
        let mut state = None;
        let mut desktop_shown = false;
        self.event_loop.run(move |event, target, control_flow| {
            let _guard = info_span!("event_loop", ?event).entered();

            // Refresh every minute at most
            control_flow.set_wait_timeout(Duration::from_secs(1));

            // Force the window to be at the bottom
            // winit doesn't let us hook into WM_WINDOWPOSCHANGING, so we have to do
            // this every update
            let window_hwnd = HWND(window.hwnd());
            let flags = SWP_ASYNCWINDOWPOS
                | SWP_NOACTIVATE
                | SWP_NOMOVE
                | SWP_NOSIZE
                | SWP_NOOWNERZORDER
                | SWP_NOSENDCHANGING;

            if desktop_shown {
                window.set_window_level(WindowLevel::AlwaysOnTop);
                // SAFETY: Valid HWNDs are being passed to the functions
                unsafe {
                    SetWindowPos(window_hwnd, HWND_TOPMOST, 0, 0, 0, 0, flags);
                }
                loop {
                    // Get previous window in z-order
                    let prev_hwnd = unsafe { GetWindow(window_hwnd, GW_HWNDPREV) };
                    if prev_hwnd == HWND::default() {
                        break;
                    }

                    // Try to position the window after the found window
                    let failed = unsafe { SetWindowPos(window_hwnd, prev_hwnd, 0, 0, 0, 0, flags) };
                    if failed.as_bool() {
                        break;
                    }
                }
            } else {
                window.set_window_level(WindowLevel::AlwaysOnBottom);
                // SAFETY: Valid HWNDs are being passed to the functions
                unsafe {
                    SetWindowPos(window_hwnd, HWND_BOTTOM, 0, 0, 0, 0, flags);
                }
            }

            // Init backend state if needed
            // TODO: do this in Resume?
            let state = state.get_or_insert_with(|| {
                info!("initializing backend state");
                BackendState::init(&window, target).expect("failed to initialize backend state")
            });

            // Handle event
            match event {
                Event::WindowEvent { window_id, event } if window_id == window.id() => {
                    // Process event with egui_winit
                    let response = state.winit_state.on_event(&egui_ctx, &event);
                    if response.repaint {
                        window.request_redraw();
                    }
                }
                Event::UserEvent(event) => match event {
                    BackendUserEvent::RequestRepaint => window.request_redraw(),
                    BackendUserEvent::DesktopShown if !desktop_shown => {
                        info!("desktop shown");
                        desktop_shown = true;
                    }
                    BackendUserEvent::DesktopHidden if desktop_shown => {
                        info!("desktop hidden");
                        desktop_shown = false;
                    }
                    BackendUserEvent::DesktopShown | BackendUserEvent::DesktopHidden => {}
                },
                Event::MainEventsCleared => {
                    // Run app logic
                    let raw_input = state.winit_state.take_egui_input(&window);
                    state.full_output.append(egui_ctx.run(raw_input, &mut ui));
                }
                Event::RedrawRequested(window_id) if window_id == window.id() => {
                    // Paint using egui_wgpu
                    let full_output = std::mem::take(&mut state.full_output);
                    let clipped_primitives = egui_ctx.tessellate(full_output.shapes);
                    state.wgpu_painter.paint_and_update_textures(
                        egui_ctx.pixels_per_point(),
                        egui_ctx
                            .style()
                            .visuals
                            .window_fill()
                            .to_normalized_gamma_f32(),
                        &clipped_primitives,
                        &full_output.textures_delta,
                    );
                }
                _ => {}
            }
        })
    }
}

/// An error that occurred while running the backend.
#[derive(Debug, Error)]
pub enum BackendError {
    /// The event loop has already been created.
    #[error("event loop has already been created")]
    EventLoopAlreadyCreated,
    /// An error occurred while creating the window.
    #[error("failed to create window")]
    CreateWindow(#[from] OsError),
}

#[instrument(skip_all, fields(hwnd, event))]
unsafe extern "system" fn win_event_hook(
    hwineventhook: HWINEVENTHOOK,
    event: u32,
    hwnd: HWND,
    idobject: i32,
    idchild: i32,
    ideventthread: u32,
    dwmseventtime: u32,
) {
    win_event_hook_safe(
        hwineventhook,
        event,
        hwnd,
        idobject,
        idchild,
        ideventthread,
        dwmseventtime,
    );
}

fn win_event_hook_safe(
    _hwineventhook: HWINEVENTHOOK,
    event: u32,
    hwnd: HWND,
    _idobject: i32,
    _idchild: i32,
    _ideventthread: u32,
    _dwmseventtime: u32,
) {
    // Check if the event is foreground change
    if event != EVENT_SYSTEM_FOREGROUND {
        return;
    }

    // Get the event loop proxy
    let Some(proxy) = EVENT_LOOP_PROXY.get() else {
        return;
    };

    // Read window class name
    let mut class_name = [0; MAX_PATH as usize];
    // SAFETY: hwnd is valid
    let len = unsafe { GetClassNameW(hwnd, &mut class_name) };
    if len == 0 {
        error!("failed to get class name");
        return;
    }

    // Convert to string
    // SAFETY: pointer is to valid null-terminated u16 buffer
    let class_name = unsafe { PWSTR(class_name.as_mut_ptr()).to_string() };
    let Ok(class_name) = class_name else {
        error!("class name is not valid UTF-16")                ;
        return;
    };

    // Check if the name is WorkerW
    let event = if class_name == "WorkerW" {
        BackendUserEvent::DesktopShown
    } else {
        BackendUserEvent::DesktopHidden
    };
    trace!(?class_name, ?event, "foreground changed");

    // Send event
    let Ok(proxy) = proxy.lock() else {
        error!("proxy mutex is poisoned");
        return;
    };
    let _ = proxy.send_event(event);
}
