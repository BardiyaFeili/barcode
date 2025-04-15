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
    println!("save file called");
    if buffer.path.is_empty() {
        buffer.path = ask_for_file_name()?;
    }

    let mut file = File::create(&buffer.path)?;
    for line in &buffer.lines {
        writeln!(file, "{}", line)?;
    }
    send_message(
        format!("File {} saved successfully", buffer.path),
        3,
        MessageType::Success,
    )?;
    Ok(())
}

pub fn save_as(buffer: &mut TextBuffer) -> Result<(), Box<dyn Error>> {
    println!("Save as called");
    let new_name = ask_for_file_name()?;

    buffer.path = new_name;
    save_to_file(buffer)?;

    Ok(())
}

fn ask_for_file_name() -> Result<String, Box<dyn Error>> {
    println!("Asked for file name");
    let file_name = user_input("file name?".to_string())?;
    if file_name.is_empty() {
        send_message(
            "You did not provide a name for the file".to_string(),
            5,
            MessageType::Error,
        )?;
    }

    confrim_path(&file_name)?;
    Ok(file_name)
}

fn confrim_path(file_name: &String) -> Result<(), Box<dyn Error>> {
    // Ensure parent directory exists
    let path = Path::new(&file_name);
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() && !parent.exists() {
            match user_input("Parent folder doesn't exist, Create? (y,N)".to_string())?.as_str() {
                "y" | "Y" => fs::create_dir_all(parent)?,
                _ => {
                    send_message(
                        "The parent directory did not exist".to_string(),
                        3,
                        MessageType::Error,
                    )?;
                    return Err("User declined to create the parent directory".into());
                }
            }
        }
    }
    Ok(())
}
