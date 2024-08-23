use dis_rs::enumerations::VariableRecordType;
use dis_rs::transmitter::model::VariableTransmitterParameter;
use crate::{BitBuffer, BodyProperties};
use crate::constants::{EIGHT_BITS, FOUR_BITS, ONE_BIT, SIXTEEN_BITS, THIRTY_TWO_BITS, THREE_BITS, TWO_BITS};
use crate::transmitter::model::Transmitter;
use crate::types::model::{CdisFloat, UVINT8};
use crate::writing::{serialize_when_present, write_value_unsigned, SerializeCdis};

impl SerializeCdis for Transmitter {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_value_unsigned(buf, cursor, EIGHT_BITS, self.fields_present_field());

        let cursor = write_value_unsigned::<u8>(buf, cursor, ONE_BIT, self.units.world_location_altitude.into());
        let cursor = write_value_unsigned::<u8>(buf, cursor, ONE_BIT, self.units.relative_antenna_location.into());
        let cursor = write_value_unsigned::<u8>(buf, cursor, ONE_BIT, self.full_update_flag.into());

        let cursor = self.radio_reference_id.serialize(buf, cursor);
        let cursor = self.radio_number.serialize(buf, cursor);
        let cursor = serialize_when_present(&self.radio_type, buf, cursor);

        let transmit_state : u8 = self.transmit_state.into();
        let cursor = write_value_unsigned(buf, cursor, TWO_BITS, transmit_state);
        let cursor = self.input_source.serialize(buf, cursor);

        let cursor = if self.variable_transmitter_parameters.is_empty() {
            cursor
        } else {
            UVINT8::from(self.variable_transmitter_parameters.len() as u8).serialize(buf, cursor)
        };

        let cursor = serialize_when_present(&self.antenna_location, buf, cursor);
        let cursor = serialize_when_present(&self.relative_antenna_location, buf, cursor);
        let cursor = if let Some(antenna_pattern_type) = self.antenna_pattern_type {
            let antenna_pattern_type: u16 = antenna_pattern_type.into();
            write_value_unsigned(buf, cursor, THREE_BITS, antenna_pattern_type)
        } else { cursor };

        let cursor = self.antenna_pattern.iter()
            .fold(cursor, |cursor, byte | write_value_unsigned(buf, cursor, EIGHT_BITS, byte) );

        let cursor = if let Some(frequency) = self.frequency {
            frequency.serialize(buf, cursor)
        } else { cursor };
        let cursor = if let Some(bandwidth) = self.transmit_frequency_bandwidth {
            bandwidth.serialize(buf, cursor)
        } else { cursor };

        let cursor = serialize_when_present(&self.power, buf, cursor);
        let cursor = if let Some(modulation) = &self.modulation_type {
            let cursor = write_value_unsigned(buf, cursor, FOUR_BITS, modulation.spread_spectrum);
            let cursor = write_value_unsigned(buf, cursor, FOUR_BITS, modulation.major_modulation);
            let cursor = write_value_unsigned(buf, cursor, FOUR_BITS, modulation.detail);
            let cursor = write_value_unsigned(buf, cursor, FOUR_BITS, modulation.radio_system);
            cursor
        } else { cursor };

        let cursor = if let Some(crypto_system) = self.crypto_system {
            let crypto_system: u16 = crypto_system.into();
            write_value_unsigned(buf, cursor, FOUR_BITS, crypto_system)
        } else { cursor };
        let cursor = if let Some(crypto_key_id) = self.crypto_key_id {
            write_value_unsigned(buf, cursor, SIXTEEN_BITS, crypto_key_id)
        } else { cursor };

        let cursor = if !self.modulation_parameters.is_empty() {
            let cursor = write_value_unsigned(buf, cursor, EIGHT_BITS, self.modulation_parameters.len());
            let cursor = self.modulation_parameters.iter().fold(cursor, |cursor, byte| write_value_unsigned(buf, cursor, EIGHT_BITS, byte));
            cursor
        } else { cursor};

        let cursor = self.antenna_pattern.iter().fold(cursor, |cursor, byte| write_value_unsigned(buf, cursor, EIGHT_BITS, byte));

        let cursor = self.variable_transmitter_parameters.iter()
            .fold(cursor, |cursor, param| { param.serialize(buf, cursor) });

        cursor
    }
}

impl SerializeCdis for VariableTransmitterParameter {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        const SIX_OCTETS: usize = 6;
        let record_type: u32 = self.record_type.into();
        let cursor = write_value_unsigned(buf, cursor, THIRTY_TWO_BITS, record_type);
        let record_length = self.fields.len() + SIX_OCTETS;
        let cursor = write_value_unsigned(buf, cursor, SIXTEEN_BITS, record_length);
        let cursor = self.fields.iter().fold(cursor, |cursor, byte| write_value_unsigned(buf, cursor, EIGHT_BITS, byte));
        cursor
    }
}