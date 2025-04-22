use std::io;

fn main() 
{
    menu(); 
}

fn menu()
{
    // updating this as more functions are added
    println!("Skriv inn et tall: 0-...");
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
            _ => println!("Not a valid number!")
        }
    }
}
