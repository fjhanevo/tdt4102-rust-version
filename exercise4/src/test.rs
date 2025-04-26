use crate::utilities::{incerement_by_value_num_times,swap_numbers};

pub fn test_call_by_value()
{
    let v0 = 0;
    let increment = 2;
    let iterations = 10;
    let result = incerement_by_value_num_times(v0, increment, iterations);
    println!("\
        v0: {v0}
increment: {increment}
iterations: {iterations}
result: {result}");

}

pub fn test_swap_numbers()
{
    let mut x = 5;
    let mut y = 10;
    println!("Original:
x = {x}
y = {y}");

    swap_numbers(&mut x, &mut y);
    println!("Swapped:
x = {x}
y = {y}");
}
