// Creates new modules from referenced code in separate files
mod test_functions;
mod shape;
mod person;

// Imports the functionality from the modules, with the option to use an alias
// (e.g. "use shape as shp;")
use shape::*;
use test_functions::*;
use person::*;

fn main() {
    // Rust's module system allows for organized and readable code

    // Example usage without needing to specify the full module path
    let rectangle = Rectangle { width: 5.0, height: 10.0 };
    println!("Rectangle width: {0}", rectangle.width());
    println!("Rectangle area: {0}", rectangle.area());
    println!("Rectangle perimeter: {0}", rectangle.perimeter());

    // Example usage with the full module path, useful in cases of library naming overlap
    // (e.g. COOLColors::shape::Rectangle)
    let rectangle = shape::Rectangle { width: 5.0, height: 10.0 };
    println!("Rectangle width: {0}", rectangle.width);
    println!("Rectangle area: {0}", rectangle.area());
    println!("Rectangle perimeter: {0}", rectangle.perimeter());

    println!("Test {}", rectangle.width());

    // Example of composition-based inheritance
    let alice = Person {
        name: String::from("Alice"),
        age: 30,
    };
    
    let alice_with_job = PersonWithJob::new(alice, String::from("Software Developer"));
    alice_with_job.say_hello_with_job();

    // In some cases, it may be more readable to use references instead of specifying the full module path
    // For example, if you have functions like api::get, api::post, api::update, and api::delete,
    talking_alot::public_function();
    quite_type::call_private_function();
}
