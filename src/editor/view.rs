use crate::editor::utils::Size;
use crate::editor::terminal::{Terminal, CursorPosition};
use crate::editor::{NAME, VERSION};

use crossterm::event::KeyCode;
use std::io::Error;
use std::cmp::min;



pub struct View {
    pub location: Location,
    size: Size,
    buffer: Buffer,
    pub needs_redraw: bool,
}

#[derive(Default)]
struct Buffer {
    lines: Vec<String>,
}

#[derive(Default)]
pub struct Location {
    pub row: usize,
    pub col: usize,
}


impl View {
    pub fn new(size: Size) -> Self {
        Self {
            location: Location::default(),
            size,
            buffer: Buffer::default(),
            needs_redraw: true,
        }
    }

    pub fn render(&mut self) -> Result<(), Error> {
        Terminal::move_cursor_to(CursorPosition { row:0, col: 0 })?;

        for row in 0..self.size.rows {
            let mut line =
                if let Some(line) = self.buffer.lines.get(row as usize) {
                    line.clone()
                } else if self.buffer.is_empty() && row == self.size.rows / 3 {
                    let message = format!("{NAME} editor -- {VERSION}");
                    let padding = (self.size.cols as usize - message.len()) / 2;
                    let spaces = " ".repeat(padding - 1);
                    format!("~{spaces}{message}")
                } else {
                    "~".to_string()
                };
            debug_assert!(!line.contains("\n"));
            debug_assert!(!line.contains("\r"));

            line.truncate(self.size.cols as usize);
            Terminal::clear_line()?;
            Terminal::print(&line)?;
            if row.saturating_add(1) < self.size.rows {
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

    pub fn move_point(&mut self, key: KeyCode) {
        match key {
            KeyCode::Right => {
                self.location.col = min(self.location.col.saturating_add(1), self.size.cols as usize - 1);
            },
            KeyCode::Left => {
                self.location.col = self.location.col.saturating_sub(1);
            },
            KeyCode::Up => {
                self.location.row = self.location.row.saturating_sub(1);
            },
            KeyCode::Down => {
                self.location.row = min(self.location.row.saturating_add(1), self.size.rows as usize - 1);
            },
            KeyCode::PageUp => {
                self.location.row = 0;
            },
            KeyCode::PageDown => {
                self.location.row = self.size.rows as usize - 1;
            },
            KeyCode::Home => {
                self.location.col = 0;
            },
            KeyCode::End => {
                self.location.col = self.size.cols as usize - 1;
            },
            _ => (),
        }
    }
}

impl Buffer {
    fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }
}
