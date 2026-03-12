use crate::enumerations::SignalTdlType;
use crate::model::EntityId;
use crate::signal::model::{EncodingScheme, Signal};

pub struct SignalBuilder(Signal);

impl Default for SignalBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl SignalBuilder {
    #[must_use]
    pub fn new() -> Self {
        SignalBuilder(Signal::default())
    }

    #[must_use]
    pub fn new_from_body(body: Signal) -> Self {
        SignalBuilder(body)
    }

    #[must_use]
    pub fn build(self) -> Signal {
        self.0
    }

    #[must_use]
    pub fn with_radio_reference_id(mut self, radio_reference_id: EntityId) -> Self {
        self.0.radio_reference_id = radio_reference_id;
        self
    }

    #[must_use]
    pub fn with_radio_number(mut self, radio_number: u16) -> Self {
        self.0.radio_number = radio_number;
        self
    }

    #[must_use]
    pub fn with_encoding_scheme(mut self, encoding_scheme: EncodingScheme) -> Self {
        self.0.encoding_scheme = encoding_scheme;
        self
    }

    #[must_use]
    pub fn with_tdl_type(mut self, tdl_type: SignalTdlType) -> Self {
        self.0.tdl_type = tdl_type;
        self
    }

    #[must_use]
    pub fn with_sample_rate(mut self, sample_rate: u32) -> Self {
        self.0.sample_rate = sample_rate;
        self
    }

    #[must_use]
    pub fn with_samples(mut self, samples: u16) -> Self {
        self.0.samples = samples;
        self
    }

    #[must_use]
    pub fn with_data(mut self, data: Vec<u8>) -> Self {
        self.0.data = data;
        self
    }
}
