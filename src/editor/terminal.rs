use crossterm::{queue, Command};
use crossterm::cursor::{Hide, Show, MoveTo};
use crossterm::terminal;
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType};


use std::io::Error;
use std::io::stdout;
use std::io::Write;
use std::fmt::Display;

pub struct Terminal {
    pub size: Size,
    pub cursor: Cursor,
}

pub struct Size {
    pub rows: u16,
    pub cols: u16,
}

pub struct Cursor {
    pub position: Position,
}

#[derive(Copy, Clone)]
pub struct Position {
    pub row: u16,
    pub col: u16,
}

impl Terminal {
    pub fn default() -> Self {
        let terminal_size = terminal::size().unwrap();
        Self {
            size: Size {
                rows: terminal_size.1,
                cols: terminal_size.0,
            },
            cursor: Cursor {
                position: Position {
                    row: 0,
                    col: 0,
                }
            }
        }
    }

    pub fn initialize() -> Result<(), Error> {
        terminal::enable_raw_mode()?;
        Self::hide_cursor()?;
        Self::clear_screen()?;
        Self::show_cursor()?;
        stdout().flush()?;
        Ok(())
    }

    pub fn show_cursor() -> Result<(), Error> {
        Self::queue_command(Show)
    }

    pub fn hide_cursor() -> Result<(), Error> {
        Self::queue_command(Hide)
    }

    pub fn print<T: Display>(str: T) -> Result<(), Error> {
        Self::queue_command(Print(str))
    }

    pub fn clear_line() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::CurrentLine))
    }

    pub fn terminate() -> Result<(), Error> {
        terminal::disable_raw_mode()
    }

    pub fn clear_screen() -> Result<(), Error> {
        queue!(stdout(), terminal::Clear(terminal::ClearType::All))?;
        Ok(())
    }

    pub fn move_cursor_to(&mut self, position: Position) -> Result<(), Error> {
        self.cursor.position = position;
        queue!(stdout(), MoveTo(position.col, position.row))
    }

    fn queue_command<T:Command>(command: T) -> Result<(), Error> {
        queue!(stdout(), command)
    }
}
