use super::super::*;

#[test]
fn default_display_uses_kilograms() {
    let mass = kilograms(12.5);

    assert_eq!(mass.to_string(), "12.5 kg");
}

#[test]
fn display_can_format_tons_with_precision() {
    let mass = kilograms(1_250.0);

    assert_eq!(
        mass.display_tons_precision(2).to_string(),
        "1.25 t"
    );
}
