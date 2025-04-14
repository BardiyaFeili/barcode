use std::{
    error::Error,
    io::{Write, stdout},
};

use crossterm::{
    cursor, execute,
    style::{Color, ResetColor, SetForegroundColor},
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

    let buffer_content = buffer.render(cursors);

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

    fn render(&self, cursors: &[Cursor]) -> Vec<String> {
        let mut rendered_lines = vec![];

        for (y, line) in self.lines.iter().enumerate() {
            let mut line_with_cursors = line.clone();

            // Check if we have any cursors on this line
            for (index, cursor) in cursors.iter().enumerate() {
                if cursor.y == y {
                    let (before_cursor, after_cursor) = line.split_at(cursor.x);

                    let cursor_color = if index == 0 {
                        self.colored_cursor(Color::Grey)
                    } else {
                        self.colored_cursor(Color::Blue)
                    };

                    line_with_cursors =
                        format!("{}{}{}", before_cursor, cursor_color, after_cursor);
                }
            }

            rendered_lines.push(line_with_cursors);
        }

        rendered_lines
    }

    fn colored_cursor(&self, color: Color) -> String {
        // Return the cursor character with the specified color
        format!("{}|{}", SetForegroundColor(color), ResetColor)
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
