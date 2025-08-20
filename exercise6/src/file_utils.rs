// src/file_utils.rs
use std::fs::OpenOptions;
use std::io::{self, Write};

pub fn write_user_input_to_file() {

    println!("Write stuff to a file, type quit to exit.");
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("tmp.txt")
        .expect("Unable to open file!");
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");

        let trimmed = input.trim();
        if trimmed == "quit" {
            break;
        }

        writeln!(file, "{}", trimmed).expect("Failed to write input!");
    }
}
