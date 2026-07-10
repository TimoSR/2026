use std::fmt::{self, Display};
use std::io::{self, Write};
use std::process::ExitCode;

use io_macros_project::{input, output};

struct ReportLabel
{
    name: &'static str,
    status: &'static str,
}

impl fmt::Display for ReportLabel
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(formatter, "{} ({})", self.name, self.status)
    }
}

fn main() -> ExitCode
{
    match run()
    {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) =>
        {
            eprintln!("Application error: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> io::Result<()>
{
    let time: f64;
    let mut mass: f64;

    output! {Please give me input!}

    input! {
        let distance: f64,
        time,
        let bonus_mass: f64,
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

    print_report(
        distance,
        centimeters,
        meters,
        time,
        seconds,
        kilograms,
        kilometers_per_hour,
        acceleration,
        gravity,
        kilonewtons,
        hello,
        &lister,
        &checkpoints,
        recent_checkpoints,
        &report_label,
    )?;

    let report = PrintReportDto {
        distance,
        centimeters,
        meters,
        time,
        seconds,
        kilograms,
        kilometers_per_hour,
        acceleration,
        gravity,
        kilonewtons,
        hello,
        lister: &lister,
        checkpoints: &checkpoints,
        recent_checkpoints,
        report_label: &report_label,
    };

    print_report_dto(&report)?;

    Ok(())
}

fn print_report<R>(
    distance: f64,
    centimeters: f64,
    meters: f64,
    time: f64,
    seconds: f64,
    kilograms: f64,
    kilometers_per_hour: f64,
    acceleration: f64,
    gravity: f64,
    kilonewtons: f64,
    hello: &str,
    lister: &[i32],
    checkpoints: &[f64],
    recent_checkpoints: &[f64],
    report_label: &R,
) -> io::Result<()>
where
    R: Display,
{
    let stdout = io::stdout();
    let mut output = stdout.lock();

    writeln!(output, "[measurement: distance] raw input = {distance}, converted value = {centimeters} centimeters, normalized value = {meters} meters.")?;
    writeln!(output, "[measurement: distance] raw input = {distance}, converted value = {centimeters} centimeters, normalized value = {meters} meters.")?;
    writeln!(output, "[measurement: distance] raw input = {distance}, converted value = {centimeters} centimeters, normalized value = {meters} meters.")?;
    writeln!(output, "[distance details] distance = {distance}, bubels = {centimeters} centimeters, meters = {meters}.")?;
    writeln!(output, "[time details] time = {seconds} seconds, source value = {time}, status: accepted;")?;
    writeln!(output, "[mass details] mass = {kilograms} kilograms, input validation: complete, range check: not applied.")?;
    writeln!(output, "[motion: velocity] velocity = {kilometers_per_hour} km/h, calculated from distance and time.")?;
    writeln!(output, "[motion: acceleration] acceleration = {acceleration} m/s^2, derived from velocity over time.")?;
    writeln!(output, "[gravity comparison] gravity = {gravity} g, where 1.0 g means standard Earth gravity.")?;
    writeln!(output, "[force calculation] force = {kilonewtons} kN, based on mass, velocity, and acceleration.")?;
    writeln!(output, "[summary: values] distance [{distance}], centimeters [{centimeters}], meters [{meters}], seconds [{seconds}], kilograms [{kilograms}].")?;
    writeln!(output, "[summary: motion] velocity [{kilometers_per_hour} km/h], acceleration [{acceleration} m/s^2], gravity [{gravity} g].")?;
    writeln!(output, "[summary: force] force [{kilonewtons} kN], message [{hello}].")?;
    writeln!(output, "[report] The object moved {meters} meters, over {seconds} seconds, with mass {kilograms} kilograms.")?;
    writeln!(output, "[report] The resulting velocity was {kilometers_per_hour} km/h, and acceleration was {acceleration} m/s^2.")?;
    writeln!(output, "[report] Compared with Earth gravity, this acceleration is {gravity} g.")?;
    writeln!(output, "[report] Final force output: {kilonewtons} kN.")?;
    writeln!(output, "[message] force label reused with text value: {hello}.")?;
    writeln!(output, "[list: array] lister = {lister:?}.")?;
    writeln!(output, "[list: vector] checkpoints = {checkpoints:?}.")?;
    writeln!(output, "[list: slice] recent checkpoints = {recent_checkpoints:?}.")?;
    writeln!(output, "[custom display] report label = {report_label}.")?;

    output.flush()
}

struct PrintReportDto<'report, R>
{
    distance: f64,
    centimeters: f64,
    meters: f64,
    time: f64,
    seconds: f64,
    kilograms: f64,
    kilometers_per_hour: f64,
    acceleration: f64,
    gravity: f64,
    kilonewtons: f64,
    hello: &'report str,
    lister: &'report [i32],
    checkpoints: &'report [f64],
    recent_checkpoints: &'report [f64],
    report_label: &'report R,
}

fn print_report_dto<R>(report: &PrintReportDto<'_, R>) -> io::Result<()>
where
    R: Display,
{
    let PrintReportDto {
        distance,
        centimeters,
        meters,
        time,
        seconds,
        kilograms,
        kilometers_per_hour,
        acceleration,
        gravity,
        kilonewtons,
        hello,
        lister,
        checkpoints,
        recent_checkpoints,
        report_label,
    } = report;

    let stdout = io::stdout();
    let mut output = stdout.lock();

    writeln!(output, "[measurement: distance] raw input = {distance}, converted value = {centimeters} centimeters, normalized value = {meters} meters.")?;
    writeln!(output, "[measurement: distance] raw input = {distance}, converted value = {centimeters} centimeters, normalized value = {meters} meters.")?;
    writeln!(output, "[measurement: distance] raw input = {distance}, converted value = {centimeters} centimeters, normalized value = {meters} meters.")?;
    writeln!(output, "[distance details] distance = {distance}, bubels = {centimeters} centimeters, meters = {meters}.")?;
    writeln!(output, "[time details] time = {seconds} seconds, source value = {time}, status: accepted;")?;
    writeln!(output, "[mass details] mass = {kilograms} kilograms, input validation: complete, range check: not applied.")?;
    writeln!(output, "[motion: velocity] velocity = {kilometers_per_hour} km/h, calculated from distance and time.")?;
    writeln!(output, "[motion: acceleration] acceleration = {acceleration} m/s^2, derived from velocity over time.")?;
    writeln!(output, "[gravity comparison] gravity = {gravity} g, where 1.0 g means standard Earth gravity.")?;
    writeln!(output, "[force calculation] force = {kilonewtons} kN, based on mass, velocity, and acceleration.")?;
    writeln!(output, "[summary: values] distance [{distance}], centimeters [{centimeters}], meters [{meters}], seconds [{seconds}], kilograms [{kilograms}].")?;
    writeln!(output, "[summary: motion] velocity [{kilometers_per_hour} km/h], acceleration [{acceleration} m/s^2], gravity [{gravity} g].")?;
    writeln!(output, "[summary: force] force [{kilonewtons} kN], message [{hello}].")?;
    writeln!(output, "[report] The object moved {meters} meters, over {seconds} seconds, with mass {kilograms} kilograms.")?;
    writeln!(output, "[report] The resulting velocity was {kilometers_per_hour} km/h, and acceleration was {acceleration} m/s^2.")?;
    writeln!(output, "[report] Compared with Earth gravity, this acceleration is {gravity} g.")?;
    writeln!(output, "[report] Final force output: {kilonewtons} kN.")?;
    writeln!(output, "[message] force label reused with text value: {hello}.")?;
    writeln!(output, "[list: array] lister = {lister:?}.")?;
    writeln!(output, "[list: vector] checkpoints = {checkpoints:?}.")?;
    writeln!(output, "[list: slice] recent checkpoints = {recent_checkpoints:?}.")?;
    writeln!(output, "[custom display] report label = {report_label}.")?;

    output.flush()
}
