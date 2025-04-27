mod test;
mod utilities;
use std::io;

fn main() 
{
    menu();
}

fn menu()
{
    println!("{}", concat!(
        "Choose function (0-2)\n",
        "0) Quit\n",
        "1) test_call_by_value()\n",
        "2) test_swap_numbers()\n",
        "3) test_student_struct()\n"
    ));

    loop {
        let mut input = String::new();
        
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");

        // convert input to int
        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match input {
            0 => break, 
            1 => test::test_call_by_value(),
            2 => test::test_swap_numbers(),
            3 => test::test_student_struct(),
            _ => println!("Not a choice!")
        }
    }
}
