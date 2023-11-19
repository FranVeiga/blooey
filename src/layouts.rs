use ratatui::prelude::{Constraint, Direction, Layout, Rect};

pub struct LayoutManager {
    home_layout: Layout,
}

impl LayoutManager {
    pub fn new() -> LayoutManager {
        let home_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(80), Constraint::Percentage(20)]);
        LayoutManager { home_layout }
    }

    pub fn get_main_list_rect(&self, framesize: Rect) -> Rect {
        self.home_layout.split(framesize)[0]
    }

    pub fn get_cheat_rect(&self, framesize: Rect) -> Rect {
        self.home_layout.split(framesize)[1]
    }

    pub fn get_alert_layout(&self, framesize: Rect) -> Rect {
        let width = 20;
        let height = 5;
        let x = framesize.width / 2 - width / 2;
        let y = framesize.height / 2 - height / 2;
        Rect {
            x,
            y,
            width,
            height,
        }
    }
}
