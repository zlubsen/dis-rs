use num_traits::FromPrimitive;
use dis_rs::enumerations::VariableRecordType;
use crate::{BodyProperties, CdisBody, CdisInteraction};
use crate::constants::{THIRTY_TWO_BITS, TWENTY_SIX_BITS};
use crate::records::model::{CdisRecord, CdisTimeStamp, EntityId};
use crate::types::model::{UVINT32, UVINT8, VarInt};

#[derive(Clone, Default, Debug, PartialEq)]
pub struct DataQuery {
    pub originating_id: EntityId,
    pub receiving_id: EntityId,
    pub request_id: UVINT32,
    pub time_interval: CdisTimeStamp,
    pub fixed_datum_ids: Vec<VariableRecordType>,
    pub variable_datum_ids: Vec<VariableRecordType>,
}

impl BodyProperties for DataQuery {
    type FieldsPresent = DataQueryFieldsPresent;
    type FieldsPresentOutput = u8;
    const FIELDS_PRESENT_LENGTH: usize = 2;

    fn fields_present_field(&self) -> Self::FieldsPresentOutput {
        (if !self.fixed_datum_ids.is_empty() { Self::FieldsPresent::FIXED_DATUMS_BIT } else { 0 })
        | (if !self.variable_datum_ids.is_empty() { Self::FieldsPresent::VARIABLE_DATUMS_BIT } else { 0 })
    }

    fn body_length_bits(&self) -> usize {
        Self::FIELDS_PRESENT_LENGTH
            + self.originating_id.record_length()
            + self.receiving_id.record_length()
            + self.request_id.record_length()
            + TWENTY_SIX_BITS // CdisTimeStamp
            + UVINT8::from(u8::from_usize(self.fixed_datum_ids.len()).unwrap_or(u8::MAX)).record_length()
            + UVINT8::from(u8::from_usize(self.variable_datum_ids.len()).unwrap_or(u8::MAX)).record_length()
            + self.fixed_datum_ids.len() * THIRTY_TWO_BITS
            + self.variable_datum_ids.len() * THIRTY_TWO_BITS
    }

    fn into_cdis_body(self) -> CdisBody {
        CdisBody::DataQuery(self)
    }
}

impl CdisInteraction for DataQuery {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.originating_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        Some(&self.receiving_id)
    }
}

pub struct DataQueryFieldsPresent;

impl DataQueryFieldsPresent {
    pub const FIXED_DATUMS_BIT: u8 = 0x02;
    pub const VARIABLE_DATUMS_BIT: u8 = 0x01;
}