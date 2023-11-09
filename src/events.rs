use crossterm::event::Event as CrosstermEvent;
use crossterm::event::{self, KeyCode, KeyEvent};
use std::sync::mpsc::Sender;

pub enum Event {
    Quit,
    Render,
    ChangeColor,
    Down,
    Up,
}

pub struct EventHandler {
    poll_timeout: std::time::Duration,
    should_quit: bool,
    pub event_tx: Sender<Event>,
}

impl EventHandler {
    pub fn new(event_tx: Sender<Event>) -> EventHandler {
        EventHandler {
            poll_timeout: std::time::Duration::from_millis(16),
            should_quit: false,
            event_tx,
        }
    }

    pub fn start_event_polling(&mut self) {
        loop {
            if event::poll(self.poll_timeout).unwrap_or(false) {
                if let Ok(e) = event::read() {
                    match self.handle_crossterm_event(e) {
                        Some(event) => self
                            .event_tx
                            .send(event)
                            .unwrap_or_else(|e| eprintln!("Failed to send event: {}", e)),
                        None => (),
                    }
                }
            } else {
                self.event_tx
                    .send(Event::Render)
                    .unwrap_or_else(|e| eprintln!("Failed to send event: {}", e));
            };
        }
    }

    pub fn handle_crossterm_event(&mut self, e: CrosstermEvent) -> Option<Event> {
        match e {
            CrosstermEvent::Key(key_event) => self.handle_crossterm_key_event(key_event),
            _ => None,
        }
    }

    pub fn handle_crossterm_key_event(&mut self, e: KeyEvent) -> Option<Event> {
        match e.code {
            KeyCode::Char('q') => self.quit(),
            KeyCode::Char('Q') => self.quit(),
            KeyCode::Char('c') => Some(Event::ChangeColor),
            KeyCode::Char('j') => Some(Event::Down),
            KeyCode::Char('k') => Some(Event::Up),
            _ => None,
        }
    }

    pub fn quit(&mut self) -> Option<Event> {
        self.should_quit = true;
        Some(Event::Quit)
    }
}
