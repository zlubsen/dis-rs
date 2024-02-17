use crate::common::{BodyInfo, Interaction};
use crate::common::model::EntityId;
use crate::constants::EIGHT_OCTETS;
use crate::enumerations::{EventType, PduType, RequiredReliabilityService, VariableRecordType};
use crate::model::{length_padded_to_num, PduBody};
use crate::record_r::builder::RecordRBuilder;

pub const BASE_RECORD_R_BODY_LENGTH: u16 = 28;
pub const BASE_RECORD_SPEC_LENGTH: u16 = 16;

/// 5.12.4.16 Record-R PDU
///
/// 7.11.16 Record-R PDU
#[derive(Debug, Default, PartialEq)]
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
    pub fn builder() -> RecordRBuilder {
        RecordRBuilder::new()
    }

    pub fn into_builder(self) -> RecordRBuilder {
        RecordRBuilder::new_from_body(self)
    }

    pub fn into_pdu_body(self) -> PduBody {
        PduBody::RecordR(self)
    }
}

impl BodyInfo for RecordR {
    fn body_length(&self) -> u16 {
        BASE_RECORD_R_BODY_LENGTH +
            (self.record_specification.record_sets.iter()
                .map(|record| {
                    let data_length_bytes = record.records.iter()
                        .map(|rec| rec.len() as u16 )
                        .sum::<u16>();
                    let padded_record = length_padded_to_num(data_length_bytes.into(), EIGHT_OCTETS);
                    BASE_RECORD_SPEC_LENGTH + padded_record.record_length as u16
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

/// 6.2.73 Record Specification record
#[derive(Debug, Default, PartialEq)]
pub struct RecordSpecification {
    pub record_sets: Vec<RecordSet>,
}

impl RecordSpecification {
    pub fn with_record_set(mut self, record: RecordSet) -> Self {
        self.record_sets.push(record);
        self
    }

    pub fn with_record_sets(mut self, records: Vec<RecordSet>) -> Self {
        self.record_sets = records;
        self
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct RecordSet {
    pub record_id: VariableRecordType,
    pub record_serial_number: u32,
    pub record_length_bytes: u16,
    pub records: Vec<Vec<u8>>,
}

impl RecordSet {
    pub fn with_record_id(mut self, record_id: VariableRecordType) -> Self {
        self.record_id = record_id;
        self
    }

    pub fn with_record_serial_number(mut self, record_serial_number: u32) -> Self {
        self.record_serial_number = record_serial_number;
        self
    }

    /// Adds `record` to be the Record Values in this RecordSet.
    /// It is specified in the DIS standard that all Record Values in a RecordSet are of the same length.
    /// It is up to the caller of the function to ensure only Record Values of same length are added,
    /// the length of the last added value is assumed for all previously added.
    pub fn with_record(mut self, record: Vec<u8>) -> Self {
        self.record_length_bytes = record.len() as u16;
        self.records.push(record);
        self
    }

    /// Sets `records` to be the records in this RecordSet.
    /// It is specified in the DIS standard that all Record Values in a RecordSet are of the same length (i.e., the inner `Vec`).
    pub fn with_records(mut self, records: Vec<Vec<u8>>) -> Self {
        self.record_length_bytes = if let Some(record) = records.first() {
            record.len()
        } else { 0 } as u16;
        self.records = records;
        self
    }
}