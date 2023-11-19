use super::Component;
use crate::{actions::Action, events::Event, layouts::LayoutManager};
use anyhow::Result;
use ratatui::{
    text::Text,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct Alert {
    text: String,
}

impl Alert {
    pub fn new(text: String) -> Alert {
        Alert { text }
    }
}

impl Component for Alert {
    fn render(&self, f: &mut Frame<'_>, layout_manager: &LayoutManager) -> Result<()> {
        let rect = layout_manager.get_alert_layout(f.size());
        let block = Block::default().borders(Borders::ALL);

        f.render_widget(Paragraph::new(self.text.clone()).block(block), rect);

        Ok(())
    }

    fn handle_events(&self, e: Event) -> Action {
        match e {
            Event::Render => Action::Noop,
            Event::Tick => Action::Noop,
            _ => Action::RemoveAlert,
        }
    }
}
