#![recursion_limit = "512"]

use io_macros_project::{input, output};

fn main() {
    let time: f64;
    let mut mass: f64;

    output! {
        << Please give me input!
    }

    input! {
        let distance: f64,
        time,
        let mut bonus_mass: f64,
        mass,
    }

    mass += bonus_mass;

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
    let hello = "Hello";
    let happy_list = [1, 3, 4];
    let happy_vec = vec![10, 20, 30];

    output! {
        << [measurement: distance] raw input = {distance}, converted value = {centimeters} centimeters, normalized value = {meters} meters.
        << [distance details] distance = {distance}, bubels = {centimeters} centimeters, meters = {meters}.
        << [time details] time = {seconds} seconds, source value = {time}, status: accepted;
        << [mass details] mass = {kilograms} kilograms, input validation: complete, range check: not applied.
        << [motion: velocity] velocity = {kilometers_per_hour} km/h, calculated from distance and time.
        << [motion: acceleration] acceleration = {acceleration} m/s^2, derived from velocity over time.
        << [gravity comparison] gravity = {gravity} g, where 1.0 g means standard Earth gravity.
        << [force calculation] force = {kilonewtons} kN, based on mass, velocity, and acceleration.
        << [summary: values] distance [{distance}], centimeters [{centimeters}], meters [{meters}], seconds [{seconds}], kilograms [{kilograms}].
        << [summary: motion] velocity [{kilometers_per_hour} km/h], acceleration [{acceleration} m/s^2], gravity [{gravity} g].
        << [summary: force] force [{kilonewtons} kN], message [{hello}].
        << [report] The object moved {meters} meters, over {seconds} seconds, with mass {kilograms} kilograms.
        << [report] The resulting velocity was {kilometers_per_hour} km/h, and acceleration was {acceleration} m/s^2.
        << [report] Compared with Earth gravity, this acceleration is {gravity} g.
        << [report] Final force output: {kilonewtons} kN.
        << [message] force label reused with text value: {hello}.
        << {happy_list}
        << {happy_vec}
    };

    println!("[measurement: distance] raw input = {}, converted value = {} centimeters, normalized value = {} meters.", distance, centimeters, meters);
    println!("[distance details] distance = {}, bubels = {} centimeters, meters = {}.", distance, centimeters, meters);
    println!("[time details] time = {} seconds, source value = {}, status: accepted;", seconds, time);
    println!("[mass details] mass = {} kilograms, input validation: complete, range check: not applied.", kilograms);
    println!("[motion: velocity] velocity = {} km/h, calculated from distance and time.", kilometers_per_hour);
    println!("[motion: acceleration] acceleration = {} m/s^2, derived from velocity over time.", acceleration);
    println!("[gravity comparison] gravity = {} g, where 1.0 g means standard Earth gravity.", gravity);
    println!("[force calculation] force = {} kN, based on mass, velocity, and acceleration.", kilonewtons);
    println!("[summary: values] distance [{}], centimeters [{}], meters [{}], seconds [{}], kilograms [{}].", distance, centimeters, meters, seconds, kilograms);
    println!("[summary: motion] velocity [{} km/h], acceleration [{} m/s^2], gravity [{} g].", kilometers_per_hour, acceleration, gravity);
    println!("[summary: force] force [{} kN], message [{}].", kilonewtons, hello);
    println!("[report] The object moved {} meters, over {} seconds, with mass {} kilograms.", meters, seconds, kilograms);
    println!("[report] The resulting velocity was {} km/h, and acceleration was {} m/s^2.", kilometers_per_hour, acceleration);
    println!("[report] Compared with Earth gravity, this acceleration is {} g.", gravity);
    println!("[report] Final force output: {} kN.", kilonewtons);
    println!("[message] force label reused with text value: {}.", hello);
    println!("{:?}", happy_list);
    println!("{:?}", happy_vec);
}
