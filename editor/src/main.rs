use std::{env, fs};
use forge_text_engine::{Position, TextBuffer};
use crossterm::{execute, terminal};
use crossterm::cursor::MoveTo;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType};
use log::error;

fn main() {
    let mut searching = false;
    let mut dirty_text = String::new();
    let mut dirty = false;
    let status = "";
    let mut query = String::new();

    let file = match env::args().nth(1) {
        Some(file) => (file),
        None => {eprintln!("No file route");
        return}
    };

    let x = fs::read_to_string(&file).unwrap();
    let mut buffer = TextBuffer::from_text(&x);
    terminal::enable_raw_mode().unwrap();
    loop {
        let mut cursor = buffer.position();
        let status = if searching {
            format!("\r\nSearch: {query} ")
        } else {
            format!("\r\nFile: {} | Ln {}, Col {}",file,buffer.position().line, buffer.position().column)
        };
        match crossterm::execute! (
            std::io::stdout(),
            Clear(ClearType::All),
            MoveTo(0,0),
            Print(buffer.to_text().replace("\n","\r\n")),
            Print(&dirty_text),
            Print(status),
            MoveTo(cursor.column as u16,cursor.line as u16)) {
                Ok(()) => (),
                Err(error) => {
                eprintln!("Screen error | Err: {}", error);
                    break
            }
        };

        dirty_text = "".to_string();

        let event = match crossterm::event::read() {
            Ok(event) => event,
            Err(error) => {
                eprintln!("Error reading the keyboard | Err: {}", error);
                break
            }
        };
        if let Event::Key(tecla) = event {
            if tecla.code == KeyCode::Esc {
                if dirty == true {
                    dirty_text = ("Changes without save | Press ctrl + s to save".to_string());
                    continue;
                }
                break
            }
            if tecla.code == KeyCode::Enter {
                dirty = true;
                buffer.insert_newline()
            }
            if tecla.code == KeyCode::Backspace {
                dirty = true;
                buffer.backspace()
            }
            //The Delete function doesn't work, will be reworked later
            if tecla.code == KeyCode::Delete {
                buffer.delete_char()
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
            if tecla.code == KeyCode::Home {
                buffer.change_position(0,0)
            }
            if tecla.modifiers == KeyModifiers::CONTROL {
                if tecla.code == KeyCode::Char('s') {
                    match fs::write(&file, buffer.to_text()) {
                        Ok(()) => dirty = false,
                        Err(error) => dirty_text = format!("Error saving | Err: {}", error)
                    };
                    continue;
                }
                if tecla.code == KeyCode::Char('f') {
                    query.clear();
                    searching = true;
                    if searching {
                        match tecla.code {
                            KeyCode::Esc => searching = false,
                            KeyCode::Enter => {
                                match buffer.find_first(&query) {
                                    Some(pos) => {buffer.change_position(pos.line,pos.column)},
                                    None => dirty_text = "No matches".to_string()
                                }
                            searching = false;
                            }
                        KeyCode::Backspace => {
                            query.pop();
                        }
                        KeyCode::Char(c) if !tecla.modifiers.contains(KeyModifiers::CONTROL) => {
                            query.push(c);
                        }
                        _ => {}
                    }
                    continue;
                }
            }
            if let KeyCode::Char(c) = tecla.code {
                buffer.insert_char(c);
                dirty = true;
            }
        };
    }}
    terminal::disable_raw_mode().unwrap();
    eprintln!("\r\n")
}