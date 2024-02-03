use crate::enumerations::ReceiverState;
use crate::model::EntityId;
use crate::receiver::model::Receiver;

pub struct ReceiverBuilder(Receiver);

impl ReceiverBuilder {
    pub fn new() -> Self {
        ReceiverBuilder(Receiver::default())
    }

    pub fn new_from_body(body: Receiver) -> Self {
        ReceiverBuilder(body)
    }

    pub fn build(self) -> Receiver {
        self.0
    }

    pub fn with_radio_reference_id(mut self, radio_reference_id: EntityId) -> Self {
        self.0.radio_reference_id = radio_reference_id;
        self
    }

    pub fn with_radio_number(mut self, radio_number: u16) -> Self {
        self.0.radio_number = radio_number;
        self
    }

    pub fn with_receiver_state(mut self, receiver_state: ReceiverState) -> Self {
        self.0.receiver_state = receiver_state;
        self
    }

    pub fn with_received_power(mut self, received_power: f32) -> Self {
        self.0.received_power = received_power;
        self
    }

    pub fn with_transmitter_radio_reference_id(mut self, transmitter_radio_reference_id: EntityId) -> Self {
        self.0.transmitter_radio_reference_id = transmitter_radio_reference_id;
        self
    }

    pub fn with_transmitter_radio_number(mut self, transmitter_radio_number: u16) -> Self {
        self.0.transmitter_radio_number = transmitter_radio_number;
        self
    }
}
