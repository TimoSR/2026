use crate::internal::implement_quantity_arithmetic;

#[derive(Copy, Clone, PartialEq, Debug)]
struct TestQuantity(pub(crate) f64);

implement_quantity_arithmetic!(TestQuantity);

#[test]
fn macro_adds_zero_and_raw_si() {
    assert_eq!(TestQuantity::ZERO, TestQuantity(0.0));
    assert_eq!(TestQuantity(12.5).raw_si(), 12.5);
}

#[test]
fn macro_adds_same_dimension_arithmetic() {
    assert_eq!(TestQuantity(2.0) + TestQuantity(3.0), TestQuantity(5.0));
    assert_eq!(TestQuantity(5.0) - TestQuantity(3.0), TestQuantity(2.0));
    assert_eq!(-TestQuantity(3.0), TestQuantity(-3.0));
}

#[test]
fn macro_adds_scalar_arithmetic() {
    assert_eq!(TestQuantity(3.0) * 2.0, TestQuantity(6.0));
    assert_eq!(2.0 * TestQuantity(3.0), TestQuantity(6.0));
    assert_eq!(TestQuantity(6.0) / 2.0, TestQuantity(3.0));
}

#[test]
fn macro_adds_ratio_and_approximate_comparison() {
    assert_eq!(TestQuantity(6.0) / TestQuantity(2.0), 3.0);
    assert!(TestQuantity(6.0).approximately_equals(TestQuantity(6.0000001), 0.000001));
    assert!(!TestQuantity(6.0).approximately_equals(TestQuantity(6.1), 0.000001));
}
