mod actions;
mod app;
mod component;
mod events;
mod tui;
use anyhow::Result;

use app::App;

fn main() -> Result<()> {
    let mut app = App::new();
    app.run()?;
    Ok(())
}
