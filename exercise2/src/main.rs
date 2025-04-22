use std::io::{self, Write};

fn main() 
{
    menu(); 
}

fn menu()
{
    // updating this as more functions are added
    println!("{}", concat!(
        "Velg funksjon:\n",
        "0) Avslutt\n",
        "1) input_and_print_integer()\n",
        "2) input_integer()\n",
        "3) print_sum_of_input_integers()\n",
        "4) is_odd()\n",
        "5) input_integers_and_print_sum()\n",
        "6) input_integers_and_print_sum_until_stopped()\n"
    ));
    loop {
        
        println!("---------");

        // flush to ensure prompt prints
        io::stdout().flush().unwrap();

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
            1 => input_and_print_integer(),
            2 => {
                let result = input_integer();
                println!("Du skrev: {result}");
            },
            3 => print_sum_of_input_integers(),
            4 => {
                for number in 0..15 {
                    println!("{} is odd: {}", number,is_odd(number));
                }
            },
            5 => input_integers_and_print_sum(),
            6 => input_integers_and_print_sum_until_stopped(),
            _ => println!("Ugyldig tall!")
        }
    }
}

fn input_and_print_integer()
{
    let mut number = String::new();

    println!("Skriv inn et tall:");
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line!");
    let number: u32 = number.trim().parse().expect("Please type a number!");

    println!("Du skrev: {}", number)

}

fn input_integer() -> i32
{
    let mut number = String::new();

    println!("Skriv inn et tall:");
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line!");

    let number: i32 = number.trim().parse().expect("Please type a number!");
    return number;
}

fn print_sum_of_input_integers()
{
    let n1 = input_integer();
    let n2 = input_integer();

    println!("Summen av tallene: {}", n1+n2);
}

fn is_odd(n: i32) -> bool
{
    n % 2 != 0
}

fn input_integers_and_print_sum()
{
    println!("Skriv inn hvor mange tall du vil ha:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");

    let input: i32 = input.trim().parse().expect("Please type a number!");

    let mut counter: i32 = 0;

    for _ in 0..input {
        let number = input_integer();
        counter += number;
    }
    println!("Summen av alle tallene ble: {counter}");
}

fn input_integers_and_print_sum_until_stopped()
{
    println!("Skriv inn antall tall du vil summere (0 for avslutt):");

    let mut counter: i32 = 0;
    loop {
        let number = input_integer();
        if number == 0 { break }
        counter += number;
    }
    println!("Summen av alle tallene ble: {counter}");
}
