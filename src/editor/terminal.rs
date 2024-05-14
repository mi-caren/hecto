use crossterm::event;
use crossterm::execute;
use crossterm::cursor::MoveTo;
use crossterm::terminal;

use std::io::Error;
use std::io::stdout;

pub struct Terminal {}

impl Terminal {
    pub fn initialize() -> Result<(), Error> {
        terminal::enable_raw_mode()?;
        Self::clear_screen()
    }

    pub fn terminate() -> Result<(), Error> {
        terminal::disable_raw_mode()
    }

    pub fn clear_screen() -> Result<(), Error> {
        execute!(stdout(), terminal::Clear(terminal::ClearType::All))
    }

    pub fn read() -> Result<event::Event, Error> {
        event::read()
    }

    pub fn size() -> Result<(u16, u16), Error> {
        terminal::size()
    }

    pub fn move_cursor_to(col: u16, row: u16) -> Result<(), Error> {
        execute!(stdout(), MoveTo(col, row))
    }
}
