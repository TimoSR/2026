use std::{collections::HashMap, mem};

fn main() {
    
    // https://doc.rust-lang.org/std/collections/

    let mut scores = HashMap::new();

    scores.insert("Selma", 999);
    scores.insert("Timothy", 50);

    println!("{:#?}", scores);

    let score = scores.get("Selma");

    match score {
        Some(x) => println!("Score: {x}"),
        None => println!("Key was not found")
    }

    let mut vector = Vec::new();

    vector.push(5);
    vector.push(10);
    vector.push(20);

    println!("Printing Vector: {:?}", vector);
    println!("Pretty Print:\n{:#?}", vector);

    const MY_STRING: &str = "Hello, World!"; // ['H','e','l'...]

    const GET_CHAR_OF_STRING: fn(&str, usize) -> char  = |_string: &str, index: usize| -> char {
        return MY_STRING.chars().nth(index).expect("String index should not be empty");
    };

    println!("{} {:?}", MY_STRING.len(), GET_CHAR_OF_STRING(MY_STRING, 0));
}