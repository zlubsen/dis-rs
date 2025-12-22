use crate::common::model::{EntityId, PduBody};
use crate::common::{BodyInfo, Interaction};
use crate::enumerations::{PduType, ReceiverState};
use crate::receiver::builder::ReceiverBuilder;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

const RECEIVER_BODY_LENGTH: u16 = 24;

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Receiver {
    pub radio_reference_id: EntityId,
    pub radio_number: u16,
    pub receiver_state: ReceiverState,
    pub received_power: f32,
    pub transmitter_radio_reference_id: EntityId,
    pub transmitter_radio_number: u16,
}

impl Receiver {
    #[must_use]
    pub fn builder() -> ReceiverBuilder {
        ReceiverBuilder::new()
    }

    #[must_use]
    pub fn into_builder(self) -> ReceiverBuilder {
        ReceiverBuilder::new_from_body(self)
    }

    #[must_use]
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

impl From<Receiver> for PduBody {
    #[inline]
    fn from(value: Receiver) -> Self {
        value.into_pdu_body()
    }
}
