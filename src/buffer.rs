use std::{
    error::Error,
    io::{Write, stdout},
};

use crossterm::{
    cursor, execute,
    style::{Color, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};

use crate::{cursors::Cursor, ui::render_number};

pub fn draw_ui(buffer: &TextBuffer, cursors: &[Cursor]) -> Result<(), Box<dyn Error>> {
    let mut stdout = stdout();
    let buffer_content = buffer.render(cursors);

    let min_x = 13;

    execute!(
        stdout,
        cursor::Hide,
        cursor::MoveTo(min_x, 0),
        Clear(ClearType::All)
    )?;

    render_number(&mut stdout, &buffer_content, cursors[0].y)?;

    for (n, line) in buffer_content.iter().enumerate() {
        execute!(stdout, cursor::MoveTo(min_x, n as u16))?;
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
            let mut chars: Vec<String> = line.chars().map(|c| c.to_string()).collect();

            // Collect cursors on this line
            let mut cursors_on_line: Vec<_> = cursors
                .iter()
                .enumerate()
                .filter(|(_, c)| c.y == y)
                .collect();

            // Sort by x to insert from left to right
            cursors_on_line.sort_by_key(|(_, c)| c.x);

            for (offset, (i, cursor)) in cursors_on_line.into_iter().enumerate() {
                let insert_at = cursor.x + offset;
                let color = if i == 0 { Color::DarkGrey } else { Color::Blue };
                let cursor_str = format!("{}│{}", SetForegroundColor(color), ResetColor);

                if insert_at <= chars.len() {
                    chars.insert(insert_at, cursor_str);
                } else {
                    while chars.len() < insert_at {
                        chars.push(" ".to_string());
                    }
                    chars.push(cursor_str);
                }
            }

            rendered_lines.push(chars.concat());
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
