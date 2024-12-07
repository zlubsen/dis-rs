use crate::common::model::{EntityId, RecordSpecification, BASE_RECORD_SPEC_RECORD_LENGTH};
use crate::common::{BodyInfo, Interaction};
use crate::constants::EIGHT_OCTETS;
use crate::enumerations::{EventType, PduType, RequiredReliabilityService};
use crate::model::{length_padded_to_num, PduBody};
use crate::record_r::builder::RecordRBuilder;

pub const BASE_RECORD_R_BODY_LENGTH: u16 = 28;

/// 5.12.4.16 Record-R PDU
///
/// 7.11.16 Record-R PDU
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RecordR {
    pub originating_id: EntityId,
    pub receiving_id: EntityId,
    pub request_id: u32,
    pub required_reliability_service: RequiredReliabilityService,
    pub event_type: EventType,
    pub response_serial_number: u32,
    pub record_specification: RecordSpecification,
}

impl RecordR {
    #[must_use]
    pub fn builder() -> RecordRBuilder {
        RecordRBuilder::new()
    }

    #[must_use]
    pub fn into_builder(self) -> RecordRBuilder {
        RecordRBuilder::new_from_body(self)
    }

    #[must_use]
    pub fn into_pdu_body(self) -> PduBody {
        PduBody::RecordR(self)
    }
}

impl BodyInfo for RecordR {
    fn body_length(&self) -> u16 {
        BASE_RECORD_R_BODY_LENGTH
            + (self
                .record_specification
                .record_sets
                .iter()
                .map(|record| {
                    let data_length_bytes = record
                        .records
                        .iter()
                        .map(|rec| rec.len() as u16)
                        .sum::<u16>();
                    let padded_record =
                        length_padded_to_num(data_length_bytes.into(), EIGHT_OCTETS);
                    BASE_RECORD_SPEC_RECORD_LENGTH + padded_record.record_length as u16
                })
                .sum::<u16>())
    }

    fn body_type(&self) -> PduType {
        PduType::RecordR
    }
}

impl Interaction for RecordR {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.originating_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        Some(&self.receiving_id)
    }
}
