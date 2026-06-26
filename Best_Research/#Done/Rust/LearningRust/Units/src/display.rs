use std::fmt;

pub(crate) fn format_unit_value(
    formatter: &mut fmt::Formatter<'_>,
    value: f64,
    unit_symbol: &'static str,
    precision: Option<usize>,
) -> fmt::Result {
    match precision {
        Some(precision) => write!(formatter, "{value:.precision$} {unit_symbol}"),
        None => write!(formatter, "{value} {unit_symbol}"),
    }
}
