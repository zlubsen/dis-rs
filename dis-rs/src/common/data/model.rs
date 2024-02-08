use crate::common::{BodyInfo, Interaction};
use crate::common::data::builder::DataBuilder;
use crate::common::model::{EntityId, FixedDatum, VariableDatum, BASE_VARIABLE_DATUM_LENGTH, FIXED_DATUM_LENGTH, length_padded_to_num, PduBody};
use crate::enumerations::PduType;
use crate::constants::EIGHT_OCTETS;

pub const BASE_DATA_BODY_LENGTH: u16 = 28;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Data {
    pub originating_id: EntityId,
    pub receiving_id: EntityId,
    pub request_id: u32,
    pub fixed_datum_records: Vec<FixedDatum>,
    pub variable_datum_records: Vec<VariableDatum>,
}

impl Data {
    pub fn builder() -> DataBuilder {
        DataBuilder::new()
    }

    pub fn into_builder(self) -> DataBuilder {
        DataBuilder::new_from_body(self)
    }

    pub fn into_pdu_body(self) -> PduBody {
        PduBody::Data(self)
    }
}

impl BodyInfo for Data {
    fn body_length(&self) -> u16 {
        BASE_DATA_BODY_LENGTH +
            (FIXED_DATUM_LENGTH * self.fixed_datum_records.len() as u16) +
            (self.variable_datum_records.iter().map(|datum| {
                let padded_record = length_padded_to_num(
                    BASE_VARIABLE_DATUM_LENGTH as usize + datum.datum_value.len(),
                    EIGHT_OCTETS);
                padded_record.record_length as u16
            } ).sum::<u16>())
    }

    fn body_type(&self) -> PduType {
        PduType::Data
    }
}

impl Interaction for Data {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.originating_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        Some(&self.receiving_id)
    }
}
