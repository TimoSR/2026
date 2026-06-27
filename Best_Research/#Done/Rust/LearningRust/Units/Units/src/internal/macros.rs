// This declarative macro removes repetitive same-dimension arithmetic.
// A procedural macro could simplify the quantity modules further if the unit list grows.
macro_rules! implement_quantity_arithmetic {
    ($quantity_type:ident) => {
        impl $quantity_type {
            pub const ZERO: Self = Self(0.0);

            #[must_use]
            pub const fn raw_si(self) -> f64 {
                self.0
            }

            #[must_use]
            pub fn approximately_equals(self, other: Self, epsilon: f64) -> bool {
                (self.0 - other.0).abs() <= epsilon
            }
        }

        impl std::ops::Add for $quantity_type {
            type Output = Self;

            fn add(self, other: Self) -> Self::Output {
                Self(self.0 + other.0)
            }
        }

        impl std::ops::Sub for $quantity_type {
            type Output = Self;

            fn sub(self, other: Self) -> Self::Output {
                Self(self.0 - other.0)
            }
        }

        impl std::ops::Neg for $quantity_type {
            type Output = Self;

            fn neg(self) -> Self::Output {
                Self(-self.0)
            }
        }

        impl std::ops::Mul<f64> for $quantity_type {
            type Output = Self;

            fn mul(self, scalar: f64) -> Self::Output {
                Self(self.0 * scalar)
            }
        }

        impl std::ops::Mul<$quantity_type> for f64 {
            type Output = $quantity_type;

            fn mul(self, quantity: $quantity_type) -> Self::Output {
                $quantity_type(self * quantity.0)
            }
        }

        impl std::ops::Div<f64> for $quantity_type {
            type Output = Self;

            fn div(self, scalar: f64) -> Self::Output {
                Self(self.0 / scalar)
            }
        }

        impl std::ops::Div<$quantity_type> for $quantity_type {
            type Output = f64;

            fn div(self, other: $quantity_type) -> Self::Output {
                self.0 / other.0
            }
        }
    };
}

pub(crate) use implement_quantity_arithmetic;
