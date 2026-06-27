use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum QuantityError
{
    NonFinite
    {
        quantity: &'static str, unit: &'static str, value: f64
    },
    DivisionByZero
    {
        operation: &'static str
    },
}

impl fmt::Display for QuantityError
{
    fn fmt<'formatter>(&self, formatter: &mut fmt::Formatter <'formatter>) -> fmt::Result
    {
        match self
        {
            QuantityError::NonFinite { quantity, unit, value } => write!(formatter, "{quantity} value must be finite, got {value} {unit}"),
            QuantityError::DivisionByZero { operation } =>
            {
                write!(formatter, "division by zero in {operation}")
            }
        }
    }
}

impl std::error::Error for QuantityError
{
}

pub(crate) fn validate_finite(quantity: &'static str, unit: &'static str, value: f64) -> Result<f64, QuantityError>
{
    if value.is_finite()
    {
        return Ok(value);
    }

    Err(QuantityError::NonFinite { quantity, unit, value })
}

pub(crate) fn check_nonzero(value: f64, operation: &'static str) -> Result<(), QuantityError>
{
    if value != 0.0
    {
        return Ok(());
    }

    Err(QuantityError::DivisionByZero { operation })
}
