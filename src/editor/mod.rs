mod terminal;

use terminal::{Terminal, Position};

use std::io::Error;
use std::io::stdout;
use std::io::Write;

use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");


pub struct Editor {
    terminal: Terminal,
    should_quit: bool,
}

impl Editor {
    pub fn default() -> Self {
        Self { should_quit: false, terminal: Terminal::default() }
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

    fn refresh_screen(&mut self) -> Result<(), std::io::Error> {
        Terminal::hide_cursor()?;

        if self.should_quit {
            Terminal::clear_screen()?;
            self.terminal.move_cursor_to(Position { row: 0, col: 0 })?;
            Terminal::print("Goodbye!\r\n")?;
        } else {
            self.draw_rows()?;
            self.terminal.move_cursor_to(Position { row: 0, col: 0 })?;
        }

        Terminal::show_cursor()?;

        stdout().flush()?;

        Ok(())
    }

    fn draw_rows(&mut self) -> Result<(), std::io::Error> {
        self.terminal.move_cursor_to(Position { row:0, col: 0 })?;

        for row in 0..self.terminal.size.rows {
            self.terminal.move_cursor_to(Position { row, col: 0 })?;

            let mut line;
            if row == self.terminal.size.rows / 3 {
                let message = format!("{NAME} editor -- {VERSION}");
                let padding = (self.terminal.size.cols as usize - message.len()) / 2;
                let spaces = " ".repeat(padding - 1);
                line = format!("~{spaces}{message}");
                line.truncate(self.terminal.size.cols as usize);
            } else {
                line = "~".to_string();
            }

            Terminal::clear_line()?;
            Terminal::print(line)?;
        }

        Ok(())
    }
}
