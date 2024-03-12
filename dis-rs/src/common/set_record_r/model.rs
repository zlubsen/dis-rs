use crate::common::{BodyInfo, Interaction};
use crate::common::model::EntityId;
use crate::constants::EIGHT_OCTETS;
use crate::enumerations::{PduType, RequiredReliabilityService};
use crate::model::{BASE_RECORD_SPEC_RECORD_LENGTH, length_padded_to_num, PduBody, RecordSpecification};
use crate::set_record_r::builder::SetRecordRBuilder;

pub const BASE_RECORD_R_BODY_LENGTH: u16 = 28;

/// 5.12.4.15 Set Record-R PDU
///
/// 7.11.15 Set Record-R PDU
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SetRecordR {
    pub originating_id: EntityId,
    pub receiving_id: EntityId,
    pub request_id: u32,
    pub required_reliability_service: RequiredReliabilityService,
    pub record_specification: RecordSpecification,
}

impl SetRecordR {
    pub fn builder() -> SetRecordRBuilder {
        SetRecordRBuilder::new()
    }

    pub fn into_builder(self) -> SetRecordRBuilder {
        SetRecordRBuilder::new_from_body(self)
    }

    pub fn into_pdu_body(self) -> PduBody {
        PduBody::SetRecordR(self)
    }
}

impl BodyInfo for SetRecordR {
    fn body_length(&self) -> u16 {
        BASE_RECORD_R_BODY_LENGTH +
            (self.record_specification.record_sets.iter()
                .map(|record| {
                    let data_length_bytes = record.records.iter()
                        .map(|rec| rec.len() as u16 )
                        .sum::<u16>();
                    let padded_record = length_padded_to_num(data_length_bytes.into(), EIGHT_OCTETS);
                    BASE_RECORD_SPEC_RECORD_LENGTH + padded_record.record_length as u16
                })
                .sum::<u16>())
    }

    fn body_type(&self) -> PduType {
        PduType::SetRecordR
    }
}

impl Interaction for SetRecordR {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.originating_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        Some(&self.receiving_id)
    }
}