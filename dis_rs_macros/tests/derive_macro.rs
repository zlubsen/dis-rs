use dis_rs_macros::PduField;

#[derive(PduField)]
#[repr(u8)]
pub enum ForceId {
    Other = 0,
    Friendly = 1,
    Opposing = 2,
    Neutral = 3,
}

#[test]
fn from_wire() {
    let wire_format : u8 = 1;
    let force_id = ForceId::from(wire_format);
    assert_eq!(force_id, ForceId::Friendly);
}
