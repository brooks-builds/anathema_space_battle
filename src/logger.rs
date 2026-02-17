use std::{fs::OpenOptions, io::Write};

pub fn log(message: String, name: &str) {
    let file_name = format!("log_{name}.log");
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_name)
        .unwrap();

    file.write_all(message.as_bytes()).unwrap();
}
