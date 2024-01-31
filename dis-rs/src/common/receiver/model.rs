use crate::common::{BodyInfo, Interaction};
use crate::common::model::{EntityId, PduBody};
use crate::enumerations::{PduType, ReceiverState};

const RECEIVER_BODY_LENGTH: u16 = 24;

#[derive(Debug, PartialEq)]
pub struct Receiver {
    pub radio_reference_id: EntityId,
    pub radio_number: u16,
    pub receiver_state: ReceiverState,
    pub received_power: f32,
    pub transmitter_radio_reference_id: EntityId,
    pub transmitter_radio_number: u16,
}

impl Default for Receiver {
    fn default() -> Self {
        Self::new()
    }
}

impl Receiver {
    pub fn new() -> Self {
        Self {
            radio_reference_id: Default::default(),
            radio_number: 0,
            receiver_state: Default::default(),
            received_power: 0.0,
            transmitter_radio_reference_id: Default::default(),
            transmitter_radio_number: 0,
        }
    }

    pub fn with_radio_reference_id(mut self, radio_reference_id: EntityId) -> Self {
        self.radio_reference_id = radio_reference_id;
        self
    }

    pub fn with_radio_number(mut self, radio_number: u16) -> Self {
        self.radio_number = radio_number;
        self
    }

    pub fn with_receiver_state(mut self, receiver_state: ReceiverState) -> Self {
        self.receiver_state = receiver_state;
        self
    }

    pub fn with_received_power(mut self, received_power: f32) -> Self {
        self.received_power = received_power;
        self
    }

    pub fn with_transmitter_radio_reference_id(mut self, transmitter_radio_reference_id: EntityId) -> Self {
        self.transmitter_radio_reference_id = transmitter_radio_reference_id;
        self
    }

    pub fn with_transmitter_radio_number(mut self, transmitter_radio_number: u16) -> Self {
        self.transmitter_radio_number = transmitter_radio_number;
        self
    }

    pub fn into_pdu_body(self) -> PduBody {
        PduBody::Receiver(self)
    }
}

impl BodyInfo for Receiver {
    fn body_length(&self) -> u16 {
        RECEIVER_BODY_LENGTH
    }

    fn body_type(&self) -> PduType {
        PduType::Receiver
    }
}

impl Interaction for Receiver {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.transmitter_radio_reference_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        Some(&self.radio_reference_id)
    }
}
