use crate::actions::Action;
use crate::components::Component;
use crate::layouts::LayoutManager;
use anyhow::Result;
use ratatui::style::Color;
use ratatui::Frame;
use ratatui::{prelude::Stylize, widgets::Paragraph};
use std::time::UNIX_EPOCH;

pub struct TestComponent {
    colors: Vec<Color>,
    color: Color,
}

impl TestComponent {
    pub fn new() -> TestComponent {
        TestComponent {
            color: Color::DarkGray,
            colors: vec![Color::DarkGray, Color::Red, Color::Blue],
        }
    }
}

impl Component for TestComponent {
    fn init(&self) -> Result<()> {
        Ok(())
    }

    fn render(&self, f: &mut Frame<'_>, layout_manager: &LayoutManager) -> Result<()> {
        let rect = layout_manager.get_cheat_rect(f.size());
        f.render_widget(
            Paragraph::new("Hello world (press q to quit)")
                .white()
                .bold()
                .bg(self.color),
            rect,
        );
        Ok(())
    }

    fn update(&mut self, action: &Action) -> Action {
        match action {
            Action::ChangeColor => {
                let i = std::time::SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .subsec_millis()
                    % 3;
                self.color = self.colors.get(i as usize).expect("").clone();
                Action::Noop
            }
            _ => Action::Noop,
        }
    }
}
