use crate::common::model::length_padded_to_num;
use crate::common::transmitter::model::{
    BeamAntennaPattern, CryptoKeyId, CryptoMode, ModulationType, SpreadSpectrum, Transmitter,
    VariableTransmitterParameter, BASE_VTP_RECORD_LENGTH, BEAM_ANTENNA_PATTERN_OCTETS,
};
use crate::common::{Serialize, SerializePdu, SupportedVersion};
use crate::constants::{EIGHT_OCTETS, ZERO_OCTETS};
use bytes::{BufMut, BytesMut};

impl SerializePdu for Transmitter {
    fn serialize_pdu(&self, _version: SupportedVersion, buf: &mut BytesMut) -> u16 {
        self.radio_reference_id.serialize(buf);
        buf.put_u16(self.radio_number);
        self.radio_type.serialize(buf);
        buf.put_u8(self.transmit_state.into());
        buf.put_u8(self.input_source.into());
        buf.put_u16(self.variable_transmitter_parameters.len() as u16);
        self.antenna_location.serialize(buf);
        self.relative_antenna_location.serialize(buf);
        buf.put_u16(self.antenna_pattern_type.into());
        if self.antenna_pattern.is_some() {
            buf.put_u16(BEAM_ANTENNA_PATTERN_OCTETS);
        } else {
            buf.put_u16(ZERO_OCTETS as u16);
        }
        buf.put_u64(self.frequency);
        buf.put_f32(self.transmit_frequency_bandwidth);
        buf.put_f32(self.power);
        self.modulation_type.serialize(buf);
        buf.put_u16(self.crypto_system.into());
        self.crypto_key_id.serialize(buf);
        if let Some(modulation_parameters) = &self.modulation_parameters {
            buf.put_u8(modulation_parameters.len() as u8);
        } else {
            buf.put_u8(ZERO_OCTETS as u8)
        }
        buf.put_u8(0u8);
        buf.put_u16(0u16);

        let modulation_parameters_bytes =
            if let Some(modulation_parameters) = &self.modulation_parameters {
                modulation_parameters
                    .iter()
                    .map(|field| {
                        buf.put_u8(*field);
                        1u16
                    })
                    .sum::<u16>()
            } else {
                0u16
            };

        let antenna_pattern_bytes = if let Some(antenna_pattern) = &self.antenna_pattern {
            antenna_pattern.serialize(buf);
            BEAM_ANTENNA_PATTERN_OCTETS
        } else {
            ZERO_OCTETS as u16
        };

        let vtp_bytes = self
            .variable_transmitter_parameters
            .iter()
            .map(|vtp| vtp.serialize(buf))
            .sum::<u16>();

        104 + modulation_parameters_bytes + antenna_pattern_bytes + vtp_bytes
    }
}

impl Serialize for ModulationType {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        let spread_spectrum_bytes = self.spread_spectrum.serialize(buf);
        let (major_modulation, detail) = self.major_modulation.to_bytes_with_detail();
        buf.put_u16(major_modulation);
        buf.put_u16(detail);
        buf.put_u16(self.radio_system.into());

        spread_spectrum_bytes + 6
    }
}

impl Serialize for SpreadSpectrum {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        buf.put_u16(u16::from(self));
        2
    }
}

impl Serialize for CryptoKeyId {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        let field = self.pseudo_crypto_key << 1;
        let field = match self.crypto_mode {
            CryptoMode::Baseband => field,
            CryptoMode::Diphase => field + 1,
        };
        buf.put_u16(field);
        2
    }
}

impl Serialize for BeamAntennaPattern {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        self.beam_direction.serialize(buf);
        buf.put_f32(self.azimuth_beamwidth);
        buf.put_f32(self.elevation_beamwidth);
        buf.put_u8(self.reference_system.into());
        buf.put_u8(0u8);
        buf.put_u16(0u16);
        buf.put_f32(self.e_z);
        buf.put_f32(self.e_x);
        buf.put_f32(self.phase);
        buf.put_u32(0u32);

        BEAM_ANTENNA_PATTERN_OCTETS
    }
}

impl Serialize for VariableTransmitterParameter {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        let record_padded_lengths = length_padded_to_num(
            BASE_VTP_RECORD_LENGTH as usize + self.fields.len(),
            EIGHT_OCTETS,
        );
        let record_length_bytes = record_padded_lengths.record_length as u16;

        buf.put_u32(self.record_type.into());
        buf.put_u16(record_length_bytes);
        buf.put(&*self.fields);
        buf.put_bytes(0u8, record_padded_lengths.padding_length);

        record_length_bytes
    }
}
