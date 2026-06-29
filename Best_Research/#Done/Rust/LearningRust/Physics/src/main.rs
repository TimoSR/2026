use physics::acceleration;
use physics::force;
use physics::length;
use physics::mass;
use physics::time;
use physics::velocity;

fn main() {
    let distance = length::centimeters(10_000.0);
    let time = time::milliseconds(9_580.0);
    let mass = mass::grams(80_000.0);

    let velocity = velocity(distance, time);
    let acceleration = acceleration(velocity, time);
    let force = force(mass, acceleration);

    println!("{}", distance);
    println!("{}", distance.display_centimeters());
    println!("distance = {}", distance.display_meters());
    println!("time = {}", time.display_seconds());
    println!("mass = {}", mass.display_kilograms());
    println!("velocity = {}", velocity.display_kilometers_per_hour_precision(2));
    println!("acceleration = {}", acceleration.display_standard_gravity_precision(4));
    println!("acceleration = {}", acceleration);
    println!("force = {}", force.display_kilonewtons_precision(4));
}
