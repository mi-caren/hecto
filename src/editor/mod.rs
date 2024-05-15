mod terminal;

use terminal::Terminal;

use std::io::Error;
use std::io::stdout;
use std::io::Write;

use crossterm::cursor::{Hide, Show};
use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use crossterm::queue;
use crossterm::terminal::{Clear, ClearType};
use crossterm::style::Print;



pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub const fn default() -> Self {
        Self { should_quit: false }
    }

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;

            if self.should_quit {
                break;
            }

            let event = read()?;
            self.evaluate_event(&event);
        }

        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                },
                _ => (),
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        queue!(stdout(), Hide)?;

        if self.should_quit {
            let size = Terminal::size()?;
            Terminal::move_cursor_to(size.0, size.1)?;
            Terminal::clear_screen()?;
            Terminal::move_cursor_to(0, 0)?;
            queue!(stdout(), Print("Goodbye!\r\n"))?;
        } else {
            Self::draw_rows()?;
            Terminal::move_cursor_to(0, 0)?;
        }

        queue!(stdout(), Show)?;

        stdout().flush()?;

        Ok(())
    }

    fn draw_rows() -> Result<(), std::io::Error> {
        Terminal::move_cursor_to(0, 0)?;
        let rows = Terminal::size()?.1;

        for row in 0..rows {
            Terminal::move_cursor_to(0, row)?;
            queue!(stdout(), Clear(ClearType::CurrentLine))?;
            queue!(stdout(), Print("~"))?;
        }

        Ok(())
    }
}
