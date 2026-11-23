use crate::constants::{
    EIGHT_BITS, FOUR_BITS, ONE_BIT, SIXTEEN_BITS, TEN_BITS, THIRTEEN_BITS, THIRTY_TWO_BITS,
    THREE_BITS, TWO_BITS,
};
use crate::records::model::{BeamAntennaPattern, CdisRecord};
use crate::transmitter::model::Transmitter;
use crate::types::model::{CdisFloat, UVINT8};
use crate::writing::{SerializeCdis, serialize_when_present, write_integer_bits};
use crate::{BitBuffer, BodyProperties};
use dis_rs::transmitter::model::VariableTransmitterParameter;

impl SerializeCdis for Transmitter {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_integer_bits(buf, cursor, EIGHT_BITS, self.fields_present_field());

        let cursor = write_integer_bits(
            buf,
            cursor,
            ONE_BIT,
            u8::from(self.units.world_location_altitude),
        );
        let cursor = write_integer_bits(
            buf,
            cursor,
            ONE_BIT,
            u8::from(self.units.relative_antenna_location),
        );
        let cursor = write_integer_bits(buf, cursor, ONE_BIT, u8::from(self.full_update_flag));

        let cursor = self.radio_reference_id.serialize(buf, cursor);
        let cursor = self.radio_number.serialize(buf, cursor);
        let cursor = serialize_when_present(&self.radio_type, buf, cursor);

        let cursor = write_integer_bits(buf, cursor, TWO_BITS, u8::from(self.transmit_state));
        let cursor = self.input_source.serialize(buf, cursor);

        let cursor = if self.variable_transmitter_parameters.is_empty() {
            cursor
        } else {
            UVINT8::from(self.variable_transmitter_parameters.len() as u8).serialize(buf, cursor)
        };

        let cursor = serialize_when_present(&self.antenna_location, buf, cursor);
        let cursor = serialize_when_present(&self.relative_antenna_location, buf, cursor);

        let cursor = if let Some(antenna_pattern_type) = self.antenna_pattern_type {
            write_integer_bits(buf, cursor, THREE_BITS, u16::from(antenna_pattern_type))
        } else {
            cursor
        };
        let cursor = if let Some(pattern) = self.antenna_pattern {
            write_integer_bits(buf, cursor, TEN_BITS, pattern.record_length())
        } else {
            cursor
        };

        let cursor = if let Some(frequency) = self.frequency {
            frequency.serialize(buf, cursor)
        } else {
            cursor
        };
        let cursor = if let Some(bandwidth) = self.transmit_frequency_bandwidth {
            bandwidth.serialize(buf, cursor)
        } else {
            cursor
        };

        let cursor = serialize_when_present(&self.power, buf, cursor);
        let cursor = if let Some(modulation) = &self.modulation_type {
            let cursor = write_integer_bits(buf, cursor, FOUR_BITS, modulation.spread_spectrum.0);
            let cursor = write_integer_bits(buf, cursor, FOUR_BITS, modulation.major_modulation);
            let cursor = write_integer_bits(buf, cursor, FOUR_BITS, modulation.detail);
            let cursor = write_integer_bits(buf, cursor, FOUR_BITS, modulation.radio_system);
            cursor
        } else {
            cursor
        };

        let cursor = if let Some(crypto_system) = self.crypto_system {
            write_integer_bits(buf, cursor, FOUR_BITS, u16::from(crypto_system))
        } else {
            cursor
        };
        let cursor = if let Some(crypto_key_id) = self.crypto_key_id {
            write_integer_bits(buf, cursor, SIXTEEN_BITS, u16::from(crypto_key_id))
        } else {
            cursor
        };

        let cursor = if !self.modulation_parameters.is_empty() {
            let cursor =
                write_integer_bits(buf, cursor, EIGHT_BITS, self.modulation_parameters.len());
            let cursor = self
                .modulation_parameters
                .iter()
                .fold(cursor, |cursor, byte| {
                    write_integer_bits(buf, cursor, EIGHT_BITS, *byte)
                });
            cursor
        } else {
            cursor
        };

        let cursor = if let Some(pattern) = self.antenna_pattern {
            pattern.serialize(buf, cursor)
        } else {
            cursor
        };

        let cursor = self
            .variable_transmitter_parameters
            .iter()
            .fold(cursor, |cursor, param| param.serialize(buf, cursor));

        cursor
    }
}

impl SerializeCdis for BeamAntennaPattern {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_integer_bits(buf, cursor, THIRTEEN_BITS, self.beam_direction_psi);
        let cursor = write_integer_bits(buf, cursor, THIRTEEN_BITS, self.beam_direction_theta);
        let cursor = write_integer_bits(buf, cursor, THIRTEEN_BITS, self.beam_direction_phi);
        let cursor = write_integer_bits(buf, cursor, THIRTEEN_BITS, self.az_beamwidth);
        let cursor = write_integer_bits(buf, cursor, THIRTEEN_BITS, self.el_beamwidth);

        let cursor = write_integer_bits(buf, cursor, TWO_BITS, u8::from(self.reference_system));
        let cursor = write_integer_bits(buf, cursor, SIXTEEN_BITS, self.e_z);
        let cursor = write_integer_bits(buf, cursor, SIXTEEN_BITS, self.e_x);
        let cursor = write_integer_bits(buf, cursor, THIRTEEN_BITS, self.phase);

        cursor
    }
}

impl SerializeCdis for VariableTransmitterParameter {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        const SIX_OCTETS: usize = 6;
        let cursor = write_integer_bits(buf, cursor, THIRTY_TWO_BITS, u32::from(self.record_type));
        let record_length = self.fields.len() + SIX_OCTETS;
        let cursor = write_integer_bits(buf, cursor, SIXTEEN_BITS, record_length);
        self.fields.iter().fold(cursor, |cursor, byte| {
            write_integer_bits(buf, cursor, EIGHT_BITS, *byte)
        })
    }
}
