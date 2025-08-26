// src/file_utils.rs
use std::fs::{self, OpenOptions};
use std::io::{self, Write};

pub fn write_user_input_to_file(file_path: &String) {

    println!("Write stuff to a file, type quit to exit.");
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)
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

pub fn add_line_numbers(file_path: &String) {
    let content = fs::read_to_string(file_path)
        .expect("Failed to read file!");

    // get each line from the file and store it in a vector
    let lines: Vec<&str> = content.lines().collect();
    
    // file extension to be added
    let file_ext: &str = ".linum";
    let out_file_name = format!("{}{}", file_path, file_ext);

    let mut out_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(out_file_name)
        .expect("Unable to add line numbers!");

    // start counting from 1
    let mut counter: u16 = 1;
    for line in lines {
        writeln!(out_file, "{}. {}", counter, line).expect("Failed to write input!");
        counter += 1;
    }
}
