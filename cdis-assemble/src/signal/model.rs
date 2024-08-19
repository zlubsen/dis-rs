use dis_rs::enumerations::{SignalEncodingClass, SignalTdlType};
use dis_rs::signal::model::EncodingScheme;
use crate::{BodyProperties, CdisBody, CdisInteraction};
use crate::constants::EIGHT_BITS;
use crate::records::model::{CdisRecord, EntityId};
use crate::types::model::{VarInt, UVINT16, UVINT32, UVINT8};

#[derive(Clone, Default, Debug, PartialEq)]
pub struct Signal {
    pub radio_reference_id: EntityId,
    pub radio_number: UVINT16,
    pub encoding_scheme_class: SignalEncodingClass,
    pub encoding_scheme_type: UVINT8,
    pub tdl_type: SignalTdlType,
    pub sample_rate: Option<UVINT32>,
    pub samples: Option<UVINT16>,
    pub data: Vec<u8>,
}

impl BodyProperties for Signal {
    type FieldsPresent = SignalFieldsPresent;
    type FieldsPresentOutput = u8;
    const FIELDS_PRESENT_LENGTH: usize = 2;

    fn fields_present_field(&self) -> Self::FieldsPresentOutput {
        (if self.sample_rate.is_some() { Self::FieldsPresent::SAMPLE_RATE_BIT } else { 0 })
        | (if self.samples.is_some() { Self::FieldsPresent::SAMPLES_BIT} else { 0 })
    }

    fn body_length_bits(&self) -> usize {
        const CONST_BIT_SIZE: usize = 24; // encoding scheme class, tdl type, data length
        Self::FIELDS_PRESENT_LENGTH + CONST_BIT_SIZE
        + self.radio_reference_id.record_length()
        + self.radio_number.record_length()
        + self.encoding_scheme_type.record_length()
        + if let Some(record) = self.sample_rate { record.record_length() } else { 0 }
        + if let Some(record) = self.samples { record.record_length() } else { 0 }
        + (self.data.len() * EIGHT_BITS)
    }

    fn into_cdis_body(self) -> CdisBody {
        CdisBody::Signal(self)
    }
}

impl CdisInteraction for Signal {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.radio_reference_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        None
    }
}

pub struct SignalFieldsPresent;

impl SignalFieldsPresent {
    pub const SAMPLE_RATE_BIT: u8 = 0x02;
    pub const SAMPLES_BIT: u8 = 0x01;
}