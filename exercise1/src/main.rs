use std::io;
// Translate Python to C++, nvm, to Rust

// Task a)
fn max_of_two(a: i32, b: i32) -> i32
{
    if a > b { 
        println!("A is greater than B");
        return a;
    }
    else {
        println!("B is greater than or equal to A");
        return b;
    }
}
// 
// Task c)
fn fibonacci(n: i32) -> i32
{
    let mut a: i32 = 0;
    let mut b: i32 = 1;
    println!("Fibonacci numbers:");

    for x in 1..n+1 {
        println!("{} {}", x, b);
        let temp: i32 = b;
        b += a;
        a = temp;
    }
    println!("----");
    return b;
}
// 
// Task d)
fn square_number_sum(n: i32) -> i32
{
    let mut total_sum: i32 = 0;
    for i in 1..n+1 {
        total_sum += i * i;
        println!("{}", i * i);
    }
    println!("{}", total_sum);
    return total_sum;
}
//
// Task e)
fn triangle_numbers_below(n: i32)
{
    let mut acc: i32 = 1;
    let mut num: i32 = 2;
    println!("Triangle numbers below {}",n);

    while acc < n {
        println!("{}",acc);
        acc += num;
        num += 1;
    }
    println!("")
}
// 
// Task f)
fn is_prime(n: i32) -> bool
{
    for j in 2..n {
        if n%j == 0 { return false; }
    }
    return true;
}
//
// Task g)
fn naive_prime_number_search(n: i32)
{
    for number in 2..n {
        if is_prime(number) {
            println!("{} is a prime", number);
        }
    }
}
//
// Task h)
fn input_and_print_name_and_age()
{
    let mut name = String::new();
    let mut age = String::new();
    println!("Skriv inn et navn: ");
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    println!("Skriv inn alderen til {} : ", name);
    io::stdin()
        .read_line(&mut age)
        .expect("Failed to read line");
    println!("{} er {} år gammel", name, age);

}

fn main() {
    // println!("Oppgave a)");
    // println!("{}", max_of_two(5, 6));
    // println!("Oppgave c)");
    // println!("{}",fibonacci(10));
    // println!("Oppgave d)");
    // println!("{}", square_number_sum(9));
    // println!("Oppgave e)");
    // triangle_numbers_below(10);
    // println!("Oppgave f-g)");
    // naive_prime_number_search(14);
    // println!("Oppgave h)");
    input_and_print_name_and_age();

}
