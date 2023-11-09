use crate::component::Component;
use crate::events::{BackendEventHandler, Event};
use crate::tui::Tui;
use anyhow::Result;
use ratatui::{prelude::Stylize, widgets::Paragraph};

pub struct App {
    components: Vec<Box<dyn Component>>,
}

impl App {
    pub fn new() -> App {
        App { components: vec![] }
    }

    pub fn run(&mut self) -> Result<()> {
        let mut tui = Tui::new();
        let event_handler = BackendEventHandler::new(std::time::Duration::from_millis(16));
        tui.enter()?;

        self.add_initial_components()?;

        for c in self.components.iter_mut() {
            c.init()?;
        }

        loop {
            tui.terminal.draw(|frame| {
                let area = frame.size();
                frame.render_widget(
                    Paragraph::new("Hello world (press q to quit)")
                        .fg(ratatui::style::Color::White)
                        .bold()
                        .on_dark_gray(),
                    area,
                )
            })?;

            // Change to use BackendEventHandler
            if let Some(e) = event_handler.poll_events()? {
                match e {
                    Event::Quit => break,
                }
            }
        }
        tui.exit()?;
        Ok(())
    }

    fn add_initial_components(&self) -> Result<()> {
        Ok(())
    }
}
