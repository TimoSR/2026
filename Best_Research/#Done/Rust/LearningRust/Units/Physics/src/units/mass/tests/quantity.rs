use super::super::*;

#[test]
fn constructors_convert_to_kilograms() {
    assert_eq!(kilograms(12.0).to_kilograms(), 12.0);
    assert_eq!(kilogram(12.0).to_kilograms(), 12.0);
    assert_eq!(grams(1_200.0).to_kilograms(), 1.2);
    assert_eq!(tons(2.5).to_kilograms(), 2_500.0);
}

#[test]
fn fallible_constructor_rejects_non_finite_values() {
    assert!(Mass::try_kilograms(f64::INFINITY).is_err());
}

#[test]
fn unit_symbols_match_mass_units() {
    assert_eq!(MassUnit::Kilograms.symbol(), "kg");
    assert_eq!(MassUnit::Grams.symbol(), "g");
    assert_eq!(MassUnit::Milligrams.symbol(), "mg");
    assert_eq!(MassUnit::Micrograms.symbol(), "ug");
    assert_eq!(MassUnit::Tons.symbol(), "t");
}
