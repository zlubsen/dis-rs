use dis_rs::enumerations::ReceiverState;
use crate::{BodyProperties, CdisBody, CdisInteraction};
use crate::records::model::{CdisRecord, EntityId};
use crate::types::model::{VarInt, UVINT16};

#[derive(Clone, Default, Debug, PartialEq)]
pub struct Receiver {
    pub radio_reference_id: EntityId,
    pub radio_number: UVINT16,
    pub receiver_state: ReceiverState,
    pub received_power: i16,
    pub transmitter_radio_reference_id: EntityId,
    pub transmitter_radio_number: UVINT16,
}

impl BodyProperties for Receiver {
    type FieldsPresent = ();
    type FieldsPresentOutput = ();
    const FIELDS_PRESENT_LENGTH: usize = 0;

    fn fields_present_field(&self) -> Self::FieldsPresentOutput {
        ()
    }

    fn body_length_bits(&self) -> usize {
        const CONST_BIT_SIZE: usize = 11; // receiver state and power.
        CONST_BIT_SIZE +
            self.radio_reference_id.record_length() +
            self.radio_number.record_length() +
            self.transmitter_radio_reference_id.record_length() +
            self.transmitter_radio_number.record_length()
    }

    fn into_cdis_body(self) -> CdisBody {
        CdisBody::Receiver(self)
    }
}

impl CdisInteraction for Receiver {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.radio_reference_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        Some(&self.transmitter_radio_reference_id)
    }
}