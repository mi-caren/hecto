use crossterm::event::{ Event, KeyCode, KeyEvent, Event::Key, KeyModifiers, Event::Resize};

#[derive(Copy, Clone)]
pub struct Size {
    pub rows: usize,
    pub cols: usize,
}

pub enum Direction {
    PageUp,
    PageDown,
    Home,
    End,
    Up,
    Left,
    Right,
    Down,
}

pub enum EditorCommand {
    None,
    Move(Direction),
    Resize(Size),
    Quit,
}

impl From<Event> for EditorCommand {
    fn from(event: Event) -> EditorCommand {
        match event {
            Key(KeyEvent {
                code, modifiers, ..
            }) => match (code, modifiers) {
                (KeyCode::Char('q'), KeyModifiers::CONTROL) => Self::Quit,
                (KeyCode::Right, _) => Self::Move(Direction::Right),
                (KeyCode::Left, _) => Self::Move(Direction::Left),
                (KeyCode::Up, _) => Self::Move(Direction::Up),
                (KeyCode::Down, _) => Self::Move(Direction::Down),
                (KeyCode::PageUp, _) => Self::Move(Direction::PageUp),
                (KeyCode::PageDown, _) => Self::Move(Direction::PageDown),
                (KeyCode::Home, _) => Self::Move(Direction::Home),
                (KeyCode::End, _) => Self::Move(Direction::End),
                _ => Self::None,
            },
            Resize(cols, rows) => Self::Resize(Size { cols: cols as usize, rows: rows as usize }),
            _ => Self::None,
        }
    }
}
