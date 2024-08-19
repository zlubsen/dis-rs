use dis_rs::enumerations::SignalEncodingType;
use dis_rs::signal::model::EncodingScheme;
use crate::records::model::EntityId;
use crate::signal::model::Signal;

use crate::codec::Codec;
use crate::types::model::{UVINT16, UVINT32, UVINT8};

type Counterpart = dis_rs::signal::model::Signal;

impl Signal {
    fn encode(item: &Counterpart) -> Self {
        let (encoding_scheme_class, encoding_scheme_type) = match item.encoding_scheme {
            EncodingScheme::EncodedAudio { encoding_class, encoding_type } => {
                let encoding_type: u16 = encoding_type.into();
                (encoding_class, UVINT8::from(encoding_type as u8))
            }
            EncodingScheme::RawBinaryData { encoding_class, nr_of_messages } => { (encoding_class, UVINT8::from(nr_of_messages as u8)) }
            EncodingScheme::ApplicationSpecificData { encoding_class, .. } => { (encoding_class, Default::default()) }
            EncodingScheme::DatabaseIndex { encoding_class, .. } => { (encoding_class, Default::default()) }
            EncodingScheme::Unspecified { encoding_class } => { (encoding_class, Default::default()) }
        };

        Self {
            radio_reference_id: EntityId::encode(&item.radio_reference_id),
            radio_number: UVINT16::from(item.radio_number),
            encoding_scheme_class,
            encoding_scheme_type,
            tdl_type: item.tdl_type,
            sample_rate: Some(UVINT32::from(item.sample_rate)), // TODO required for audio, optional for all others
            samples: Some(UVINT16::from(item.samples)), // TODO required for audio, optional for all others
            data: item.data.clone(),
        }
    }

    fn decode(&self) -> Counterpart {

    }
}