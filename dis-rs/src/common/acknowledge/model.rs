use crate::{EntityId, PduBody, PduType};
use crate::common::{BodyInfo, Interaction};
use crate::enumerations::{AcknowledgeFlag, ResponseFlag};

const ACKNOWLEDGE_BODY_LENGTH : u16 = 20;

#[derive(Debug, PartialEq)]
pub struct Acknowledge {
    pub originating_id: EntityId,
    pub receiving_id: EntityId,
    pub acknowledge_flag: AcknowledgeFlag,
    pub response_flag: ResponseFlag,
    pub request_id: u32,
}

impl Default for Acknowledge {
    fn default() -> Self {
        Self::new()
    }
}

impl Acknowledge {
    pub fn new() -> Self {
        Self {
            originating_id: Default::default(),
            receiving_id: Default::default(),
            acknowledge_flag: AcknowledgeFlag::default(),
            response_flag: ResponseFlag::default(),
            request_id: 0,
        }
    }

    pub fn with_origination_id(mut self, originating_id: EntityId) -> Self {
        self.originating_id = originating_id;
        self
    }

    pub fn with_receiving_id(mut self, receiving_id: EntityId) -> Self {
        self.receiving_id = receiving_id;
        self
    }

    pub fn with_acknowledge_flag(mut self, acknowledge_flag: AcknowledgeFlag) -> Self {
        self.acknowledge_flag = acknowledge_flag;
        self
    }

    pub fn with_response_flag(mut self, response_flag: ResponseFlag) -> Self {
        self.response_flag = response_flag;
        self
    }

    pub fn with_request_id(mut self, request_id: u32) -> Self {
        self.request_id = request_id;
        self
    }

    pub fn into_pdu_body(self) -> PduBody {
        PduBody::Acknowledge(self)
    }
}

impl BodyInfo for Acknowledge {
    fn body_length(&self) -> u16 {
        ACKNOWLEDGE_BODY_LENGTH
    }

    fn body_type(&self) -> PduType {
        PduType::Acknowledge
    }
}

impl Interaction for Acknowledge {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.originating_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        Some(&self.receiving_id)
    }
}