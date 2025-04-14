use std::{
    error::Error,
    fs::{self, File, OpenOptions},
    io::Write,
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
        let file_name = user_input()?;
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
