use anyhow::Result;
use crossterm::event::Event as CrosstermEvent;
use crossterm::event::{self, KeyCode, KeyEvent, KeyEventKind};
use std::sync::mpsc;

pub enum Event {
    Quit,
}

pub struct BackendEventHandler {
    // event_tx: mpsc::Sender<Event>,
    poll_timeout: std::time::Duration,
}

impl BackendEventHandler {
    pub fn new(poll_timeout: std::time::Duration) -> BackendEventHandler {
        BackendEventHandler { poll_timeout }
    }

    pub fn poll_events(&self) -> Result<Option<Event>> {
        let mut response_event = None;
        if event::poll(std::time::Duration::from_millis(16))? {
            let e = event::read()?;
            response_event = self.handle_crossterm_event(e)?;
        }
        return Ok(response_event);
    }

    pub fn handle_crossterm_event(&self, e: CrosstermEvent) -> Result<Option<Event>> {
        let app_event = match e {
            CrosstermEvent::Key(key_event) => self.handle_crossterm_key_event(key_event)?,
            _ => None,
        };
        Ok(app_event)
    }

    pub fn handle_crossterm_key_event(&self, e: KeyEvent) -> Result<Option<Event>> {
        let app_event = match e.code {
            KeyCode::Char('q') => Some(Event::Quit),
            KeyCode::Char('Q') => Some(Event::Quit),
            _ => None,
        };
        Ok(app_event)
    }
}
