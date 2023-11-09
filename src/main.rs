mod actions;
mod app;
mod components;
mod events;
mod layouts;
mod tui;
use anyhow::Result;

use app::App;

fn main() -> Result<()> {
    let mut app = App::new();
    app.run()?;
    Ok(())
}
