#[derive(Copy, Clone)]
pub enum ProtocolVersion {
    Other = 0,
    VERSION_1_0_MAY_92 = 1,             // DIS PDU version 1.0 (May 92)
    IEEE_1278_1993 = 2,                 // IEEE 1278-1993
    VERSION_2_0_THIRD_DRAFT = 3,        // DIS PDU version 2.0 - third draft (May 93)
    VERSION_2_0_FOURTH_DRAFT = 4,       // DIS PDU version 2.0 - fourth draft (revised) March 16, 1994
    IEEE_1278_1_1995 = 5,               // IEEE 1278.1-1995 / DIS 6
    IEEE_1278_1_2012 = 6,               // IEEE 1278.1-2012 / DIS 7
}

impl From<u8> for ProtocolVersion {
    fn from(value: u8) -> Self {
        match value {
            1 => ProtocolVersion::VERSION_1_0_MAY_92,
            2 => ProtocolVersion::IEEE_1278_1993,
            3 => ProtocolVersion::VERSION_2_0_THIRD_DRAFT,
            4 => ProtocolVersion::VERSION_2_0_FOURTH_DRAFT,
            5 => ProtocolVersion::IEEE_1278_1_1995,
            6 => ProtocolVersion::IEEE_1278_1_2012,
            0 | _ => ProtocolVersion::Other,
        }
    }
}