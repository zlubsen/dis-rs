use crate::constants::TWO_BITS;
use crate::records::model::{CdisRecord, EntityId};
use crate::types::model::{UVINT32, VarInt};
use crate::{BodyProperties, CdisBody, CdisInteraction};
use dis_rs::model::DatumSpecification;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct ActionRequest {
    pub originating_id: EntityId,
    pub receiving_id: EntityId,
    pub request_id: UVINT32,
    pub action_id: UVINT32,
    pub datum_specification: DatumSpecification,
}

impl BodyProperties for ActionRequest {
    type FieldsPresentFlags = ActionRequestFieldsPresent;
    type FieldsPresentField = u8;
    const FIELDS_PRESENT_FLAGS_LENGTH: usize = TWO_BITS;

    fn fields_present_field(&self) -> Self::FieldsPresentField {
        (if self.datum_specification.fixed_datum_records.is_empty() {
            0
        } else {
            Self::FieldsPresentFlags::FIXED_DATUMS_BIT
        }) | (if self.datum_specification.variable_datum_records.is_empty() {
            0
        } else {
            Self::FieldsPresentFlags::VARIABLE_DATUMS_BIT
        })
    }

    fn body_length_bits(&self) -> usize {
        Self::FIELDS_PRESENT_FLAGS_LENGTH
            + self.originating_id.record_length()
            + self.receiving_id.record_length()
            + self.request_id.record_length()
            + self.action_id.record_length()
            + self.datum_specification.record_length()
    }

    fn into_cdis_body(self) -> CdisBody {
        CdisBody::ActionRequest(self)
    }
}

impl CdisInteraction for ActionRequest {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.originating_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        Some(&self.receiving_id)
    }
}

pub struct ActionRequestFieldsPresent;

impl ActionRequestFieldsPresent {
    pub const FIXED_DATUMS_BIT: u8 = 0x02;
    pub const VARIABLE_DATUMS_BIT: u8 = 0x01;
}
