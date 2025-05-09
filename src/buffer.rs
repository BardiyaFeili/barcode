use crossterm::style::{Color, ResetColor, SetForegroundColor};

use crate::cursors::Cursor;

#[derive(Debug)]
pub struct TextBuffer {
    pub lines: Vec<String>,
    pub path: String,
    pub view_start: usize,
}

impl TextBuffer {
    pub fn new() -> Self {
        Self {
            lines: vec![String::new()],
            path: String::new(),
            view_start: 1,
        }
    }

    pub fn render(&self, cursors: &[Cursor]) -> Vec<String> {
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

    pub fn view_go_down(&mut self) {
        self.view_start += 1;
    }

    pub fn view_go_up(&mut self) {
        self.view_start -= 1;
    }
}
