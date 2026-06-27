use super::super::*;

#[test]
fn constructors_convert_to_newtons() {
    assert_eq!(newtons(12.0).to_newtons(), 12.0);
    assert_eq!(millinewtons(1_200.0).to_newtons(), 1.2);
    assert_eq!(kilonewtons(2.5).to_newtons(), 2_500.0);
}

#[test]
fn unit_symbols_match_force_units() {
    assert_eq!(ForceUnit::Newtons.symbol(), "N");
    assert_eq!(ForceUnit::Millinewtons.symbol(), "mN");
    assert_eq!(ForceUnit::Kilonewtons.symbol(), "kN");
}
