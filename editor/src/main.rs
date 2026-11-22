use std::fs;
use forge_text_engine::{Position, TextBuffer};
use crossterm::{execute, terminal};
use crossterm::cursor::MoveTo;
use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType};
use log::error;

fn main() {
    let mut buffer = TextBuffer::new();
    let mut cursor = buffer.position();
    let x = fs::read_to_string("editor/test.txt");

    terminal::enable_raw_mode().unwrap();
    loop {
        cursor = buffer.position();
        match crossterm::execute! (
            std::io::stdout(),
            Clear(ClearType::All),
            MoveTo(0,0),
            Print(buffer.to_text().replace("\n","\r\n")),
            MoveTo(cursor.column as u16,cursor.line as u16)) {
                Ok(()) => (),
                Err(error) => {
                eprintln!("Screen error | Err: {}", error);
                break
            }
        };

        let event = match crossterm::event::read() {
            Ok(event) => event,
            Err(error) => {
                eprintln!("Error reading the keyboard | Err: {}", error);
                break
            }
        };
        if let Event::Key(tecla) = event {
            if tecla.code == KeyCode::Esc {
                break
            }
            if tecla.code == KeyCode::Enter {
                buffer.insert_newline()
            }
            if tecla.code == KeyCode::Backspace {
                buffer.backspace()
            }
            if tecla.code == KeyCode::Down {
                buffer.move_down()
            }
            if tecla.code == KeyCode::Up {
                buffer.move_up()
            }
            if tecla.code == KeyCode::Left {
                buffer.move_left()
            }
            if tecla.code == KeyCode::Right {
                buffer.move_right()
            }
            if let KeyCode::Char(c) = tecla.code {
                buffer.insert_char(c)
            }
        };
    }

    terminal::disable_raw_mode().unwrap();

    fs::write("editor/output.txt", buffer.to_text()).unwrap();
    let text = buffer.to_text();
        println!("\r\n{}", text);
}