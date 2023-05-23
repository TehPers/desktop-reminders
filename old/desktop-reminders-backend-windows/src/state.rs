use std::fmt::{Debug, Formatter};

use egui::FullOutput;
use egui_wgpu::{wgpu::PowerPreference, winit::Painter, WgpuConfiguration};
use egui_winit::{
    winit::{event_loop::EventLoopWindowTarget, window::Window},
    State,
};
use thiserror::Error;

use super::user_event::BackendUserEvent;

pub struct BackendState {
    pub winit_state: State,
    pub full_output: FullOutput,
    pub wgpu_painter: Painter,
}

impl BackendState {
    pub fn init(
        window: &Window,
        target: &EventLoopWindowTarget<BackendUserEvent>,
    ) -> Result<Self, BackendInitError> {
        let winit_state = State::new(target);
        let full_output = FullOutput::default();
        let wgpu_config = WgpuConfiguration {
            power_preference: PowerPreference::LowPower,
            ..Default::default()
        };
        let mut wgpu_painter = Painter::new(wgpu_config, 1, 0, false);

        // Initialize painter
        // SAFETY: `window` is valid and lives for the duration of the program
        futures::executor::block_on(unsafe { wgpu_painter.set_window(Some(window)) })?;

        Ok(Self {
            winit_state,
            full_output,
            wgpu_painter,
        })
    }
}

impl Debug for BackendState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BackendState").finish_non_exhaustive()
    }
}

/// An error that can occur during backend initialization.
#[derive(Debug, Error)]
pub enum BackendInitError {
    /// An error occurred while initializing the wgpu painter.
    #[error("failed to initialize wgpu")]
    WgpuInitFailed(#[from] egui_wgpu::WgpuError),
}
