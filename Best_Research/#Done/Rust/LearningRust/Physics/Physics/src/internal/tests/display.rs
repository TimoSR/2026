use std::fmt;

use crate::internal::format_unit_value;

struct TestFormattedValue {
    value: f64,
    unit_symbol: &'static str,
    precision: Option<usize>,
}

impl fmt::Display for TestFormattedValue {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        format_unit_value(formatter, self.value, self.unit_symbol, self.precision)
    }
}

#[test]
fn format_unit_value_uses_full_value_without_precision() {
    let value = TestFormattedValue {
        value: 12.345,
        unit_symbol: "m",
        precision: None,
    };

    assert_eq!(value.to_string(), "12.345 m");
}

#[test]
fn format_unit_value_rounds_with_precision() {
    let value = TestFormattedValue {
        value: 12.345,
        unit_symbol: "m",
        precision: Some(2),
    };

    assert_eq!(value.to_string(), "12.35 m");
}
