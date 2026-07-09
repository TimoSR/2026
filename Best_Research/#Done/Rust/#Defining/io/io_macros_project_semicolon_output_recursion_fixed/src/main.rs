use std::fmt;
use std::io;

use io_macros_project::{input, output, output_buffered_to, output_to};

struct ReportLabel {
    name: &'static str,
    status: &'static str,
}

impl fmt::Display for ReportLabel {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} ({})", self.name, self.status)
    }
}

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
    let lister = [1, 3, 4];
    let checkpoints = vec![distance, centimeters, meters];
    let recent_checkpoints = &checkpoints[1..];
    let report_label = ReportLabel { name: "force", status: "accepted" };

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
        << [list: array] lister = {lister}.
        << [list: vector] checkpoints = {checkpoints}.
        << [list: slice] recent checkpoints = {recent_checkpoints}.
        << [custom display] report label = {report_label}.
    }

    println!("[measurement: distance] raw input = {distance}, converted value = {centimeters} centimeters, normalized value = {meters} meters.");
    println!("[distance details] distance = {distance}, bubels = {centimeters} centimeters, meters = {meters}.");
    println!("[time details] time = {seconds} seconds, source value = {time}, status: accepted;");
    println!("[mass details] mass = {kilograms} kilograms, input validation: complete, range check: not applied.");
    println!("[motion: velocity] velocity = {kilometers_per_hour} km/h, calculated from distance and time.");
    println!("[motion: acceleration] acceleration = {acceleration} m/s^2, derived from velocity over time.");
    println!("[gravity comparison] gravity = {gravity} g, where 1.0 g means standard Earth gravity.");
    println!("[force calculation] force = {kilonewtons} kN, based on mass, velocity, and acceleration.");
    println!("[summary: values] distance [{distance}], centimeters [{centimeters}], meters [{meters}], seconds [{seconds}], kilograms [{kilograms}].");
    println!("[summary: motion] velocity [{kilometers_per_hour} km/h], acceleration [{acceleration} m/s^2], gravity [{gravity} g].");
    println!("[summary: force] force [{kilonewtons} kN], message [{hello}].");
    println!("[report] The object moved {meters} meters, over {seconds} seconds, with mass {kilograms} kilograms.");
    println!("[report] The resulting velocity was {kilometers_per_hour} km/h, and acceleration was {acceleration} m/s^2.");
    println!("[report] Compared with Earth gravity, this acceleration is {gravity} g.");
    println!("[report] Final force output: {kilonewtons} kN.");
    println!("[message] force label reused with text value: {hello}.");
    println!("[list: array] lister = {lister:?}.");
    println!("[list: vector] checkpoints = {checkpoints:?}.");
    println!("[list: slice] recent checkpoints = {recent_checkpoints:?}.");
    println!("[custom display] report label = {report_label}.");

    let mut direct_writer = io::stdout();

    output_to! {
        writer: &mut direct_writer,
        << [writer output] output_to! writes directly to any Write target.
    }

    output_buffered_to! {
        writer: &mut direct_writer,
        buffer: 8 KB,
        << [writer output] output_buffered_to! builds one buffer, then writes it once.
    }
}
