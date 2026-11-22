use std::fs;
use forge_text_engine::TextBuffer;
use crossterm::{terminal, };
use crossterm::event::KeyEvent;

fn main() {

    let mut buffer = TextBuffer::new();
    let x = fs::read_to_string("editor/test.txt");
    crossterm::terminal::enable_raw_mode();
    let event = crossterm::event::read();
    terminal::disable_raw_mode().unwrap();
    let event = event.unwrap();


    fs::write("editor/output.txt", buffer.to_text()).unwrap();
    let text = buffer.to_text();
        println!("{:?}", event);
}