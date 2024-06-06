mod terminal;
mod utils;
mod view;

use terminal::{Terminal, CursorPosition};
use view::View;

use std::io::Error;
use std::io::stdout;
use std::io::Write;

use crossterm::event::{read, Event, Event::Key, Event::Resize, KeyCode, KeyEvent, KeyModifiers};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");


pub struct Editor {
    terminal: Terminal,
    view: View,
    should_quit: bool,
}


impl Editor {
    pub fn new() -> Self {
        //Retrieve the current hook, which by default does some nice printing of the panic
        let current_hook = std::panic::take_hook();
        // Define a new closure that takes a reference to the PanicInfo.
        // Move any external variables needed within the closure here. 
        // Place the closure into a Box and set it as the new panic hook.
        std::panic::set_hook(Box::new(move |panic_info| {
            let _ = Terminal::terminate();
            // Our custom panic hook logic goes here
            // Execute the original hook to retain default panic output behavior<
            current_hook(panic_info);
        }));

        let terminal = Terminal::new();
        let terminal_size = terminal.size;
        Self {
            terminal,
            view: View::new(terminal_size),
            should_quit: false,
        }
    }

    pub fn run(&mut self) {
        if let Err(error) = Terminal::initialize() {
            Terminal::log_error("Unable to initialize terminal", error);
            return;
        }

        let args: Vec<String> = std::env::args().collect();
        if let Some(filename) = args.get(1) {
            if let Err(error) = self.view.load(filename) {
                Terminal::log_error(&format!("Unable to load file {filename}"), error);
                // maybe I can just open an empty buffer
                return;
            }
        }

        self.repl();
    }

    fn repl(&mut self) {
        loop {
            if let Err(error) = self.refresh_screen() {
                Terminal::log_error("An error occurred while refreshing the screen", error);
                break;
            }

            if self.should_quit {
                break;
            }

            match read() {
                Ok(event) => self.evaluate_event(&event),
                Err(error) => {
                    Terminal::log_error("Unable to read terminal events", error);
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
                    self.view.move_point(*code);
                },
                _ => (),
            }
        } else if let Resize(cols, rows) = event {
            self.terminal.size.cols = *cols;
            self.terminal.size.rows = *rows;
            self.view.needs_redraw = true;
        }
    }

    fn refresh_screen(&mut self) -> Result<(), std::io::Error> {
        if let Err(error) = Terminal::hide_cursor() {
            Terminal::log_error("Unable to hide cursor while refreshing screen", error);
        }

        if self.should_quit {
            if let Err(error) = self.print_goodbye() {
                Terminal::log_error("Error while quitting", error);
            };
        } else {
            if self.view.needs_redraw {
                self.view.render()?;
            }
            Terminal::move_cursor_to(CursorPosition {
                row: self.view.location.row as u16,
                col: self.view.location.col as u16,
            })?;
        }

        Terminal::show_cursor()?;

        stdout().flush()?;

        Ok(())
    }

    fn print_goodbye(&mut self) -> Result<(), Error> {
        Terminal::clear_screen()?;
        Terminal::move_cursor_to(CursorPosition { row: 0, col: 0 })?;
        Terminal::print("Goodbye!\r\n")
    }
}

