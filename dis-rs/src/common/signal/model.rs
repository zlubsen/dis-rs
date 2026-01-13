use crate::common::model::{length_padded_to_num, EntityId, PduBody};
use crate::common::{BodyInfo, Interaction};
use crate::constants::FOUR_OCTETS;
use crate::enumerations::{
    PduType, SignalEncodingClass, SignalEncodingType, SignalTdlType,
    SignalUserProtocolIdentificationNumber,
};
use crate::signal::builder::SignalBuilder;
use crate::BodyRaw;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub const BASE_SIGNAL_BODY_LENGTH: u16 = 20;

/// 5.8.4 Signal PDU
///
/// 7.7.3 Signal PDU
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Signal {
    pub radio_reference_id: EntityId,
    pub radio_number: u16,
    pub encoding_scheme: EncodingScheme,
    pub tdl_type: SignalTdlType,
    pub sample_rate: u32,
    pub samples: u16,
    pub data: Vec<u8>,
}

impl BodyRaw for Signal {
    type Builder = SignalBuilder;

    fn builder() -> Self::Builder {
        Self::Builder::new()
    }

    fn into_builder(self) -> Self::Builder {
        Self::Builder::new_from_body(self)
    }

    fn into_pdu_body(self) -> PduBody {
        PduBody::Signal(self)
    }
}

impl BodyInfo for Signal {
    fn body_length(&self) -> u16 {
        BASE_SIGNAL_BODY_LENGTH
            + length_padded_to_num(self.data.len(), FOUR_OCTETS).record_length as u16
    }

    fn body_type(&self) -> PduType {
        PduType::Signal
    }
}

impl Interaction for Signal {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.radio_reference_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        None
    }
}

/// Table 177—Encoding Scheme record (7.7.3)
///
/// 5.8.4.3.2 Field-specific requirements
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum EncodingScheme {
    EncodedAudio {
        encoding_class: SignalEncodingClass,
        encoding_type: SignalEncodingType,
    },
    RawBinaryData {
        encoding_class: SignalEncodingClass,
        nr_of_messages: u16,
    },
    ApplicationSpecificData {
        encoding_class: SignalEncodingClass,
        user_protocol_id: SignalUserProtocolIdentificationNumber,
    },
    DatabaseIndex {
        encoding_class: SignalEncodingClass,
        index: u32,
        offset_milli_secs: u32,
        duration_milli_secs: u32,
    },
    Unspecified {
        encoding_class: SignalEncodingClass,
    },
}

impl Default for EncodingScheme {
    fn default() -> Self {
        EncodingScheme::EncodedAudio {
            encoding_class: SignalEncodingClass::default(),
            encoding_type: SignalEncodingType::default(),
        }
    }
}
