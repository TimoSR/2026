pub mod test {
    pub trait Summary {
        fn summarize(&self) -> String;
    }

    #[derive(Debug)]
    pub struct Article {
        headline: String,
        location: String,
        artist: String,
    }

    // I prefer String::from to take ownership as I tells where the data is located in the example

    impl Article {
        pub fn new_boxed(headline: &str, location: &str, artist: &str) -> Box<Self> {
            return Box::new(Article {
                artist: String::from(artist),
                location: String::from(location),
                headline: String::from(headline),
            });
        }

        pub fn new(headline: &str, location: &str, artist: &str) -> Self {
            return Article {
                artist: String::from(artist),
                location: location.to_owned(),
                headline: headline.to_string(),
            };
        }
    }

    impl Summary for Article {
        fn summarize(&self) -> String {
            format!(
                "{0}, ({1}) by {2} ",
                self.headline, self.location, self.artist
            )
        }
    }

    pub struct Point {
        x: i32,
        y: i32,
    }

    impl Point {
        pub fn new(x: i32, y: i32) -> Self {
            return Point{x, y}
        }

        pub fn new_boxed(x: i32, y: i32) -> Box<Self> {
            return Box::new(Point{x, y});
        }
    }

    pub trait Calculations {
        fn distance_to(self, point: &Point) -> (i32, i32);
    }

    impl Calculations for Point {
        fn distance_to(self, point: &Point) -> (i32, i32) {
            return ((self.x - point.x ).abs(), (self.y - point.y).abs());
        }
    }
}
