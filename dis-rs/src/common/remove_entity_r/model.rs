use crate::common::{BodyInfo, Interaction};
use crate::common::model::{EntityId, PduBody};
use crate::enumerations::{PduType, RequiredReliabilityService};
use crate::common::remove_entity_r::builder::RemoveEntityRBuilder;

const REMOVE_ENTITY_R_BODY_LENGTH : u16 = 20;

#[derive(Debug, Default, PartialEq)]
pub struct RemoveEntityR {
    pub originating_id: EntityId,
    pub receiving_id: EntityId,
    pub required_reliability_service: RequiredReliabilityService,
    pub request_id: u32,
}

impl RemoveEntityR {
    pub fn builder() -> RemoveEntityRBuilder {
        RemoveEntityRBuilder::new()
    }

    pub fn into_builder(self) -> RemoveEntityRBuilder {
        RemoveEntityRBuilder::new_from_body(self)
    }

    pub fn into_pdu_body(self) -> PduBody {
        PduBody::RemoveEntityR(self)
    }
}

impl BodyInfo for RemoveEntityR {
    fn body_length(&self) -> u16 {
        REMOVE_ENTITY_R_BODY_LENGTH
    }

    fn body_type(&self) -> PduType {
        PduType::RemoveEntityR
    }
}

impl Interaction for RemoveEntityR {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.originating_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        Some(&self.receiving_id)
    }
}