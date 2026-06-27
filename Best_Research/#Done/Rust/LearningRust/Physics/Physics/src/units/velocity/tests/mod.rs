mod display;
mod operators;
mod quantity;
mod root_function;

fn assert_close(left: f64, right: f64)
{
    assert!((left - right).abs() <= 0.000001, "{left} != {right}");
}
