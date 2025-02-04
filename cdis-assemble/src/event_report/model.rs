use crate::records::model::{CdisRecord, EntityId};
use crate::types::model::{VarInt, UVINT32};
use crate::{BodyProperties, CdisBody, CdisInteraction};
use dis_rs::model::DatumSpecification;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct EventReport {
    pub originating_id: EntityId,
    pub receiving_id: EntityId,
    pub event_type: UVINT32,
    pub datum_specification: DatumSpecification,
}

impl BodyProperties for EventReport {
    type FieldsPresent = EventReportFieldsPresent;
    type FieldsPresentOutput = u8;
    const FIELDS_PRESENT_LENGTH: usize = 2;

    fn fields_present_field(&self) -> Self::FieldsPresentOutput {
        (if !self.datum_specification.fixed_datum_records.is_empty() {
            Self::FieldsPresent::FIXED_DATUMS_BIT
        } else {
            0
        }) | (if !self.datum_specification.variable_datum_records.is_empty() {
            Self::FieldsPresent::VARIABLE_DATUMS_BIT
        } else {
            0
        })
    }

    fn body_length_bits(&self) -> usize {
        Self::FIELDS_PRESENT_LENGTH
            + self.originating_id.record_length()
            + self.receiving_id.record_length()
            + self.event_type.record_length()
            + self.datum_specification.record_length()
    }

    fn into_cdis_body(self) -> CdisBody {
        CdisBody::EventReport(self)
    }
}

impl CdisInteraction for EventReport {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.originating_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        Some(&self.receiving_id)
    }
}

pub struct EventReportFieldsPresent;

impl EventReportFieldsPresent {
    pub const FIXED_DATUMS_BIT: u8 = 0x02;
    pub const VARIABLE_DATUMS_BIT: u8 = 0x01;
}
