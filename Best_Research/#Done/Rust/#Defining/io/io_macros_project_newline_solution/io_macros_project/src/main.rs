use io_macros_project::{input, output};

fn main() {
    let distance: f64;
    let time: f64;
    let mass: f64;

    input! {
        distance,
        time,
        mass
    }

    let velocity = distance / time;
    let acceleration = velocity / time;
    let force = mass * acceleration;

    let centimeters = distance * 100.0;
    let meters = distance;
    let seconds = time;
    let kilograms = mass;
    let kilometers_per_hour = velocity * 3.6;
    let gravity = acceleration / 9.80665;
    let kilonewtons = force / 1000.0;
    let hello = String::from("Hello");

    output! {
        distance = {distance} bubels = {centimeters} centimeters
        centimeters = {centimeters}
        meters = {meters}
        time = {seconds} seconds
        mass = {kilograms} kilograms
        velocity = {kilometers_per_hour} km/h
        acceleration = {acceleration} m/s^2
        gravity = {gravity} g
        force = {kilonewtons} kN
        force = {hello}
    }
}
