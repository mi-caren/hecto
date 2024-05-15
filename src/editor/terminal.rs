use crossterm::queue;
use crossterm::cursor::{Hide, Show, MoveTo};
use crossterm::terminal;

use std::io::Error;
use std::io::stdout;
use std::io::Write;

pub struct Terminal {
    pub size: Size,
}

pub struct Size {
    pub rows: u16,
    pub cols: u16,
}

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
            }
        }
    }

    pub fn initialize() -> Result<(), Error> {
        terminal::enable_raw_mode()?;
        queue!(stdout(), Hide)?;
        Self::clear_screen()?;
        queue!(stdout(), Show)?;
        stdout().flush()?;
        Ok(())
    }

    pub fn terminate() -> Result<(), Error> {
        terminal::disable_raw_mode()
    }

    pub fn clear_screen() -> Result<(), Error> {
        // for _row in 0..Self::size()?.1 {
        //     print!("\r\n");
        //     // execute!(stdout(), terminal::Clear(terminal::ClearType::All))
        // }
        queue!(stdout(), terminal::Clear(terminal::ClearType::All))?;
        Ok(())
    }

    pub fn move_cursor_to(position: Position) -> Result<(), Error> {
        queue!(stdout(), MoveTo(position.col, position.row))
    }
}
