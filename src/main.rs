use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::io::stdin;

use tui::backend::TermionBackend;
use tui::layout::Constraint;
use tui::Terminal;
use tui::widgets::{ Block, Row, Table, TableState};

use termion::raw::IntoRawMode;
use termion::input::MouseTerminal;
use termion::screen::AlternateScreen;
use termion::event::Key;
use termion::input::TermRead;

let stdout = io::stdout().into_raw_mode()?; // enters raw mode, gives full control of IO


fn main() {
    println!("Hello, world!");
}
