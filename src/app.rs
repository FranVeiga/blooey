use crate::bluetooth::BluetoothManager;
use crate::components::test_component::TestComponent;
use crate::components::{alert, select_list::SelectList, Component};
use crate::events::{Event, EventHandler};
use crate::layouts::LayoutManager;
use crate::{actions::Action, tui::Tui};
use anyhow::Result;
use std::collections::VecDeque;
use std::sync::mpsc::{self, Receiver, Sender};

pub struct App {
    components: Vec<Box<dyn Component>>,
    active_alert: Option<alert::Alert>,
    should_quit: bool,
    actions_pending: VecDeque<Action>,
    tick_rate: u64,
    event_rx: Option<Receiver<Event>>,
    tui: Tui,
    layout_manager: LayoutManager,
    bluetooth_manager: BluetoothManager,
}

impl App {
    pub async fn new() -> App {
        App {
            components: Vec::new(),
            active_alert: None,
            should_quit: false,
            actions_pending: VecDeque::new(),
            tick_rate: 30,
            event_rx: None,
            tui: Tui::new(),
            layout_manager: LayoutManager::new(),
            bluetooth_manager: BluetoothManager::create().await.unwrap(),
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        let (event_tx, event_rx): (Sender<Event>, Receiver<Event>) = mpsc::channel();
        self.event_rx = Some(event_rx);

        let event_handler = EventHandler::new(event_tx.clone(), self.tick_rate);
        event_handler.start_event_polling();

        self.tui.enter()?;

        self.add_initial_components().await?;

        for c in self.components.iter_mut() {
            c.init()?;
        }

        loop {
            if self.should_quit {
                break;
            };

            if let Ok(e) = self
                .event_rx
                .as_ref()
                .expect("Should have an event channel")
                .recv()
            {
                match e {
                    Event::Quit => self.actions_pending.push_back(Action::Quit),
                    Event::ChangeColor => self.actions_pending.push_back(Action::ChangeColor),
                    Event::Down => self.actions_pending.push_back(Action::Down),
                    Event::Up => self.actions_pending.push_back(Action::Up),
                    Event::ConnectDevice => self
                        .actions_pending
                        .push_back(Action::SelectDeviceForConnection),
                    Event::DisconnectDevice => self
                        .actions_pending
                        .push_back(Action::SelectDeviceForDisconnection),
                    Event::Render => {
                        self.tui
                            .terminal
                            .draw(|frame| {
                                for component in self.components.iter_mut() {
                                    component.render(frame, &self.layout_manager).unwrap_or(());
                                }
                                if let Some(alert) = self.active_alert.as_ref() {
                                    alert.render(frame, &self.layout_manager).unwrap_or(());
                                }
                            })
                            .unwrap();
                    }
                    Event::Tick => {
                        while !self.actions_pending.is_empty() {
                            if let Some(action) = self.actions_pending.pop_front() {
                                match action {
                                    Action::Noop => continue,
                                    _ => (),
                                };
                                for component in self.components.iter_mut() {
                                    let a = component.update(&action);
                                    match a {
                                        Action::Noop => (),
                                        _ => self.actions_pending.push_back(a),
                                    }
                                }
                                self.handle_action(&action).await;
                            }
                        }
                    }
                }
            }
        }
        self.tui.exit()?;
        Ok(())
    }

    async fn add_initial_components(&mut self) -> Result<()> {
        self.components.push(Box::new(SelectList::new().await?));
        self.components.push(Box::new(TestComponent::new()));
        Ok(())
    }

    async fn handle_action(&mut self, action: &Action) {
        match action {
            Action::Quit => self.should_quit = true,

            Action::Connect(device) => {
                if let Err(_) = self.bluetooth_manager.connect_device(device.address).await {
                    self.alert("Failed to connect to device")
                }
            }

            Action::Disconnect(device) => {
                if let Err(_) = self
                    .bluetooth_manager
                    .disconnect_device(device.address)
                    .await
                {
                    self.alert("Failed to disconnect to device")
                }
            }

            Action::RemoveAlert => self.remove_alert(),

            _ => (),
        }
    }

    fn alert(&mut self, text: &str) {
        let alert = alert::Alert::new(String::from(text));
        self.active_alert = Some(alert);
    }

    fn remove_alert(&mut self) {
        self.active_alert = None
    }
}
