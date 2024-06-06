use crate::editor::utils::Size;

use crossterm::{queue, Command};
use crossterm::cursor::{Hide, Show, MoveTo};
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType};


use std::io::Error;
use std::io::stdout;
use std::io::Write;

pub struct Terminal {
    pub size: Size,
    // pub cursor: Cursor,
}

// #[derive(Default)]
// pub struct Cursor {
//     pub position: Position,
// }

#[derive(Copy, Clone, Default)]
pub struct CursorPosition {
    pub row: u16,
    pub col: u16,
}


impl Drop for Terminal {
    fn drop(&mut self) {
        let _ = Terminal::terminate();
    }
}

impl Terminal {
    pub fn new() -> Self {
        let terminal_size = terminal::size().unwrap();
        Self {
            size: Size {
                rows: terminal_size.1 as usize,
                cols: terminal_size.0 as usize,
            },
            // cursor: Cursor::default(),
        }
    }

    pub fn initialize() -> Result<(), Error> {
        terminal::enable_raw_mode()?;
        Self::queue_command(EnterAlternateScreen)?;
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

    pub fn print(str: &str) -> Result<(), Error> {
        Self::queue_command(Print(str))
    }

    pub fn log_error(str: &str, error: Error) {
        if let Err(_) = Self::queue_command(LeaveAlternateScreen) {
            return;
        }
        let _ = Self::print(&format!("{str}: {error}\n\r"));
        if let Err(inner_error) = Self::queue_command(EnterAlternateScreen) {
            eprint!("Error: {inner_error}; While logging error: {error}\n\r");
        }
        assert!(if let Ok(_) = stdout().flush() { true } else { false });
    }

    pub fn clear_line() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::CurrentLine))
    }

    pub fn terminate() -> Result<(), Error> {
        terminal::disable_raw_mode()?;
        Self::queue_command(LeaveAlternateScreen)?;
        stdout().flush()
    }

    pub fn clear_screen() -> Result<(), Error> {
        queue!(stdout(), terminal::Clear(terminal::ClearType::All))?;
        Ok(())
    }

    pub fn move_cursor_to(position: CursorPosition) -> Result<(), Error> {
        // self.cursor.position = position;
        queue!(stdout(), MoveTo(position.col, position.row))
    }

    fn queue_command<T:Command>(command: T) -> Result<(), Error> {
        queue!(stdout(), command)
    }
}
