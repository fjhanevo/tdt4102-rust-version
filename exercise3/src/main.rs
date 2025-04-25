pub mod cannonball;
pub mod utilities;

fn main() {
    let vec = cannonball::get_velocity_vector(27.5, 15.64);

    println!("{}, {}", vec[0], vec[1]);
    cannonball::play_target_practice();

}
