#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// Clippy
#![cfg_attr(debug_assertions, warn(clippy::all, clippy::pedantic))]
#![cfg_attr(not(debug_assertions), deny(clippy::all, clippy::pedantic))]
#![allow(
    clippy::manual_strip,
    clippy::module_inception,
    clippy::module_name_repetitions,
    clippy::too_many_lines,
    clippy::wildcard_imports
)]

mod components;
mod models;
mod widgets;

fn main() -> color_eyre::Result<()> {
    use color_eyre::eyre::Context as _;
    use desktop_reminders_backend_windows::Backend;
    use tracing::metadata::LevelFilter;
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Registry};

    use crate::widgets::App;

    Registry::default()
        .with(tracing_subscriber::fmt::layer().compact())
        .with(if cfg!(debug_assertions) {
            LevelFilter::DEBUG
        } else {
            LevelFilter::INFO
        })
        .try_init()
        .wrap_err("failed to initialize tracing")?;

    let backend = Backend::new();
    let mut app = App::with_test_reminders();
    backend
        .run(move |ctx| app.show(ctx))
        .wrap_err("failed to run backend")
}
