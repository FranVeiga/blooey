use super::{Action, Component};
use crate::bluetooth::{BluetoothManager, DeviceListItem};
use crate::layouts::LayoutManager;
use anyhow::Result;
use ratatui::prelude::Alignment;
use ratatui::style::{Color, Stylize};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

pub struct SelectList {
    bluetooth_manager: BluetoothManager,
    devices: Vec<DeviceListItem>,
    selected_index: usize,
}

impl SelectList {
    pub async fn new() -> Result<SelectList> {
        let bluetooth_manager = BluetoothManager::create().await?;
        let contents = bluetooth_manager.discovered_devices_list().await?;
        Ok(SelectList {
            devices: contents,
            selected_index: 0,
            bluetooth_manager,
        })
    }

    fn increment_index(&mut self) {
        let new_index = (self.devices.len() - 1).min(self.selected_index + 1);
        self.selected_index = new_index
    }

    fn decrement_index(&mut self) {
        let new_index = 0.max(self.selected_index as i64 - 1);
        self.selected_index = new_index as usize
    }

    fn update_device_list(&mut self) -> Result<()> {
        // for item in self.devices.iter() {
        //     let device = self.bluetooth_manager.get_device(item)?;
        // }

        Ok(())
    }
}

impl Component for SelectList {
    fn render(&self, f: &mut Frame<'_>, layout_manager: &LayoutManager) -> Result<()> {
        let rect = layout_manager.get_main_list_rect(f.size());
        let block = Block::default().borders(Borders::ALL).title(
            ratatui::widgets::block::Title::from("Bluetooth devices").alignment(Alignment::Left),
        );
        let mut items: Vec<Line> = Vec::new();
        for i in 0..self.devices.len() {
            let color = if self.devices[i].is_connected {
                Color::Green
            } else {
                Color::White
            };
            let device_name = self.devices[i].name.clone().fg(color);
            if i == self.selected_index {
                items.push(Line::from(vec![
                    "> ".bold().into(),
                    device_name.bold().into(),
                ]))
            } else {
                items.push(device_name.into())
            }
        }
        f.render_widget(Paragraph::new(items).block(block), rect);

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
            Action::SelectDeviceForConnection => {
                let device = self
                    .devices
                    .get(self.selected_index)
                    .expect("Should only select available items");
                Action::Connect(device.clone())
            }
            Action::SelectDeviceForDisconnection => {
                let device = self
                    .devices
                    .get(self.selected_index)
                    .expect("Should only select available items");
                Action::Disconnect(device.clone())
            }
            Action::UpdateDeviceList => self.update_device_list(),

            _ => Action::Noop,
        }
    }
}
