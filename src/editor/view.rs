use crate::editor::utils::Size;
use crate::editor::terminal::{Terminal, CursorPosition};
use crate::editor::{NAME, VERSION};

use crossterm::event::KeyCode;
use std::io::Error;



pub struct View {
    pub location: Location,
    pub size: Size,
    buffer: Buffer,
    pub needs_redraw: bool,
    pub scroll_offset: ScrollOffset,
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

#[derive(Default)]
pub struct ScrollOffset {
    pub rows: usize,
    pub cols: usize,
}


impl View {
    pub fn new(size: Size) -> Self {
        Self {
            location: Location::default(),
            size,
            buffer: Buffer::default(),
            needs_redraw: true,
            scroll_offset: ScrollOffset::default(),
        }
    }

    pub fn render(&mut self) -> Result<(), Error> {
        Terminal::move_cursor_to(CursorPosition { row:0, col: 0 })?;

        let start_row = self.scroll_offset.rows;
        let end_row = self.scroll_offset.rows + self.size.rows;
        let start_col = self.scroll_offset.cols;

        for row in start_row..end_row {
            let mut line =
                if let Some(line) = self.buffer.lines.get(row as usize) {
                    if let Some(line_slice) = line.get(start_col..) {
                        line_slice.to_string()
                    } else {
                        "".to_string()
                    }
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
            if row.saturating_add(1) < end_row {
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
                self.location.col = self.location.col.saturating_add(1);
            },
            KeyCode::Left => {
                self.location.col = self.location.col.saturating_sub(1);
            },
            KeyCode::Up => {
                self.location.row = self.location.row.saturating_sub(1);
            },
            KeyCode::Down => {
                self.location.row = self.location.row.saturating_add(1);
            },
            KeyCode::PageUp => {
                self.location.row = 0;
            },
            KeyCode::PageDown => {
                self.location.row = self.scroll_offset.rows + self.size.rows as usize;
            },
            KeyCode::Home => {
                self.location.col = 0;
            },
            KeyCode::End => {
                self.location.col = self.scroll_offset.cols + self.size.cols as usize;
            },
            _ => (),
        }

        self.handle_scroll();
    }

    pub fn handle_scroll(&mut self) {
        if self.location.col >= self.scroll_offset.cols + self.size.cols || self.location.col < self.scroll_offset.cols {
            self.scroll_offset.cols = self.location.col.saturating_sub(self.size.cols);
            self.needs_redraw = true;
        }
        if self.location.row >= self.scroll_offset.rows + self.size.rows || self.location.row < self.scroll_offset.rows {
            self.scroll_offset.rows = self.location.row.saturating_sub(self.size.rows);
            self.needs_redraw = true;
        }
    }
}

impl Buffer {
    fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }
}
