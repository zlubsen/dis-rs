use dis_rs_macros::PduConversion;

#[derive(PduConversion, PartialEq, Debug, Default)]
#[repr(u8)]
pub enum FieldU8 {
    #[default]
    FieldOne = 0,
    FieldTwo = 1,
}

#[derive(PduConversion, PartialEq, Debug, Default)]
#[repr(u16)]
pub enum FieldU16 {
    #[default]
    One = 0,
    Two = 1,
    Three = 2,
}

#[test]
fn field_u8_from_derive_test() {
    let wire_input: u8 = 0;
    let field = FieldU8::from(wire_input);
    assert_eq!(field, FieldU8::FieldOne);
}

#[test]
fn field_u8_default_derive_test() {
    let wire_input: u8 = 5;
    let field = FieldU8::from(wire_input);
    assert_eq!(field, FieldU8::FieldOne);
}

#[test]
fn field_u16_from_derive_test() {
    let wire_input: u16 = 0;
    let field = FieldU16::from(wire_input);
    assert_eq!(field, FieldU16::One);
}
