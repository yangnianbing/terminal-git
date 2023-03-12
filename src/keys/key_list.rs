use crossterm::event::{KeyCode, KeyModifiers, KeyEvent};

pub struct KeysList {
    pub tab_status: GituiKeyEvent,
}

pub struct GituiKeyEvent {
    pub code: KeyCode,
    pub modifiers: KeyModifiers,
}

impl GituiKeyEvent {
    pub const fn new(code: KeyCode, modifiers: KeyModifiers) -> Self {
        Self{ code, modifiers}
    }
}