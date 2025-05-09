use std::{
    error::Error,
    io::{Stdout, Write, stdout},
    thread::sleep,
    time::{Duration, Instant},
};

use crossterm::{
    ExecutableCommand, cursor,
    event::{self, Event, KeyCode},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType, size},
};

pub enum MessageType {
    Success,
    Error,
}

impl MessageType {
    fn title(&self) -> String {
        match self {
            Self::Success => "Success".to_string(),
            Self::Error => "Error".to_string(),
        }
    }
    fn color(&self) -> Color {
        match self {
            Self::Success => Color::Cyan,
            Self::Error => Color::Red,
        }
    }
}

pub fn send_message(
    message: String,
    seconds: u64,
    msg_type: MessageType,
) -> Result<(), Box<dyn Error>> {
    let mut stdout = stdout();
    stdout.execute(SetForegroundColor(msg_type.color()))?;

    let duration = Duration::from_secs(seconds);
    let start = Instant::now();

    let cols = size().unwrap().0;

    let width = message.len() + 6;
    let x: u16 = cols - width as u16;
    let y: u16 = 2;

    while Instant::now() - start < duration {
        draw_box(
            width,
            x,
            y,
            &message,
            &msg_type.title().to_string(),
            &mut stdout,
        )?;
    }
    stdout.execute(ResetColor)?;
    Ok(())
}

pub fn user_input(title: String) -> Result<String, Box<dyn Error>> {
    let mut stdout = stdout();
    stdout.execute(SetForegroundColor(Color::Cyan))?;
    let mut input = String::new();

    let (cols, rows) = size().unwrap();

    // Calculate position: center horizontally, 80% vertically
    let width: u16 = 70;
    let x = cols / 2 - width / 2;
    let y = (rows as f32 * 0.8).round() as u16;

    loop {
        draw_box(width as usize, x, y, &input, &title, &mut stdout)?;

        if let Event::Key(event) = event::read()? {
            match event.code {
                KeyCode::Enter => break,
                KeyCode::Char(c) => input.push(c),
                KeyCode::Backspace => {
                    input.pop();
                }
                KeyCode::Esc => {
                    send_message(
                        "User input was cancelled".to_string(),
                        3,
                        MessageType::Error,
                    )?;
                    return Err("User canceled input".into());
                }
                _ => {}
            }
        }
    }
    stdout.execute(ResetColor)?;

    Ok(input.trim().to_string())
}

fn draw_box(
    width: usize,
    x: u16,
    y: u16,
    input: &String,
    title: &String,
    stdout: &mut Stdout,
) -> Result<(), Box<dyn Error>> {
    let prompt_top = format!("╭{:─^width$}╮", format!(" {} ", title), width = width - 2);
    let prompt_mid = format!("│ > {:width$}│", input, width = width - 5);
    let prompt_bot = format!("╰{:─^width$}╯", "", width = width - 2);

    execute!(
        stdout,
        cursor::MoveTo(x, y),
        Clear(ClearType::CurrentLine),
        Print(prompt_top),
        cursor::MoveTo(x, y + 1),
        Print(prompt_mid),
        cursor::MoveTo(x, y + 2),
        Print(prompt_bot),
    )?;

    sleep(Duration::from_millis(10));

    stdout.flush()?;
    Ok(())
}
