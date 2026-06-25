
// using a module to simplify imports in mod.rs

pub mod person_impl {

    // Example of composition as inheritance

    pub struct Person {
        pub name: String,
        pub age: u32,
    }
    
    impl Person {
        pub fn say_hello(&self) {
            println!("Hello, my name is {} and I'm {} years old.", self.name, self.age);
        }
    }
    
    pub struct PersonWithJob {
        pub person: Person,
        pub job_title: String,
    }
    
    impl PersonWithJob {
        pub fn new(person: Person, job_title: String) -> PersonWithJob {
            PersonWithJob {
                person,
                job_title,
            }
        }
    
        pub fn say_hello_with_job(&self) {
            self.person.say_hello();
            println!("I work as a {}.", self.job_title);
        }
    }

}