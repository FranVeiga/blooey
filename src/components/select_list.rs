use crate::layouts::LayoutManager;

use super::{Action, Component};
use anyhow::Result;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::prelude::Rect;
use ratatui::style::Stylize;
use ratatui::text::Line;
use ratatui::widgets::Paragraph;
use ratatui::Frame;

pub struct SelectList {
    contents: Vec<String>,
    selected_index: usize,
}

impl SelectList {
    pub fn new(contents: Vec<String>) -> SelectList {
        SelectList {
            contents,
            selected_index: 0,
        }
    }

    pub fn increment_index(&mut self) {
        let new_index = (self.contents.len() - 1).min(self.selected_index + 1);
        self.selected_index = new_index
    }

    pub fn decrement_index(&mut self) {
        let new_index = 0.max(self.selected_index as i64 - 1);
        self.selected_index = new_index as usize
    }
}

impl Component for SelectList {
    fn render(&self, f: &mut Frame<'_>, layout_manager: &LayoutManager) -> Result<()> {
        let rect = layout_manager.get_main_list_rect(f.size());
        let mut lines: Vec<Line> = Vec::new();
        for i in 0..self.contents.len() {
            if i == self.selected_index {
                lines.push(Line::from(vec![
                    "> ".white().bold().into(),
                    self.contents[i].clone().white().bold().into(),
                ]))
            } else {
                lines.push(self.contents[i].clone().white().into())
            }
        }
        f.render_widget(Paragraph::new(lines), rect);

        Ok(())
    }

    fn update(&mut self, action: &Action) -> Action {
        match action {
            Action::Up => {
                self.decrement_index();
                Action::Noop
            }
            Action::Down => {
                self.increment_index();
                Action::Noop
            }
            _ => Action::Noop,
        }
    }
}
