fn add_one(x: i32) -> i32 {
    x + 1
}

fn double(x: i32) -> i32 {
    x * 2
}

fn main() {
    //let data :script_functions::WorkingData;
    let mut fn_vec :Vec<fn(i32) -> i32> = Vec::new();

    fn_vec.push(add_one);
    fn_vec.push(double);

    for func in fn_vec {
        println!("Result: {}", func(5));
    }
}