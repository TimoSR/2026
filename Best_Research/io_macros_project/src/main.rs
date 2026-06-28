use io_macros_project::{input, output};

fn main()
{
    let distance: f64;
    let time: f64;
    let mass: f64;
    let hello: String = "Hello".to_string();

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

    println!();

    output! {
        Calculation summary,
        Distance input was {distance} bubels, which equals {centimeters} centimeters,
        The same distance is also {meters} meters,
        Time was measured as {seconds} seconds,
        Mass was measured as {kilograms} kilograms,
        Velocity was calculated as {kilometers_per_hour} kilometers per hour,
        Acceleration was calculated as {acceleration} meters per second squared,
        Gravity ratio compared to Earth gravity is {gravity},
        Force was calculated as {kilonewtons} kilonewtons,
        Message value is {hello},
    }
}
