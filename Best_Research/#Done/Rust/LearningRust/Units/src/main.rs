use units::{
    acceleration, force, length, mass, time, velocity, Acceleration, Force, Length, Mass, Time,
    Velocity,
};

fn main() {
    let distance: Length = length::centimeters(10_000.0);
    let time: Time = time::milliseconds(9_580.0);
    let mass: Mass = mass::grams(80_000.0);

    assert_eq!(distance, length::meters(100.0));
    assert_eq!(time, time::seconds(9.58));
    assert_eq!(mass, mass::tons(0.08));
    assert!(0.1 + 0.2 != 0.3);

    let velocity: Velocity = distance / time;
    let acceleration: Acceleration = velocity / time;
    let force: Force = mass * acceleration;

    println!(
        "distance = {}",
        distance.display_as(length::LengthUnit::Meters)
    );
    println!("time = {}", time.display_as(time::TimeUnit::Seconds));
    println!("mass = {}", mass.display_as(mass::MassUnit::Kilograms));
    println!(
        "velocity = {}",
        velocity.display_as_precision(velocity::VelocityUnit::KilometersPerHour, 2)
    );
    println!(
        "acceleration = {}",
        acceleration.display_as_precision(acceleration::AccelerationUnit::StandardGravity, 4)
    );

    println!("acceleration = {}", acceleration);

    println!(
        "force = {}",
        force.display_as_precision(force::ForceUnit::Kilonewtons, 4)
    );
}
