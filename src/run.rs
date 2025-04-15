use crate::buffer::{TextBuffer, draw_ui};
use crate::cursors::Cursor;
use crate::file::{save_as, save_to_file};

use std::error::Error;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};

pub fn run(buffer: &mut TextBuffer) -> Result<(), Box<dyn std::error::Error>> {
    let mut cursors: Vec<Cursor> = vec![Cursor { x: 0, y: 0 }];
    let mut end = false;

    loop {
        draw_ui(buffer, &cursors)?;
        if end {
            return Ok(());
        }
        if event::poll(Duration::from_millis(500))? {
            if let Ok(Event::Key(key_event)) = event::read() {
                handle_key(key_event, buffer, &mut cursors, &mut end)?;
                if end {
                    break;
                }
            }
        }
    }

    fn handle_key(
        key_event: KeyEvent,
        buffer: &mut TextBuffer,
        cursors: &mut Vec<Cursor>,
        end: &mut bool,
    ) -> Result<(), Box<dyn Error>> {
        match key_event.code {
            KeyCode::Char('S')
                if key_event.modifiers.contains(KeyModifiers::CONTROL)
                    && key_event.modifiers.contains(KeyModifiers::SHIFT) =>
            {
                save_as(buffer)?;
                Ok(())
            }
            KeyCode::Char('s') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                save_to_file(buffer)?;
                Ok(())
            }
            KeyCode::Char('a') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                cursors.push(Cursor {
                    x: cursors[0].x,
                    y: cursors[0].y,
                });
                Ok(())
            }
            KeyCode::Char('x') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                cursors.truncate(1);
                Ok(())
            }

            KeyCode::Char(char) if char.is_ascii_graphic() || char == ' ' => {
                for cursor in cursors {
                    // Use mutable reference here
                    cursor.insert_char(buffer, char);
                }
                Ok(())
            }

            KeyCode::Left => {
                if let Some(cursor) = cursors.get_mut(0) {
                    cursor.move_left(buffer);
                }
                Ok(())
            }
            KeyCode::Right => {
                if let Some(cursor) = cursors.get_mut(0) {
                    cursor.move_right(buffer);
                }
                Ok(())
            }
            KeyCode::Up => {
                if let Some(cursor) = cursors.get_mut(0) {
                    cursor.move_up(buffer);
                }
                Ok(())
            }
            KeyCode::Down => {
                if let Some(cursor) = cursors.get_mut(0) {
                    cursor.move_down(buffer);
                }
                Ok(())
            }

            KeyCode::Enter => {
                // sort by line descending so later inserts don’t mess up earlier ones
                let mut sorted_cursors: Vec<_> = cursors.iter_mut().collect();
                sorted_cursors.sort_by(|a, b| b.y.cmp(&a.y));

                for cursor in sorted_cursors {
                    buffer.insert_newline(cursor);
                }

                Ok(())
            }

            KeyCode::Backspace => {
                for cursor in cursors {
                    cursor.backspace(buffer);
                }

                Ok(())
            }

            KeyCode::Esc => {
                *end = true;
                Ok(())
            }

            _ => Ok(()),
        }
    }

    Ok(())
}
