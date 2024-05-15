mod terminal;

use terminal::{Terminal, Position};

use std::io::Error;
use std::io::stdout;
use std::io::Write;

use crossterm::cursor::{Hide, Show};
use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use crossterm::queue;
use crossterm::terminal::{Clear, ClearType};
use crossterm::style::Print;



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

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        queue!(stdout(), Hide)?;

        if self.should_quit {
            // let size = Terminal::size()?;
            // Terminal::move_cursor_to(size.0, size.1)?;
            Terminal::clear_screen()?;
            Terminal::move_cursor_to(Position { row: 0, col: 0 })?;
            queue!(stdout(), Print("Goodbye!\r\n"))?;
        } else {
            self.draw_rows()?;
            Terminal::move_cursor_to(Position { row: 0, col: 0 })?;
        }

        queue!(stdout(), Show)?;

        stdout().flush()?;

        Ok(())
    }

    fn draw_rows(&self) -> Result<(), std::io::Error> {
        Terminal::move_cursor_to(Position { row:0, col: 0 })?;

        for row in 0..self.terminal.size.rows {
            Terminal::move_cursor_to(Position { row, col: 0 })?;
            queue!(stdout(), Clear(ClearType::CurrentLine))?;
            queue!(stdout(), Print("~"))?;

            if row == self.terminal.size.rows / 3 {
                let message = "Hecto - v0.1.0";
                Terminal::move_cursor_to(Position{
                    row,
                    col: (self.terminal.size.cols - (message.len() as u16)) / 2
                })?;
                queue!(stdout(), Print(message))?;
            }
        }

        Ok(())
    }
}
