use std::ops::{Div, Mul};

use crate::{Acceleration, Force, Length, Mass, Time, Velocity};

impl Div<Time> for Length {
    type Output = Velocity;

    fn div(self, time: Time) -> Self::Output {
        Velocity::meters_per_second(self.as_meters() / time.as_seconds())
    }
}

impl Div<Velocity> for Length {
    type Output = Time;

    fn div(self, velocity: Velocity) -> Self::Output {
        Time::seconds(self.as_meters() / velocity.as_meters_per_second())
    }
}

impl Mul<Time> for Velocity {
    type Output = Length;

    fn mul(self, time: Time) -> Self::Output {
        Length::meters(self.as_meters_per_second() * time.as_seconds())
    }
}

impl Mul<Velocity> for Time {
    type Output = Length;

    fn mul(self, velocity: Velocity) -> Self::Output {
        Length::meters(self.as_seconds() * velocity.as_meters_per_second())
    }
}

impl Div<Time> for Velocity {
    type Output = Acceleration;

    fn div(self, time: Time) -> Self::Output {
        Acceleration::meters_per_second_squared(self.as_meters_per_second() / time.as_seconds())
    }
}

impl Div<Acceleration> for Velocity {
    type Output = Time;

    fn div(self, acceleration: Acceleration) -> Self::Output {
        Time::seconds(self.as_meters_per_second() / acceleration.as_meters_per_second_squared())
    }
}

impl Mul<Time> for Acceleration {
    type Output = Velocity;

    fn mul(self, time: Time) -> Self::Output {
        Velocity::meters_per_second(self.as_meters_per_second_squared() * time.as_seconds())
    }
}

impl Mul<Acceleration> for Time {
    type Output = Velocity;

    fn mul(self, acceleration: Acceleration) -> Self::Output {
        Velocity::meters_per_second(self.as_seconds() * acceleration.as_meters_per_second_squared())
    }
}

impl Mul<Acceleration> for Mass {
    type Output = Force;

    fn mul(self, acceleration: Acceleration) -> Self::Output {
        Force::newtons(self.as_kilograms() * acceleration.as_meters_per_second_squared())
    }
}

impl Mul<Mass> for Acceleration {
    type Output = Force;

    fn mul(self, mass: Mass) -> Self::Output {
        Force::newtons(self.as_meters_per_second_squared() * mass.as_kilograms())
    }
}

impl Div<Mass> for Force {
    type Output = Acceleration;

    fn div(self, mass: Mass) -> Self::Output {
        Acceleration::meters_per_second_squared(self.as_newtons() / mass.as_kilograms())
    }
}

impl Div<Acceleration> for Force {
    type Output = Mass;

    fn div(self, acceleration: Acceleration) -> Self::Output {
        Mass::kilograms(self.as_newtons() / acceleration.as_meters_per_second_squared())
    }
}
