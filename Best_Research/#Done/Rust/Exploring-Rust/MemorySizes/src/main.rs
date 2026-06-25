use std::mem;

fn main() {

    // Use size_of::<i32>() to get the size in bytes
    let size_in_bytes = mem::size_of::<i32>();

    // Print the result
    println!("The size of an i32 is: {} bytes", size_in_bytes);

    // You can do this for other types too:
    println!("The size of a u8 is: {} byte", mem::size_of::<u8>());
    println!("The size of an f64 is: {} bytes", mem::size_of::<f64>());
    println!("The size of a bool is: {} byte", mem::size_of::<bool>()); // Note: bool size is implementation-defined, but often 1
    println!("The size of a pointer is: {} bytes", mem::size_of::<*const u8>()); // Size depends on architecture (e.g., 8 on 64-bit)
}
