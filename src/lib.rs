#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

pub struct TextBuffer {
    lines: Vec<Vec<char>>,
    cursor: Position,
}

impl TextBuffer {
    pub fn new() -> Self {
        Self {
            lines: vec![Vec::new()],
            cursor: Position { line: 0, column: 0 },
        }
    }

    pub fn insert_char(&mut self, c: char) {
        let line = &mut self.lines[self.cursor.line];
        line.insert(self.cursor.column, c);
        self.cursor.column += 1;
    }

    /// Deletes the character at the cursor, joining the next line at end of line.
    pub fn delete_char(&mut self) {
        let line = self.cursor.line;
        if self.cursor.column < self.lines[line].len() {
            self.lines[line].remove(self.cursor.column);
        } else if line + 1 < self.lines.len() {
            let next_line = self.lines.remove(line + 1);
            self.lines[line].extend(next_line);
        }
    }

    pub fn position(&self) -> Position {
        self.cursor
    }

    /// Deletes the character before the cursor, joining with the previous line
    /// when the cursor is at the start of a line.
    pub fn backspace(&mut self) {
        if self.cursor.column > 0 {
            let line = &mut self.lines[self.cursor.line];
            line.remove(self.cursor.column - 1);
            self.cursor.column -= 1;
        } else if self.cursor.line > 0 {
            let current_line = self.lines.remove(self.cursor.line);
            self.cursor.line -= 1;
            self.cursor.column = self.lines[self.cursor.line].len();
            self.lines[self.cursor.line].extend(current_line);
        }
    }

    pub fn insert_newline(&mut self) {
        let remainder = self.lines[self.cursor.line].split_off(self.cursor.column);
        self.cursor.line += 1;
        self.cursor.column = 0;
        self.lines.insert(self.cursor.line, remainder);
    }

    pub fn to_text(&self) -> String {
        self.lines
            .iter()
            .map(|line| line.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn from_text(text: &str) -> Self {
        let lines = text
            .split('\n')
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        Self {
            lines,
            cursor: Position { line: 0, column: 0 },
        }
    }

    pub fn move_left(&mut self) {
        if self.cursor.column > 0 {
            self.cursor.column -= 1;
        } else if self.cursor.line > 0 {
            self.cursor.line -= 1;
            self.cursor.column = self.lines[self.cursor.line].len();
        }
    }

    pub fn move_right(&mut self) {
        if self.cursor.column < self.lines[self.cursor.line].len() {
            self.cursor.column += 1;
        } else if self.cursor.line + 1 < self.lines.len() {
            self.cursor.line += 1;
            self.cursor.column = 0;
        }
    }

    pub fn move_up(&mut self) {
        if self.cursor.line > 0 {
            self.cursor.line -= 1;
            self.cursor.column = self.cursor.column.min(self.lines[self.cursor.line].len());
        }
    }

    pub fn move_down(&mut self) {
        if self.cursor.line + 1 < self.lines.len() {
            self.cursor.line += 1;
            self.cursor.column = self.cursor.column.min(self.lines[self.cursor.line].len());
        }
    }

    /// Moves the cursor to a valid character boundary, clamping out-of-range positions.
    pub fn change_position(&mut self, line: usize, column: usize) {
        self.cursor.line = line.min(self.lines.len() - 1);
        self.cursor.column = column.min(self.lines[self.cursor.line].len());
    }

    /// Returns the first character position where query occurs, including across lines.
    pub fn find_first(&self, query: &str) -> Option<Position> {
        let needle = query.chars().collect::<Vec<_>>();
        if needle.is_empty() {
            return None;
        }

        let haystack = self.to_text().chars().collect::<Vec<_>>();
        let start = haystack
            .windows(needle.len())
            .position(|window| window == needle.as_slice())?;

        let mut line = 0;
        let mut column = 0;
        for character in haystack.iter().take(start) {
            if *character == '\n' {
                line += 1;
                column = 0;
            } else {
                column += 1;
            }
        }

        Some(Position { line, column })
    }
}

impl Default for TextBuffer {
    fn default() -> Self {
        Self::new()
    }
}