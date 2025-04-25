use rand::Rng;

pub fn random_with_limits(lower_lim: u32, upper_lim: u32) -> u32
{
    let mut rng = rand::thread_rng();
    rng.gen_range(lower_lim..=upper_lim)
}
