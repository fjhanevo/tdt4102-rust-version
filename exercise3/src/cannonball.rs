use std::io;
use std::f32;
use crate::utilities::random_with_limits;

fn accl_y() -> f32 { -9.81 }

fn vel_y(init_vel: f32, time: f32) -> f32
{
    init_vel * accl_y() * time 
}

fn pos_x(init_pos: f32, init_vel: f32, time: f32) -> f32
{
    init_pos + init_vel*time
}

fn pos_y(init_pos: f32, init_vel: f32, time: f32) -> f32
{
    init_pos + init_vel*time + (accl_y()*time)/2.0
}

fn print_time(time: f32)
{
    let total_seconds = time.round() as u32;
    let hrs = total_seconds / 3600;
    let mins = (total_seconds % 3600) / 60;
    let secs = total_seconds % 60;

    println!("{:02}:{:02}:{:02}", hrs, mins, secs);

}

fn flight_time(init_vel_y: f32) -> f32
{
    -2.0 * init_vel_y/accl_y()
}

pub fn get_user_input_theta() -> f32
{
    println!("Skriv inn vinkel: ");
    let mut theta = String::new();

    io::stdin()
        .read_line(&mut theta)
        .expect("Failed to read line!");
    let theta: f32 = theta.trim().parse().expect("Not a number!");

    theta
}

pub fn get_user_input_init_velocity() -> f32
{
    println!("Skriv inn hastighet: ");
    let mut velocity = String::new();

    io::stdin()
        .read_line(&mut velocity)
        .expect("Failed to read line!");
    let velocity: f32 = velocity.trim().parse().expect("Not a number!");

    velocity
}

fn deg_to_rad(deg: f32) -> f32
{
    deg * std::f32::consts::PI / 180.0 
}

pub fn get_velocity_x(theta: f32, init_vel: f32) -> f32
{
    init_vel * deg_to_rad(theta).cos()
}

pub fn get_velocity_y(theta: f32, init_vel: f32) -> f32
{
    init_vel * deg_to_rad(theta).sin()
}

pub fn get_velocity_vector(theta: f32, init_vel: f32) -> Vec<f32>
{
    let mut result = Vec::new();
    result.push(get_velocity_x(theta, init_vel));
    result.push(get_velocity_y(theta, init_vel));

    result
}

pub fn get_distance_traveled(vel_x: f32, vel_y: f32) -> f32
{
    pos_x(0.0, vel_x, flight_time(vel_y))
}

fn target_practice(dist_to_target: f32, vel_x: f32, vel_y: f32) -> f32
{
    (dist_to_target - get_distance_traveled(vel_x, vel_y)).abs()
}

pub fn play_target_practice()
{
    const MIN_TARGET: u32 = 100;
    const MAX_TARGET: u32 = 1000;
    const MAX_TRIES: u32 = 10;
    const MAX_ERROR: f32 = 5.0;

    let distance_to_target = random_with_limits(MIN_TARGET, MAX_TARGET) as f32;
    let mut won: bool = false;

    // For debugging
    println!("Target at {}", distance_to_target);
    for _ in 0..MAX_TRIES {
        let vector: Vec<f32> = get_velocity_vector(get_user_input_theta(), get_user_input_init_velocity());
        let error = target_practice(distance_to_target,vector[0],vector[1]);
        let dist_traveled = get_distance_traveled(vector[0], vector[1]);

        println!("Your shot went: {} m\nDistance from target {} m", dist_traveled, error);
        if error < MAX_ERROR {
            won = true;
            break;
        }
        if dist_traveled > distance_to_target {
            println!("You overshot the target\n")
        }
        else {
            println!("You didn't reach the target\n")
        }
    }
    if won { println!("You won!") }
    else { println!("You lost!") }



}
