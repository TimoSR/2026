use units::{length, mass, time, Acceleration, Force, Length, Mass, Time, Velocity};

fn main()
{
    let distance: Length = length::centimeters(10_000.0);
    let time: Time = time::milliseconds(9_580.0);
    let mass: Mass = mass::grams(80_000.0);
    let velocity: Velocity = distance / time;
    let acceleration: Acceleration = velocity / time;
    let force: Force = mass * acceleration;

    println!("distance = {} m", distance.as_meters());
    println!("time = {} s", time.as_seconds());
    println!("mass = {} kg", mass.as_kilograms());
    println!("velocity = {:.2} km/h", velocity.as_kilometers_per_hour());
    println!("acceleration = {:.4} g0", acceleration.as_standard_gravity());
    println!("acceleration = {}", acceleration);
    println!("force = {:.4} kN", force.as_kilonewtons());
}
