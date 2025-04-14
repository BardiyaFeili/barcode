use std::{
    error::Error,
    fs::{self, File, OpenOptions},
    io::Write,
    path::Path,
};

use crate::{
    buffer::TextBuffer,
    input::{MessageType, send_message, user_input},
};

pub fn read_file(file: String) -> Result<TextBuffer, Box<dyn Error>> {
    OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .truncate(false)
        .open(&file)?; // Just to create if needed
    let content = fs::read_to_string(&file)?;

    Ok(TextBuffer {
        lines: content.lines().map(|line| line.to_string()).collect(),
        path: file,
    })
}

pub fn save_to_file(buffer: &mut TextBuffer) -> Result<(), Box<dyn Error>> {
    if buffer.path.is_empty() {
        let file_name = user_input("file name?".to_string())?;
        if file_name.is_empty() {
            send_message(
                "You did not provide a name for the file".to_string(),
                5,
                MessageType::Error,
            )?;
            return Ok(());
        }
        buffer.path = file_name;
    }

    let path = Path::new(&buffer.path);

    // Ensure parent directory exists
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            match user_input("Parent folder doesn't exist, Create? (y,N)".to_string())?.as_str() {
                "y" | "Y" => fs::create_dir_all(parent)?,
                _ => {
                    send_message(
                        "The parent directory did not exist".to_string(),
                        3,
                        MessageType::Error,
                    )?;
                    return Ok(());
                }
            }
        }
    }

    let mut file = File::create(&buffer.path)?;
    for line in &buffer.lines {
        writeln!(file, "{}", line)?;
    }
    send_message(
        "File saved successfully".to_string(),
        3,
        MessageType::Success,
    )?;
    Ok(())
}
