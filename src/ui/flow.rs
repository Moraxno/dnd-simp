use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};

pub struct KeyHandler {
    pub code: KeyCode,
    pub modifiers: KeyModifiers,
    pub command: String,
}


pub trait HandlesKeyEvents {
    fn get_handlers(&self) -> Vec<KeyHandler>;
}