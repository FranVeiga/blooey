use crossterm::event::Event as CrosstermEvent;
use crossterm::event::{self, KeyCode, KeyEvent};
use std::sync::mpsc::Sender;
use std::time::Instant;

pub enum Event {
    Quit,
    Render,
    ChangeColor,
    Down,
    Up,
    Tick,
    ConnectDevice,
    DisconnectDevice,
}

pub struct EventHandler {
    tick_rate: u64,
    should_quit: bool,
    pub event_tx: Sender<Event>,
    prev_tick: Instant,
}

impl EventHandler {
    pub fn new(event_tx: Sender<Event>, tick_rate: u64) -> EventHandler {
        EventHandler {
            should_quit: false,
            tick_rate,
            event_tx,
            prev_tick: Instant::now(),
        }
    }

    pub fn start_event_polling(mut self) {
        std::thread::spawn(move || loop {
            let t = Instant::now();
            let last_tick = Instant::now();
            self.event_tx
                .send(Event::Render)
                .unwrap_or_else(|e| eprintln!("Failed to send event: {}", e));
            let timeout = std::time::Duration::from_millis(self.tick_rate)
                .saturating_sub(last_tick.elapsed());

            self.prev_tick = t;

            if event::poll(timeout).unwrap_or(false) {
                if let Ok(e) = event::read() {
                    match self.handle_crossterm_event(e) {
                        Some(event) => self
                            .event_tx
                            .send(event)
                            .unwrap_or_else(|e| eprintln!("Failed to send event: {}", e)),
                        None => (),
                    }
                }
            }

            if last_tick.elapsed() >= std::time::Duration::from_millis(self.tick_rate) {
                self.event_tx
                    .send(Event::Tick)
                    .unwrap_or_else(|e| eprintln!("Failed to send event: {}", e));
            }
        });
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
            KeyCode::Char('r') => Some(Event::ChangeColor),
            KeyCode::Char('j') => Some(Event::Down),
            KeyCode::Char('k') => Some(Event::Up),
            KeyCode::Char('c') => Some(Event::ConnectDevice),
            KeyCode::Char('d') => Some(Event::DisconnectDevice),
            _ => None,
        }
    }

    pub fn quit(&mut self) -> Option<Event> {
        self.should_quit = true;
        Some(Event::Quit)
    }
}
