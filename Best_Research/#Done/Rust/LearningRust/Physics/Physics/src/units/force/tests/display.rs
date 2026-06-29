use super::super::*;

#[test]
fn default_display_uses_newtons() {
    let force = newtons(12.5);

    assert_eq!(force.to_string(), "12.5 N");
}

#[test]
fn display_can_format_kilonewtons_with_precision() {
    let force = newtons(1_250.0);

    assert_eq!(force.display_kilonewtons_precision(2).to_string(), "1.25 kN");
}
