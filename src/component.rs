use crate::actions::Action;
use crate::events::Event;
use anyhow::Result;
use ratatui::prelude::Rect;
use ratatui::Frame;

pub trait Component {
    fn init(&self) -> Result<()> {
        Ok(())
    }
    fn handle_events(&self, e: Event) -> Action {
        match e {
            Event::Quit => Action::Quit,
        }
    }

    #[allow(unused_variables)]
    fn update(&self, action: Action) -> Action {
        Action::Noop
    }

    #[allow(unused_variables)]
    fn render(&self, f: Frame<'_>, rect: Rect) -> Result<()>;
}
