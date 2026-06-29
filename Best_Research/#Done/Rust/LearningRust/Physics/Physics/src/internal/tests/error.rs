use crate::internal::QuantityError;
use crate::internal::check_nonzero;
use crate::internal::validate_finite;

#[test]
fn validate_finite_accepts_finite_values() {
    assert_eq!(validate_finite("Length", "m", 12.5), Ok(12.5));
}

#[test]
fn validate_finite_rejects_infinity() {
    let error = validate_finite("Length", "m", f64::INFINITY).unwrap_err();

    assert_eq!(
        error,
        QuantityError::NonFinite {
            quantity: "Length",
            unit: "m",
            value: f64::INFINITY,
        }
    );
    assert_eq!(error.to_string(), "Length value must be finite, got inf m");
}

#[test]
fn check_nonzero_accepts_nonzero_values() {
    assert_eq!(check_nonzero(12.5, "Length / Time"), Ok(()));
}

#[test]
fn check_nonzero_rejects_zero() {
    let error = check_nonzero(0.0, "Length / Time").unwrap_err();

    assert_eq!(error, QuantityError::DivisionByZero { operation: "Length / Time" });
    assert_eq!(error.to_string(), "division by zero in Length / Time");
}
