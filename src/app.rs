use crate::components::test_component::TestComponent;
use crate::events::{Event, EventHandler};
use crate::layouts::LayoutManager;
use crate::{actions::Action, tui::Tui};
use crate::{components::select_list::SelectList, components::Component};
use anyhow::Result;
use std::collections::VecDeque;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

pub struct App {
    components: Vec<Box<dyn Component>>,
    should_quit: bool,
    actions_pending: VecDeque<Action>,
    tick_rate: u64,
    event_rx: Option<Receiver<Event>>,
    tui: Tui,
    layout_manager: LayoutManager,
}

impl App {
    pub fn new() -> App {
        App {
            components: Vec::new(),
            should_quit: false,
            actions_pending: VecDeque::new(),
            tick_rate: 10,
            event_rx: None,
            tui: Tui::new(),
            layout_manager: LayoutManager::new(),
        }
    }

    pub fn run(&mut self) -> Result<()> {
        let (event_tx, event_rx): (Sender<Event>, Receiver<Event>) = mpsc::channel();
        self.event_rx = Some(event_rx);

        let mut event_handler = EventHandler::new(event_tx);

        let _evth_handle = thread::spawn(move || {
            event_handler.start_event_polling();
        });
        self.tui.enter()?;

        self.add_initial_components()?;

        for c in self.components.iter_mut() {
            c.init()?;
        }

        loop {
            if self.should_quit {
                break;
            };

            self.handle_events();

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
                    self.handle_action(&action)
                }
            }

            thread::sleep(std::time::Duration::from_millis(self.tick_rate));
        }
        self.tui.exit()?;
        Ok(())
    }

    fn add_initial_components(&mut self) -> Result<()> {
        self.components.push(Box::new(SelectList::new(vec![
            "Hola".into(),
            "Que".into(),
            "Tal".into(),
            "Testing".into(),
        ])));
        self.components.push(Box::new(TestComponent::new()));
        Ok(())
    }

    fn handle_action(&mut self, action: &Action) {
        match action {
            Action::Quit => self.should_quit = true,
            _ => (),
        }
    }

    fn handle_events(&mut self) {
        if let Ok(e) = self
            .event_rx
            .as_ref()
            .expect("Couldn't listen to event channel")
            .try_recv()
        {
            match e {
                Event::Quit => self.actions_pending.push_back(Action::Quit),
                Event::Render => {
                    self.tui
                        .terminal
                        .draw(|frame| {
                            for component in self.components.iter_mut() {
                                component.render(frame, &self.layout_manager).unwrap_or(());
                            }
                        })
                        .unwrap();
                }
                Event::ChangeColor => self.actions_pending.push_back(Action::ChangeColor),
                Event::Down => self.actions_pending.push_back(Action::Down),
                Event::Up => self.actions_pending.push_back(Action::Up),
            }
        }
    }
}
