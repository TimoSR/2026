
pub mod talking_alot {

    pub fn public_function() {
        println!("Hello World!");
    }
}

pub mod quite_type {

    fn private_function() {
        println!("This is a private function");
    }
    
    pub fn call_private_function() {
        private_function();
    }
}