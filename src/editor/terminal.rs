use crossterm::queue;
use crossterm::cursor::{Hide, Show, MoveTo};
use crossterm::terminal;

use std::io::Error;
use std::io::stdout;
use std::io::Write;

pub struct Terminal {}

impl Terminal {
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

    pub fn size() -> Result<(u16, u16), Error> {
        terminal::size()
    }

    pub fn move_cursor_to(col: u16, row: u16) -> Result<(), Error> {
        queue!(stdout(), MoveTo(col, row))
    }
}
