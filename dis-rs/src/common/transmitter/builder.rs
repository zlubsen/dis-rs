use crate::enumerations::{
    TransmitterAntennaPatternType, TransmitterCryptoSystem, TransmitterInputSource,
    TransmitterTransmitState,
};
use crate::model::{EntityId, EntityType, Location, VectorF32};
use crate::transmitter::model::{
    BeamAntennaPattern, CryptoKeyId, ModulationType, Transmitter, VariableTransmitterParameter,
};

pub struct TransmitterBuilder(Transmitter);

impl TransmitterBuilder {
    pub fn new() -> Self {
        TransmitterBuilder(Transmitter::default())
    }

    pub fn new_from_body(body: Transmitter) -> Self {
        TransmitterBuilder(body)
    }

    pub fn build(self) -> Transmitter {
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

    pub fn with_radio_type(mut self, radio_type: EntityType) -> Self {
        self.0.radio_type = radio_type;
        self
    }

    pub fn with_transmit_state(mut self, transmit_state: TransmitterTransmitState) -> Self {
        self.0.transmit_state = transmit_state;
        self
    }

    pub fn with_input_source(mut self, input_source: TransmitterInputSource) -> Self {
        self.0.input_source = input_source;
        self
    }

    pub fn with_antenna_location(mut self, antenna_location: Location) -> Self {
        self.0.antenna_location = antenna_location;
        self
    }

    pub fn with_relative_antenna_location(mut self, relative_antenna_location: VectorF32) -> Self {
        self.0.relative_antenna_location = relative_antenna_location;
        self
    }

    pub fn with_antenna_pattern_type(
        mut self,
        antenna_pattern_type: TransmitterAntennaPatternType,
    ) -> Self {
        self.0.antenna_pattern_type = antenna_pattern_type;
        self
    }

    pub fn with_frequency(mut self, frequency: u64) -> Self {
        self.0.frequency = frequency;
        self
    }

    pub fn with_transmit_frequency_bandwidth(mut self, transmit_frequency_bandwidth: f32) -> Self {
        self.0.transmit_frequency_bandwidth = transmit_frequency_bandwidth;
        self
    }
    pub fn with_power(mut self, power: f32) -> Self {
        self.0.power = power;
        self
    }

    pub fn with_modulation_type(mut self, modulation_type: ModulationType) -> Self {
        self.0.modulation_type = modulation_type;
        self
    }

    pub fn with_crypto_system(mut self, crypto_system: TransmitterCryptoSystem) -> Self {
        self.0.crypto_system = crypto_system;
        self
    }

    pub fn with_crypto_key_id(mut self, crypto_key_id: CryptoKeyId) -> Self {
        self.0.crypto_key_id = crypto_key_id;
        self
    }
    pub fn with_modulation_parameters(mut self, modulation_parameters: Vec<u8>) -> Self {
        self.0.modulation_parameters = Some(modulation_parameters);
        self
    }

    pub fn with_antenna_pattern(mut self, antenna_pattern: BeamAntennaPattern) -> Self {
        self.0.antenna_pattern = Some(antenna_pattern);
        self
    }

    pub fn with_variable_transmitter_parameters(
        mut self,
        variable_transmitter_parameters: Vec<VariableTransmitterParameter>,
    ) -> Self {
        self.0.variable_transmitter_parameters = variable_transmitter_parameters;
        self
    }

    pub fn with_variable_transmitter_parameter(
        mut self,
        variable_transmitter_parameter: VariableTransmitterParameter,
    ) -> Self {
        self.0
            .variable_transmitter_parameters
            .push(variable_transmitter_parameter);
        self
    }
}
