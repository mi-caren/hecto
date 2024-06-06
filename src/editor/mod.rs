mod terminal;

use terminal::{Terminal, Position};

use std::io::Error;
use std::io::stdout;
use std::io::Write;
use std::cmp::min;

use crossterm::event::{read, Event, Event::Key, Event::Resize, KeyCode, KeyEvent, KeyModifiers};

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
    needs_redraw: bool,
}

#[derive(Default)]
struct Buffer {
    lines: Vec<String>,
}


impl Editor {
    pub fn run(&mut self) {
        if let Err(error) = Terminal::initialize() {
            println!("Unable no initialize terminal {error}");
            return;
        }

        let args: Vec<String> = std::env::args().collect();
        if let Some(filename) = args.get(1) {
            if let Err(error) = self.view.load(filename) {
                println!("Unable to load file {filename}: {error}\r");
                // maybe I can just open an empty buffer
                return;
            }
        }

        self.view.needs_redraw = true;
        self.repl();
    }

    fn repl(&mut self) {
        loop {
            if let Err(error) = self.refresh_screen() {
                println!("An error occurred while refreshing the screen: {error}");
                break;
            }

            if self.should_quit {
                break;
            }

            match read() {
                Ok(event) => self.evaluate_event(&event),
                Err(error) => {
                    println!("Unable to read terminal events: {error}");
                    break;
                }
            }
        }
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
                | KeyCode::End => {
                    self.move_point(*code);
                },
                _ => (),
            }
        } else if let Resize(cols, rows) = event {
            self.terminal.size.cols = *cols;
            self.terminal.size.rows = *rows;
            self.view.needs_redraw = true;
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
        if let Err(error) = Terminal::hide_cursor() {
            println!("Unable to hide cursor while refreshing screen: {error}");
        }

        if self.should_quit {
            if let Err(error) = self.print_goodbye() {
                println!("Error while quitting: {error}");
            };
        } else {
            if self.view.needs_redraw {
                self.view.render(&mut self.terminal)?;
            }
            self.terminal.move_cursor_to(Position {
                row: self.location.row as u16,
                col: self.location.col as u16,
            })?;
        }

        Terminal::show_cursor()?;

        stdout().flush()?;

        Ok(())
    }

    fn print_goodbye(&mut self) -> Result<(), Error> {
        Terminal::clear_screen()?;
        self.terminal.move_cursor_to(Position { row: 0, col: 0 })?;
        Terminal::print("Goodbye!\r\n")
    }
}

impl View {
    pub fn render(&mut self, terminal: &mut Terminal) -> Result<(), Error> {
        terminal.move_cursor_to(Position { row:0, col: 0 })?;

        for row in 0..terminal.size.rows {
            let mut line =
                if let Some(line) = self.buffer.lines.get(row as usize) {
                    line.clone()
                } else if self.buffer.is_empty() && row == terminal.size.rows / 3 {
                    let message = format!("{NAME} editor -- {VERSION}");
                    let padding = (terminal.size.cols as usize - message.len()) / 2;
                    let spaces = " ".repeat(padding - 1);
                    format!("~{spaces}{message}")
                } else {
                    "~".to_string()
                };
            debug_assert!(!line.contains("\n"));
            debug_assert!(!line.contains("\r"));

            line.truncate(terminal.size.cols as usize);
            Terminal::clear_line()?;
            Terminal::print(&line)?;
            if row.saturating_add(1) < terminal.size.rows {
                Terminal::print("\r\n")?;
            }
        }

        self.needs_redraw = false;
        Ok(())
    }

    pub fn load(&mut self, filename: &String) -> Result<(), Error> {
        let file_contents = std::fs::read_to_string(filename)?;

        for line in file_contents.lines() {
            self.buffer.lines.push(line.to_string());
        }

        self.needs_redraw = true;
        Ok(())
    }
}

impl Buffer {
    fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }
}
