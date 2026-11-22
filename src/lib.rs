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
}