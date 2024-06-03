mod terminal;

use terminal::{Terminal, Position};

use std::io::Error;
use std::io::stdout;
use std::io::Write;
use std::cmp::min;

use crossterm::event::{read, Event, Event::Key, KeyCode, KeyEvent, KeyModifiers};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");


#[derive(Default)]
pub struct Editor {
    terminal: Terminal,
    location: Location,
    view: View,
    should_quit: bool,
}

#[derive(Default)]
pub struct Location {
    pub row: usize,
    pub col: usize,
}

#[derive(Default)]
pub struct View {
    buffer: Buffer,
}

#[derive(Default)]
struct Buffer {
    lines: Vec<String>,
}


impl Editor {
    pub fn run(&mut self) {
        Terminal::initialize().unwrap();

        let args: Vec<String> = std::env::args().collect();
        if let Some(filename) = args.get(1) {
            self.view.load(filename).unwrap();
        }

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
            self.view.render(&mut self.terminal)?;
            self.terminal.move_cursor_to(Position {
                row: self.location.row as u16,
                col: self.location.col as u16,
            })?;
        }

        Terminal::show_cursor()?;

        stdout().flush()?;

        Ok(())
    }
}

impl View {
    pub fn render(&self, terminal: &mut Terminal) -> Result<(), Error> {
        terminal.move_cursor_to(Position { row:0, col: 0 })?;

        for row in 0..terminal.size.rows {
            let line =
                if let Some(line) = self.buffer.lines.get(row as usize) {
                    line.clone()
                } else if self.buffer.is_empty() && row == terminal.size.rows / 3 {
                    let mut message = format!("{NAME} editor -- {VERSION}");
                    let padding = (terminal.size.cols as usize - message.len()) / 2;
                    let spaces = " ".repeat(padding - 1);
                    message = format!("~{spaces}{message}");
                    message.truncate(terminal.size.cols as usize);
                    message
                } else {
                    "~".to_string()
                };

            Terminal::clear_line()?;
            Terminal::print(&line)?;
            if row.saturating_add(1) < terminal.size.rows {
                Terminal::print("\r\n")?;
            }
        }

        Ok(())
    }

    pub fn load(&mut self, filename: &String) -> Result<(), Error> {
        let file_contents = std::fs::read_to_string(filename)?;

        for line in file_contents.lines() {
            self.buffer.lines.push(line.to_string());
        }

        Ok(())
    }
}

impl Buffer {
    fn is_empty(&self) -> bool {
        self.lines.len() == 0
    }
}
