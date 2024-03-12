use crate::acknowledge_r::builder::AcknowledgeRBuilder;
use crate::common::model::{EntityId, PduBody};
use crate::common::{BodyInfo, Interaction};
use crate::enumerations::{AcknowledgeFlag, ResponseFlag, PduType};

const ACKNOWLEDGE_R_BODY_LENGTH : u16 = 20;

/// 5.12.4.5 Acknowledge-R PDU
///
/// 7.11.6 Acknowledge-R PDU
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AcknowledgeR {
    pub originating_id: EntityId,
    pub receiving_id: EntityId,
    pub acknowledge_flag: AcknowledgeFlag,
    pub response_flag: ResponseFlag,
    pub request_id: u32,
}

impl AcknowledgeR {
    pub fn builder() -> AcknowledgeRBuilder {
        AcknowledgeRBuilder::new()
    }

    pub fn into_builder(self) -> AcknowledgeRBuilder {
        AcknowledgeRBuilder::new_from_body(self)
    }

    pub fn into_pdu_body(self) -> PduBody {
        PduBody::AcknowledgeR(self)
    }
}

impl BodyInfo for AcknowledgeR {
    fn body_length(&self) -> u16 {
        ACKNOWLEDGE_R_BODY_LENGTH
    }

    fn body_type(&self) -> PduType {
        PduType::AcknowledgeR
    }
}

impl Interaction for AcknowledgeR {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.originating_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        Some(&self.receiving_id)
    }
}