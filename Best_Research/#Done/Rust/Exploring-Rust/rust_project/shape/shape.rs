// Traits can be seen as interfaces

pub trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl Rectangle {
    pub fn width(&self) -> f64 {
        self.width
    }
}

impl Shape for Rectangle {

    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 { 2.0 * (self.width + self.height) }
}