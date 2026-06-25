use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

// ---------- Core newtypes ----------
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct Length(pub f64); // meters

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct Time(pub f64); // seconds

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct Mass(pub f64); // kilograms

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct Velocity(pub f64); // m/s

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct Acceleration(pub f64); // m/s^2

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct Force(pub f64); // N = kg·m/s^2

// ---------- Constructors ----------
#[inline]
pub fn meters(v: f64) -> Length {
    return Length(v);
}

#[inline]
pub fn seconds(v: f64) -> Time {
    return Time(v);
}

#[inline]
pub fn kilogram(v: f64) -> Mass {
    return Mass(v);
}

#[inline]
pub fn meters_pr_second(v: f64) -> Velocity {
    return Velocity(v);
}

#[inline]
pub fn meters_pr_second_pr_second(v: f64) -> Acceleration {
    return Acceleration(v);
}

#[inline]
pub fn force(v: f64) -> Force {
    return Force(v);
}

// ---------- Pretty printing ----------
impl fmt::Display for Length {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{} m", self.0);
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{} s", self.0);
    }
}

impl fmt::Display for Mass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{} kg", self.0);
    }
}

impl fmt::Display for Velocity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{} m/s", self.0);
    }
}

impl fmt::Display for Acceleration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{} m/s^2", self.0);
    }
}

impl fmt::Display for Force {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{} N", self.0);
    }
}

// ---------- Same-type +/− ----------
macro_rules! impl_add_sub_same {
    ($t:ident) => {
        impl Add<$t> for $t {
            type Output = $t;

            #[inline]
            fn add(self, rhs: $t) -> Self::Output {
                return $t(self.0 + rhs.0);
            }
        }

        impl Sub<$t> for $t {
            type Output = $t;

            #[inline]
            fn sub(self, rhs: $t) -> Self::Output {
                return $t(self.0 - rhs.0);
            }
        }
    };
}

impl_add_sub_same!(Length);
impl_add_sub_same!(Time);
impl_add_sub_same!(Mass);
impl_add_sub_same!(Velocity);
impl_add_sub_same!(Acceleration);
impl_add_sub_same!(Force);

// ---------- Scalar * and / ----------
macro_rules! impl_scalar {
    ($t:ident) => {
        impl Mul<f64> for $t {
            type Output = $t;

            #[inline]
            fn mul(self, rhs: f64) -> Self::Output {
                return $t(self.0 * rhs);
            }
        }

        impl Div<f64> for $t {
            type Output = $t;

            #[inline]
            fn div(self, rhs: f64) -> Self::Output {
                return $t(self.0 / rhs);
            }
        }

        impl Mul<$t> for f64 {
            type Output = $t;

            #[inline]
            fn mul(self, rhs: $t) -> Self::Output {
                return $t(self * rhs.0);
            }
        }
    };
}

impl_scalar!(Length);
impl_scalar!(Time);
impl_scalar!(Mass);
impl_scalar!(Velocity);
impl_scalar!(Acceleration);
impl_scalar!(Force);

// ---------- Cross-unit ops ----------

// v = L / T
impl Div<Time> for Length {
    type Output = Velocity;

    #[inline]
    fn div(self, rhs: Time) -> Self::Output {
        return Velocity(self.0 / rhs.0);
    }
}

// a = v / T
impl Div<Time> for Velocity {
    type Output = Acceleration;

    #[inline]
    fn div(self, rhs: Time) -> Self::Output {
        return Acceleration(self.0 / rhs.0);
    }
}

// L = v * T
impl Mul<Time> for Velocity {
    type Output = Length;

    #[inline]
    fn mul(self, rhs: Time) -> Self::Output {
        return Length(self.0 * rhs.0);
    }
}

// L = T * v
impl Mul<Velocity> for Time {
    type Output = Length;

    #[inline]
    fn mul(self, rhs: Velocity) -> Self::Output {
        return Length(self.0 * rhs.0);
    }
}

// v = a * T
impl Mul<Time> for Acceleration {
    type Output = Velocity;

    #[inline]
    fn mul(self, rhs: Time) -> Self::Output {
        return Velocity(self.0 * rhs.0);
    }
}

// v = T * a
impl Mul<Acceleration> for Time {
    type Output = Velocity;

    #[inline]
    fn mul(self, rhs: Acceleration) -> Self::Output {
        return Velocity(self.0 * rhs.0);
    }
}

// F = m * a
impl Mul<Acceleration> for Mass {
    type Output = Force;

    #[inline]
    fn mul(self, rhs: Acceleration) -> Self::Output {
        return Force(self.0 * rhs.0);
    }
}

impl Mul<Mass> for Acceleration {
    type Output = Force;

    #[inline]
    fn mul(self, rhs: Mass) -> Self::Output {
        return Force(self.0 * rhs.0);
    }
}

// a = F / m
impl Div<Mass> for Force {
    type Output = Acceleration;

    #[inline]
    fn div(self, rhs: Mass) -> Self::Output {
        return Acceleration(self.0 / rhs.0);
    }
}

// m = F / a
impl Div<Acceleration> for Force {
    type Output = Mass;

    #[inline]
    fn div(self, rhs: Acceleration) -> Self::Output {
        return Mass(self.0 / rhs.0);
    }
}

// ---------- Demo ----------
fn main() {
    let distance = meters(100.0);
    let time = seconds(9.58);
    let mass = kilogram(80.0);

    let velocity: Velocity = distance / time; // m/s
    let acceleration: Acceleration = velocity / time; // m/s^2
    let force: Force = mass * acceleration; // N

    println!("velocity = {}", velocity);
    println!("acceleration = {}", acceleration);
    println!("force = {}", force);

    // Uncomment to see a friendly compile error:
    // let bad = distance + time; // error: cannot add `Time` to `Length`
}