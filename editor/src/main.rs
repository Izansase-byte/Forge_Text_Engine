use std::{env, fs, io};
use forge_text_engine::TextBuffer;
use crossterm::cursor::MoveTo;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use crossterm::style::Print;
use crossterm::terminal::{self, Clear, ClearType};

fn main() {
    let file = match env::args().nth(1) {
        Some(file) => file,
        None => {
            eprintln!("No file path provided");
            return;
        }
    };

    let contents = match fs::read_to_string(&file) {
        Ok(contents) => contents,
        Err(error) if error.kind() == io::ErrorKind::NotFound => String::new(),
        Err(error) => {
            eprintln!("Could not open {file}: {error}");
            return;
        }
    };

    let mut buffer = TextBuffer::from_text(&contents);
    let mut searching = false;
    let mut query = String::new();
    let mut dirty = false;
    let mut message = String::new();

    if let Err(error) = terminal::enable_raw_mode() {
        eprintln!("Could not enable terminal raw mode: {error}");
        return;
    }

    'editor: loop {
        let text = buffer.to_text();
        let status = if searching {
            format!("Search: {query}  (Enter: jump, Esc: cancel)")
        } else {
            let position = buffer.position();
            format!(
                "File: {file} | Ln {}, Col {} | Ctrl+F: search | Ctrl+S: save | Esc: quit",
                position.line + 1,
                position.column + 1
            )
        };

        let mut stdout = io::stdout();
        let draw_result = crossterm::execute!(
            stdout,
            Clear(ClearType::All),
            MoveTo(0, 0),
            Print(text.replace('\n', "\r\n")),
            Print("\r\n"),
            Print(&status),
            Print("\r\n"),
            Print(&message)
        );

        if let Err(error) = draw_result {
            message = format!("Screen error: {error}");
            break;
        }

        message.clear();
        let cursor = if searching {
            let row = text.chars().filter(|character| *character == '\n').count() + 1;
            (8 + query.chars().count(), row)
        } else {
            let position = buffer.position();
            (position.column, position.line)
        };

        if let Err(error) = crossterm::execute!(
            io::stdout(),
            MoveTo(cursor.0.min(u16::MAX as usize) as u16, cursor.1.min(u16::MAX as usize) as u16)
        ) {
            message = format!("Cursor error: {error}");
            break;
        }

        let event = match event::read() {
            Ok(event) => event,
            Err(error) => {
                message = format!("Keyboard error: {error}");
                break;
            }
        };
        let Event::Key(key) = event else {
            continue;
        };

        // Search mode consumes its own keys so Enter, Backspace, and typed
        // characters cannot accidentally edit the document.
        if searching {
            match key.code {
                KeyCode::Esc => {
                    searching = false;
                    query.clear();
                }
                KeyCode::Enter => {
                    match buffer.find_first(&query) {
                        Some(position) => {
                            buffer.change_position(position.line, position.column);
                        }
                        None => message = format!("No matches for \"{query}\""),
                    }
                    searching = false;
                }
                KeyCode::Backspace => {
                    query.pop();
                }
                KeyCode::Char(character)
                    if !key.modifiers.contains(KeyModifiers::CONTROL)
                        && !key.modifiers.contains(KeyModifiers::ALT) =>
                {
                    query.push(character);
                }
                _ => {}
            }
            continue;
        }

        if key.modifiers.contains(KeyModifiers::CONTROL) {
            match key.code {
                KeyCode::Char('f') | KeyCode::Char('F') => {
                    query.clear();
                    searching = true;
                    continue;
                }
                KeyCode::Char('s') | KeyCode::Char('S') => {
                    match fs::write(&file, buffer.to_text()) {
                        Ok(()) => {
                            dirty = false;
                            message = "Saved".to_string();
                        }
                        Err(error) => message = format!("Save failed: {error}"),
                    }
                    continue;
                }
                _ => continue,
            }
        }

        match key.code {
            KeyCode::Esc => {
                if dirty {
                    message = "Unsaved changes. Press Ctrl+S to save, then Esc to quit.".to_string();
                } else {
                    break 'editor;
                }
            }
            KeyCode::Enter => {
                buffer.insert_newline();
                dirty = true;
            }
            KeyCode::Backspace => {
                buffer.backspace();
                dirty = true;
            }
            KeyCode::Delete => {
                buffer.delete_char();
                dirty = true;
            }
            KeyCode::Left => buffer.move_left(),
            KeyCode::Right => buffer.move_right(),
            KeyCode::Up => buffer.move_up(),
            KeyCode::Down => buffer.move_down(),
            KeyCode::Home => {
                let position = buffer.position();
                buffer.change_position(position.line, 0);
            }
            KeyCode::End => {
                let position = buffer.position();
                buffer.change_position(position.line, usize::MAX);
            }
            KeyCode::Char(character)
                if !key.modifiers.contains(KeyModifiers::ALT)
                    && !key.modifiers.contains(KeyModifiers::SUPER) =>
            {
                buffer.insert_char(character);
                dirty = true;
            }
            _ => {}
        }
    }

    if let Err(error) = terminal::disable_raw_mode() {
        eprintln!("Could not restore terminal mode: {error}");
    }
    if !message.is_empty() {
        eprintln!("{message}");
    }
}