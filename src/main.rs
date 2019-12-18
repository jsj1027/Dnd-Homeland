use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen},
};
use std::{
    io::{stdout, Error, Write},
    sync::mpsc,
    thread,
    time::Duration,
};
use tui::{backend::CrosstermBackend, Terminal};

fn main() -> () {
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen);

    let backend = CrosstermBackend::new(stdout);
    
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.hide_cursor().unwrap();
}
