pub fn some_public_function() {
    println!("Called public function in utils module.");
    internal_helper();
}

pub fn helper_function() {
    println!("Called another public helper function in utils.");
}

fn internal_helper() {
    println!("Called internal helper in utils.");
}
