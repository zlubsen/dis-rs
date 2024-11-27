use crate::records::model::{EncodingScheme, EntityId};
use crate::signal::model::Signal;
use dis_rs::signal::model::EncodingScheme as DisEncodingScheme;

use crate::codec::Codec;
use crate::types::model::{UVINT16, UVINT32};

type Counterpart = dis_rs::signal::model::Signal;

impl Signal {
    pub fn encode(item: &Counterpart) -> Self {
        let (sample_rate, samples) =
            if let DisEncodingScheme::EncodedAudio { .. } = item.encoding_scheme {
                (
                    Some(UVINT32::from(item.sample_rate)),
                    Some(UVINT16::from(item.samples)),
                ) // both are required for audio
            } else {
                (None, None) // optional for all others - leave out; 13.21 g) and i)
            };

        Self {
            radio_reference_id: EntityId::encode(&item.radio_reference_id),
            radio_number: UVINT16::from(item.radio_number),
            encoding_scheme: EncodingScheme::encode(&item.encoding_scheme),
            tdl_type: item.tdl_type,
            sample_rate,
            samples,
            data: item.data.clone(),
        }
    }

    pub fn decode(&self) -> Counterpart {
        let sample_rate = if let Some(sample_rate) = self.sample_rate {
            sample_rate.value
        } else {
            Default::default()
        };
        let samples = if let Some(samples) = self.samples {
            samples.value
        } else {
            Default::default()
        };

        Counterpart::builder()
            .with_radio_reference_id(self.radio_reference_id.decode())
            .with_radio_number(self.radio_number.value)
            .with_encoding_scheme(self.encoding_scheme.decode())
            .with_tdl_type(self.tdl_type)
            .with_sample_rate(sample_rate)
            .with_samples(samples)
            .with_data(self.data.clone())
            .build()
    }
}
