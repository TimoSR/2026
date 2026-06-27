use super::super::*;

#[test]
fn constructors_convert_to_seconds() {
    assert_eq!(seconds(12.0).as_seconds(), 12.0);
    assert_eq!(milliseconds(1_200.0).as_seconds(), 1.2);
    assert_eq!(minutes(2.0).as_seconds(), 120.0);
    assert_eq!(hours(2.0).as_seconds(), 7_200.0);
}

#[test]
fn fallible_constructor_rejects_non_finite_values() {
    assert!(Time::try_seconds(f64::NAN).is_err());
}

#[test]
fn unit_symbols_match_time_units() {
    assert_eq!(TimeUnit::Seconds.symbol(), "s");
    assert_eq!(TimeUnit::Milliseconds.symbol(), "ms");
    assert_eq!(TimeUnit::Microseconds.symbol(), "us");
    assert_eq!(TimeUnit::Nanoseconds.symbol(), "ns");
    assert_eq!(TimeUnit::Minutes.symbol(), "min");
    assert_eq!(TimeUnit::Hours.symbol(), "h");
}
