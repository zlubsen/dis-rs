use crate::common::model::{PduHeader, ProtocolVersion};
use crate::v7::model::PduStatus;
use crate::enumerations::{PduType, ProtocolFamily};

#[allow(dead_code)]
pub struct PduHeaderBuilder {
    pub protocol_version : Option<ProtocolVersion>,
    pub exercise_id : Option<u8>,
    pub pdu_type : Option<PduType>,
    pub protocol_family : Option<ProtocolFamily>,
    pub time_stamp : Option<u32>,
    pub pdu_length : Option<u16>,
    pub pdu_status : Option<PduStatus>,
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
            pdu_status: None,
            padding : 0u16,
        }
    }

    pub fn protocol_version(mut self, version: ProtocolVersion) -> Self {
        self.protocol_version = Some(version);
        self
    }

    pub fn exercise_id(mut self, id: u8) -> Self {
        self.exercise_id = Some(id);
        self
    }

    pub fn pdu_type(mut self, pdu_type: PduType) -> Self {
        self.pdu_type = Some(pdu_type);
        self
    }

    pub fn protocol_family(mut self, family: ProtocolFamily) -> Self {
        self.protocol_family = Some(family);
        self
    }

    pub fn time_stamp(mut self, time: u32) -> Self {
        self.time_stamp = Some(time);
        self
    }

    pub fn pdu_length(mut self, length: u16) -> Self {
        self.pdu_length = Some(length);
        self
    }

    pub fn pdu_status(mut self, status: PduStatus) -> Self {
        self.pdu_status = Some(status);
        self
    }

    #[allow(dead_code)]
    pub fn padding(mut self, padding: u16) -> Self {
        self.padding = padding;
        self
    }

    fn validate(&self) -> Result<(), ()> {
        if self.protocol_version.is_some() &&
            self.exercise_id.is_some() &&
            self.pdu_type.is_some() &&
            self.protocol_family.is_some() &&
            self.time_stamp.is_some() &&
            self.pdu_length.is_some() &&
            self.validate_v7_has_status()
            {
            Ok(())
        } else { Err(()) }
    }

    fn validate_v7_has_status(&self) -> bool {
        if let Some(version) = self.protocol_version {
            if version == ProtocolVersion::Ieee1278_1_2012 {
                return self.pdu_status.is_some()
            }
        }
        true
    }

    pub fn build(&self) -> Result<PduHeader, ()> {
        if self.validate().is_err() {
            return Err(())
        }
        Ok(PduHeader {
            protocol_version: self.protocol_version.expect("Value expected, but not found."),
            exercise_id: self.exercise_id.expect("Value expected, but not found."),
            pdu_type: self.pdu_type.expect("Value expected, but not found."),
            protocol_family: self.protocol_family.expect("Value expected, but not found."),
            time_stamp: self.time_stamp.expect("Value expected, but not found."),
            pdu_length: self.pdu_length.expect("Value expected, but not found."),
            pdu_status: self.pdu_status,
            padding: self.padding,
        })
    }
}