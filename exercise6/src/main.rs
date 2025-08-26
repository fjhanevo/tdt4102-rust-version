mod file_utils;
use file_utils::{write_user_input_to_file, add_line_numbers };
fn main() {
    let file = "tmp.txt";
    write_user_input_to_file(&file.to_owned());
    add_line_numbers(&file.to_owned());

}
