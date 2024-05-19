mod terminal;

use terminal::{Terminal, Position};

use std::io::Error;
use std::io::stdout;
use std::io::Write;
use std::cmp::min;

use crossterm::event::{read, Event, Event::Key, KeyCode, KeyEvent, KeyModifiers};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");


pub struct Editor {
    terminal: Terminal,
    location: Location,
    should_quit: bool,
}

pub struct Location {
    pub row: usize,
    pub col: usize,
}

impl Editor {
    pub fn default() -> Self {
        Self {
            should_quit: false,
            terminal: Terminal::default(),
            location: Location {
                row: 0,
                col: 0,
            }
        }
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
                KeyCode::Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                },
                KeyCode::Right
                | KeyCode::Left
                | KeyCode::Up
                | KeyCode::Down
                | KeyCode::PageUp
                | KeyCode::PageDown
                | KeyCode::Home
                |KeyCode::End => {
                    self.move_point(*code);
                },
                _ => (),
            }
        }
    }

    fn move_point(&mut self, key: KeyCode) {
        match key {
            KeyCode::Right => {
                self.location.col = min(self.location.col.saturating_add(1), self.terminal.size.cols as usize - 1);
            },
            KeyCode::Left => {
                self.location.col = self.location.col.saturating_sub(1);
            },
            KeyCode::Up => {
                self.location.row = self.location.row.saturating_sub(1);
            },
            KeyCode::Down => {
                self.location.row = min(self.location.row.saturating_add(1), self.terminal.size.rows as usize - 1);
            },
            KeyCode::PageUp => {
                self.location.row = 0;
            },
            KeyCode::PageDown => {
                self.location.row = self.terminal.size.rows as usize - 1;
            },
            KeyCode::Home => {
                self.location.col = 0;
            },
            KeyCode::End => {
                self.location.col = self.terminal.size.cols as usize - 1;
            },
            _ => (),
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
            self.terminal.move_cursor_to(Position {
                row: self.location.row as u16,
                col: self.location.col as u16,
            })?;
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
