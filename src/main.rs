mod actions;
mod app;
mod bluetooth;
mod components;
mod events;
mod layouts;
mod tui;
use anyhow::Result;

use app::App;

#[tokio::main]
async fn main() -> Result<()> {
    let mut app = App::new().await;
    app.run().await?;
    Ok(())
}
