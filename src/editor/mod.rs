mod terminal;

use terminal::Terminal;

use std::io::Error;

use crossterm::event::{Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};



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

            let event = Terminal::read()?;
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
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::move_cursor_to(0, 0)?;
            print!("Goodbye!\r\n");
        } else {
            Self::draw_rows()?;
            Terminal::move_cursor_to(0, 0)?;
        }

        Ok(())
    }

    fn draw_rows() -> Result<(), std::io::Error> {
        Terminal::move_cursor_to(0, 0)?;
        let rows = Terminal::size()?.1;

        for row in 0..rows {
            Terminal::move_cursor_to(0, row)?;
            print!("~");
        }

        Ok(())
    }
}
