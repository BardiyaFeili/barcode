use std::{
    error::Error,
    io::{Write, stdout},
};

use crossterm::{
    cursor, execute,
    terminal::{Clear, ClearType},
};

use crate::cursors::Cursor;

pub fn draw_ui(buffer: &TextBuffer, cursors: &[Cursor]) -> Result<(), Box<dyn Error>> {
    let mut stdout = stdout();

    execute!(
        stdout,
        cursor::Hide,
        cursor::MoveTo(0, 0),
        Clear(ClearType::All)
    )?;

    let buffer_content = buffer.render(&cursors[0]);

    for (n, line) in buffer_content.iter().enumerate() {
        execute!(stdout, cursor::MoveTo(0, n as u16))?;
        writeln!(stdout, "{}", line)?;
    }

    stdout.flush()?;
    Ok(())
}

#[derive(Debug)]
pub struct TextBuffer {
    pub lines: Vec<String>,
    pub path: String,
}

impl TextBuffer {
    pub fn new() -> Self {
        Self {
            lines: vec![String::new()],
            path: String::new(),
        }
    }

    fn render(&self, cursor: &Cursor) -> Vec<String> {
        let mut rendered_lines = vec![];

        for (y, line) in self.lines.iter().enumerate() {
            if y == cursor.y {
                let mut line_with_cursor = line.clone();
                if cursor.x <= line.len() {
                    line_with_cursor.insert(cursor.x, '|');
                } else {
                    // Cursor is past the end, pad with spaces
                    line_with_cursor.push_str(&" ".repeat(cursor.x - line.len()));
                    line_with_cursor.push('|');
                }
                rendered_lines.push(line_with_cursor);
            } else {
                rendered_lines.push(line.clone());
            }
        }

        rendered_lines
    }

    pub fn insert_newline(&mut self, cursor: &mut Cursor) {
        if cursor.y >= self.lines.len() {
            self.lines.push(String::new());
            cursor.x = 0;
            cursor.y += 1;
            return;
        }

        let current_line = &mut self.lines[cursor.y];
        let new_line = current_line.split_off(cursor.x);
        self.lines.insert(cursor.y + 1, new_line);
        cursor.x = 0;
        cursor.y += 1;
    }
}
