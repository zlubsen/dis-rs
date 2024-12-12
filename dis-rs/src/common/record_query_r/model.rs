use crate::common::model::EntityId;
use crate::common::{BodyInfo, Interaction};
use crate::constants::FOUR_OCTETS;
use crate::enumerations::{
    PduType, RecordQueryREventType, RequiredReliabilityService, VariableRecordType,
};
use crate::model::{PduBody, TimeStamp};
use crate::record_query_r::builder::RecordQueryRBuilder;
use serde::{Deserialize, Serialize};

pub const BASE_RECORD_QUERY_R_BODY_LENGTH: u16 = 28;

/// 5.12.4.14 Record Query-R PDU
///
/// 7.11.14 Record Query-R PDU
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RecordQueryR {
    pub originating_id: EntityId,
    pub receiving_id: EntityId,
    pub request_id: u32,
    pub required_reliability_service: RequiredReliabilityService,
    pub event_type: RecordQueryREventType,
    pub time: TimeStamp,
    pub record_query_specification: RecordQuerySpecification,
}

impl RecordQueryR {
    #[must_use]
    pub fn builder() -> RecordQueryRBuilder {
        RecordQueryRBuilder::new()
    }

    #[must_use]
    pub fn into_builder(self) -> RecordQueryRBuilder {
        RecordQueryRBuilder::new_from_body(self)
    }

    #[must_use]
    pub fn into_pdu_body(self) -> PduBody {
        PduBody::RecordQueryR(self)
    }
}

impl BodyInfo for RecordQueryR {
    fn body_length(&self) -> u16 {
        BASE_RECORD_QUERY_R_BODY_LENGTH
            + (self.record_query_specification.record_ids.len() * FOUR_OCTETS) as u16
    }

    fn body_type(&self) -> PduType {
        PduType::RecordQueryR
    }
}

impl Interaction for RecordQueryR {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.originating_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        Some(&self.receiving_id)
    }
}

/// 6.2.72 Record Query Specification record
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RecordQuerySpecification {
    pub record_ids: Vec<VariableRecordType>,
}

impl RecordQuerySpecification {
    #[must_use]
    pub fn with_record_id(mut self, record_id: VariableRecordType) -> Self {
        self.record_ids.push(record_id);
        self
    }

    #[must_use]
    pub fn with_record_ids(mut self, record_ids: Vec<VariableRecordType>) -> Self {
        self.record_ids = record_ids;
        self
    }
}
