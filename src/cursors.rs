use crate::buffer::TextBuffer;

#[derive(Debug, Clone)]
pub struct Cursor {
    pub x: usize,
    pub y: usize,
}

impl Cursor {
    pub fn insert_char(&mut self, buffer: &mut TextBuffer, char: char) {
        if buffer.lines.len() <= self.y {
            buffer.lines.push(String::new());
        }

        buffer.lines[self.y].insert(self.x, char);

        self.move_right(buffer);
    }

    pub fn backspace(&mut self, buffer: &mut TextBuffer) {
        if self.x > 0 {
            self.x -= 1;
            buffer.lines[self.y].remove(self.x);
        } else if self.y > 0 {
            let prev_line_len = buffer.lines[self.y - 1].len();
            let current_line = buffer.lines.remove(self.y);
            let prev_line = &mut buffer.lines[self.y - 1];

            prev_line.push_str(&current_line);

            self.x = prev_line_len;
            self.y -= 1;
        }
    }

    pub fn move_left(&mut self, buffer: &TextBuffer) {
        if self.x > 0 {
            self.x -= 1;
        } else if self.y > 0 {
            self.y -= 1;
            self.x = buffer.lines[self.y].len();
        }
    }

    pub fn move_right(&mut self, buffer: &TextBuffer) {
        if self.x < buffer.lines[self.y].len() {
            self.x += 1;
        } else if self.y + 1 < buffer.lines.len() {
            self.y += 1;
            self.x = 0;
        }
    }

    pub fn move_up(&mut self, buffer: &TextBuffer) {
        if self.y == 0 {
            self.x = 0;
        } else {
            self.y -= 1;
            let line_len = buffer.lines[self.y].len();
            self.x = self.x.min(line_len); // clamp x
        }
    }

    pub fn move_down(&mut self, buffer: &TextBuffer) {
        if self.y + 1 >= buffer.lines.len() {
            self.y = buffer.lines.len() - 1;
            self.x = buffer.lines[self.y].len();
        } else {
            self.y += 1;
            let line_len = buffer.lines[self.y].len();
            self.x = self.x.min(line_len);
        }
    }
}
