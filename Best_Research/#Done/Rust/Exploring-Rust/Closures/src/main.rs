
fn do_more_math<F>(operation: F) -> i32
where
    F: Fn(i32) -> i32, // `operation` must be a closure that takes an i32 and returns an i32
{
    let number_to_process = 10; // Example value
    let result = operation(number_to_process); // Call the passed-in closure
    println!(
        "Called the closure with {}. Result: {}",
        number_to_process, result
    );

    return result;
}

fn main() {
    // Evaluated at compile time
    const ADD_TWO: fn (i32) -> i32 = |a: i32| -> i32 {
        return a + 2;
    };

    // May be evaluated at compile time
    // Simpler and easier to read
    let add_three = |a: i32| {
        return a + 3;
    };

    fn do_math(x:i32, a: fn(i32) -> i32) -> i32 {
        return a(x);
    }

    // a good example how to pass a function as input
    // and captures the scoped variable inside do_more_math
    let result = do_more_math(move |input: i32| {
        return input + 100;
    });

    println!("{}",ADD_TWO(2));
    println!("{0}",add_three(2));
    println!("{0}",ADD_TWO(6));
    println!("{}", do_math(12, add_three));
    println!("{0}", result);
}
