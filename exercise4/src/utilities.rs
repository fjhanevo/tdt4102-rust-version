use std::fmt;
use rand::{random, Rng};

pub fn incerement_by_value_num_times(mut start_value: u32, increment: u32, num_times: u32) -> u32
{
    for _ in 0..=num_times {
        start_value += increment;
    }
    start_value
}

pub fn swap_numbers(a: &mut u32, b: &mut u32)
{
    let temp = *a;
    *a = *b;
    *b = temp;
}

#[derive(Debug)]
pub struct Student {
    pub name: String,
    pub study_program: String,
    pub age: u32,
}

// Nice way to display Student with println!; 
impl fmt::Display for Student {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "--- Student Info ---")?;
        writeln!(f, "Name : {}",self.name)?;
        writeln!(f, "Study: {}",self.study_program)?;
        writeln!(f, "Age  : {}", self.age)
    }
}

pub fn print_student(student: &Student)
{
    println!("{}", student);
}

fn random_with_limits(lower_lim: u32, upper_lim: u32) -> u32
{
    let mut rng = rand::thread_rng();
    rng.gen_range(lower_lim..=upper_lim)
}

pub fn randomize_string(n: u32, lower_lim: char, upper_lim: char) -> String
{
    let mut return_string = String::new();
    for _ in 0..n {
        let rand_u32 = random_with_limits(lower_lim as u32, upper_lim as u32);
        if let Some(c) = char::from_u32(rand_u32) { 
            return_string.push(c);
        }
    } 
    return_string
}
