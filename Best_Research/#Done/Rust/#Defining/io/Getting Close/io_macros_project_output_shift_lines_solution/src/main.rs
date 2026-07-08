#![recursion_limit = "1024"]

use io_macros_project::{input, output};

fn main() {
    let time: f64;
    let mass: f64;

    input! {
        let mut distance: f64,
        time,
        mass,
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
    let hello = "Hello";

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
    }
}
