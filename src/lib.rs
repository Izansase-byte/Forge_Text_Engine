pub struct Position {pub line: usize, pub column: usize}
pub struct TextBuffer {lines: Vec<Vec<char>>, cursor: Position}

impl TextBuffer {
    pub fn new() -> Self {
        Self{lines: vec!(Vec::new()), cursor: Position{ line: 0, column: 0 }}
    }
    pub fn insert_char(&mut self, c: char) {
        let line = &mut self.lines[self.cursor.line];
        line.insert(self.cursor.column, c);
        self.cursor.column += 1;
    }

    fn delete_char(&mut self) {
        let line = &mut self.lines[self.cursor.line];
        line.remove(self.cursor.column);
    }

    pub fn backspace(&mut self) {
        let line = &mut self.lines[self.cursor.line];
        if self.cursor.column > 0 {
            line.remove(self.cursor.column - 1);
            self.cursor.column -= 1;
        }
        if self.cursor.column == 0 {
            if self.cursor.line > 0 {
                self.delete_char();
            }
        }

    }

    pub fn insert_newline(&mut self) {
        let line = &mut self.lines[self.cursor.line];
        let cut = Vec::split_off(line, self.cursor.column );
        self.lines.insert(self.cursor.line + 1, cut);
        self.cursor.line += 1;
        self.cursor.column = 0;
    }

    pub fn to_text(&self) -> String {
        self.lines.iter().map(|line| line.iter().collect::<String>()).collect::<Vec<String>>().join("\n")
    }
}