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
