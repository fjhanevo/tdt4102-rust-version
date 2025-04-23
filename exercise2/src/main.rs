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
        "6) input_integers_and_print_sum_until_stopped()\n",
        "7) input_double()\n",
        "8) convert_nok_to_eur()\n",
        "9) print_multiplication_table()\n",
        "10) solve_quadratic_equation()\n"
    ));
    loop {
        
        println!("---------");
        print!("Skriv inn tall (0-10): ");

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
                let number = input_integer();
                println!("Du skrev: {number}");
            },
            3 => print_sum_of_input_integers(),
            4 => {
                for number in 0..15 {
                    println!("{} is odd: {}", number,is_odd(number));
                }
            },
            5 => input_integers_and_print_sum(),
            6 => input_integers_and_print_sum_until_stopped(),
            7 => {
                let number =input_double();
                println!("Du skrev: {number}");
            } 
            8 => convert_nok_to_eur(),
            9 => print_multiplication_table(),
            10 => solve_quadratic_equation(),
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

fn input_double() -> f32
{
    println!("Skriv inn et flyttall");
    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line!");
    
    let number: f32 = number.trim().parse().expect("Not a number!");
    number
}

fn convert_nok_to_eur()
{
    let number = input_double();

    const EUR_TO_NOK: f32 =  10.5;
    println!("{number} NOK == {} EUR", number/EUR_TO_NOK);
}

fn print_multiplication_table()
{
    println!("Enter height:");
    let mut height = String::new();
    io::stdin()
        .read_line(&mut height)
        .expect("Failed to read line!");

    let height: u32 = height.trim().parse().expect("Not a number!");

    println!("Enter width:");
    let mut width = String::new();
    io::stdin()
        .read_line(&mut width)
        .expect("Failed to read line!");

    let width: u32 = width.trim().parse().expect("Not a number!");

    for i in 1..=height {
        for j in 1..=width {
            print!("{}   ", i*j);
        }
        println!("");
    }
}

fn discriminant(a: f32, b: f32, c: f32) -> f32
{
    b.powi(2) - 4.0*a*c
}

fn print_real_roots(a: f32, b: f32, c: f32)
{
    let disc: f32 = discriminant(a, b, c);
    if disc > 0.0 {
        println!("Solution 1: {}", (-b + f32::sqrt(disc)) / 2.0*a);
        println!("Solution 2: {}", (-b - f32::sqrt(disc)) / 2.0*a);
    }
    else if disc == 0.0 {
        println!("Solution: {}", (-b + f32::sqrt(disc)) / 2.0*a);
    } 
    else {
        println!("No solution!")
    }
}

fn solve_quadratic_equation()
{
    println!("Skriv inn a,b og c for andregradsligningen:");
    let a: f32 = input_double();
    let b: f32 = input_double();
    let c: f32 = input_double();
    print_real_roots(a, b, c);
}

fn calculate_balance(amount: i32, rate: i32, years: u32) -> Vec<i32>
{
    let mut result: Vec<i32> = Vec::new();
    for year in 0..=years {
        result.push(amount*(1+rate/100).pow(year));
    }
    result
}
