use crate::dis::v6::model::{PduHeader, PduType, ProtocolFamily, ProtocolVersion};

pub struct PduHeaderBuilder {
    pub protocol_version : Option<ProtocolVersion>,
    pub exercise_id : Option<u8>,
    pub pdu_type : Option<PduType>,
    pub protocol_family : Option<ProtocolFamily>,
    pub time_stamp : Option<u32>,
    pub pdu_length : Option<u16>,
    pub padding : u16,
}

impl PduHeaderBuilder {
    pub fn new() -> PduHeaderBuilder {
        PduHeaderBuilder {
            protocol_version : None,
            exercise_id : None,
            pdu_type : None,
            protocol_family : None,
            time_stamp : None,
            pdu_length : None,
            padding : 0u16,
        }
    }

    pub fn build(self) -> PduHeader {
        PduHeader {
            protocol_version: self.protocol_version.expect("Value expected, but not found."),
            exercise_id: self.exercise_id.expect("Value expected, but not found."),
            pdu_type: self.pdu_type.expect("Value expected, but not found."),
            protocol_family: self.protocol_family.expect("Value expected, but not found."),
            time_stamp: self.time_stamp.expect("Value expected, but not found."),
            pdu_length: self.pdu_length.expect("Value expected, but not found."),
            padding: self.padding,
        }
    }
}