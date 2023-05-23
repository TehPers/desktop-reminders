mod backend;
mod models;
mod startup;
mod ui;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    crate::startup::start()
}
