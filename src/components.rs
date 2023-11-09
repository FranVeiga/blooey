use crate::actions::Action;
use crate::events::Event;
use crate::layouts::LayoutManager;
use anyhow::Result;
use ratatui::Frame;

pub mod select_list;
pub mod test_component;

pub trait Component {
    fn init(&self) -> Result<()> {
        Ok(())
    }
    fn handle_events(&self, e: Event) -> Action {
        match e {
            Event::Quit => Action::Quit,
            _ => Action::Noop,
        }
    }

    fn update(&mut self, action: &Action) -> Action {
        Action::Noop
    }

    fn render(&self, f: &mut Frame<'_>, layout_manager: &LayoutManager) -> Result<()>;
}
