#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ProtocolVersion {
    Other = 0,
    Version1_0May92 = 1,                // DIS PDU version 1.0 (May 92)
    Ieee1278_1993 = 2,                  // IEEE 1278-1993
    Version2_0ThirdDraft = 3,           // DIS PDU version 2.0 - third draft (May 93)
    Version2_0FourthDraft = 4,          // DIS PDU version 2.0 - fourth draft (revised) March 16, 1994
    Ieee1278_1_1995 = 5,                // IEEE 1278.1-1995 / DIS 5
    Ieee1278_1a1998 = 6,                // IEEE 1278.1a-1998 / DIS 6
    Ieee1278_1_2012 = 7,               // IEEE 1278.1-2012 / DIS 7
}

impl From<u8> for ProtocolVersion {
    fn from(value: u8) -> Self {
        match value {
            1 => ProtocolVersion::Version1_0May92,
            2 => ProtocolVersion::Ieee1278_1993,
            3 => ProtocolVersion::Version2_0ThirdDraft,
            4 => ProtocolVersion::Version2_0FourthDraft,
            5 => ProtocolVersion::Ieee1278_1_1995,
            6 => ProtocolVersion::Ieee1278_1a1998,
            7 => ProtocolVersion::Ieee1278_1_2012,
            0 | _ => ProtocolVersion::Other,
        }
    }
}