use bytes::{BufMut, BytesMut};
use crate::common::electromagnetic_emission::model::{Beam, ElectromagneticEmission, EmitterSystem, FundamentalParameterData, JammingTechnique, TrackJam};
use crate::common::{Serialize, SerializePdu, SupportedVersion};

impl SerializePdu for ElectromagneticEmission {
    fn serialize_pdu(&self, _version: SupportedVersion, buf: &mut BytesMut) -> u16 {
        let entity_bytes = self.emitting_entity_id.serialize(buf);
        let event_bytes = self.event_id.serialize(buf);
        buf.put_u8(self.state_update_indicator.into());
        buf.put_u8(self.emitter_systems.len() as u8);
        buf.put_u16(0u16);
        let systems_bytes = self.emitter_systems.iter()
            .map(|system| system.serialize(buf))
            .sum::<u16>();

        entity_bytes + event_bytes + 4 + systems_bytes
    }
}

impl Serialize for EmitterSystem {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        let system_length_in_words = self.system_data_length_bytes() / 4;
        buf.put_u8(system_length_in_words as u8);
        buf.put_u8(self.beams.len() as u8);
        buf.put_u16(0u16);
        buf.put_u16(self.name.into());
        buf.put_u8(self.function.into());
        buf.put_u8(self.number);
        let location_bytes = self.location.serialize(buf);

        let beams_bytes = self.beams.iter()
            .map(|beam| beam.serialize(buf))
            .sum::<u16>();

        8 + location_bytes + beams_bytes
    }
}

impl Serialize for Beam {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        let beam_length_in_words = self.beam_data_length_bytes() / 4;
        buf.put_u8(beam_length_in_words as u8);
        buf.put_u8(self.number);
        buf.put_u16(self.parameter_index);
        let parameter_data_bytes = self.parameter_data.serialize(buf);
        let beam_data_bytes = self.beam_data.serialize(buf);
        buf.put_u8(self.beam_function.into());
        buf.put_u8(self.track_jam_data.len() as u8);
        buf.put_u8(self.high_density_track_jam.into());
        buf.put_u8(self.beam_status.into());
        let technique_bytes = self.jamming_technique.serialize(buf);

        let tracks_bytes = self.track_jam_data.iter()
            .map(|tracks| tracks.serialize(buf))
            .sum::<u16>();

        8 + parameter_data_bytes + beam_data_bytes + technique_bytes + tracks_bytes
    }
}

impl Serialize for FundamentalParameterData {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        buf.put_f32(self.frequency);
        buf.put_f32(self.frequency_range);
        buf.put_f32(self.effective_power);
        buf.put_f32(self.pulse_repetition_frequency);
        buf.put_f32(self.pulse_width);

        20
    }
}

impl Serialize for JammingTechnique {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        buf.put_u8(self.kind);
        buf.put_u8(self.category);
        buf.put_u8(self.subcategory);
        buf.put_u8(self.specific);

        4
    }
}

impl Serialize for TrackJam {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        let entity_bytes = self.entity_id.serialize(buf);
        buf.put_u8(self.emitter);
        buf.put_u8(self.beam);

        entity_bytes + 2
    }
}

#[cfg(test)]
mod tests {
    use bytes::BytesMut;
    use crate::common::electromagnetic_emission::model::{Beam, ElectromagneticEmission, EmitterSystem, FundamentalParameterData, TrackJam};
    use crate::{ElectromagneticEmissionBeamFunction, EmitterName, EmitterSystemFunction, EntityId, EventId, HighDensityTrackJam, Pdu, PduHeader, SimulationAddress, VectorF32};
    use crate::common::{BodyInfo, Serialize};
    use crate::common::model::BeamData;

    #[test]
    fn write_pdu_emission_with_tracks() {
        let body = ElectromagneticEmission::new()
            .with_emitting_entity_id(
                EntityId::new(500, 11111, 62))
            .with_event_id(
                EventId::new(SimulationAddress::new(500, 11111), 577))
            .with_emitter_system(EmitterSystem::new()
                .with_name(EmitterName::Unnamed_32175)
                .with_function(EmitterSystemFunction::Multifunction_1)
                .with_number(1)
                .with_location(VectorF32::new(0f32,0f32,5f32))
                .with_beam(Beam::new()
                    .with_number(2)
                    .with_parameter_index(122)
                    .with_parameter_data(FundamentalParameterData::new()
                        .with_frequency(6000000000f32)
                        .with_frequency_range(0f32)
                        .with_effective_power(f32::from_be_bytes([0x42, 0xf0, 0x00, 0x00]))
                        .with_pulse_repetition_frequency(0f32)
                        .with_pulse_width(0f32))
                    .with_beam_data(BeamData::new()
                        .with_azimuth_center(f32::from_be_bytes([0x3e, 0x05, 0xd5, 0xff]))
                        .with_azimuth_sweep(f32::from_be_bytes([0x3c, 0x0e, 0xfa, 0x35]))
                        .with_elevation_center(f32::from_be_bytes([0x3f, 0x30, 0x95, 0x12]))
                        .with_elevation_sweep(f32::from_be_bytes([0x3c, 0x0e, 0xfa, 0x35]))
                        .with_sweep_sync(0f32))
                    .with_beam_function(ElectromagneticEmissionBeamFunction::Illumination)
                    .with_high_density_track_jam(HighDensityTrackJam::NotSelected)
                    .with_track_jam(TrackJam::new()
                        .with_beam(0)
                        .with_emitter(0)
                        .with_entity_id(EntityId::new(500,11111,71))))
                .with_beam(Beam::new()
                    .with_number(1)
                    .with_parameter_index(100)
                    .with_parameter_data(FundamentalParameterData::new()
                        .with_frequency(6000000000f32)
                        .with_frequency_range(0f32)
                        .with_effective_power(f32::from_be_bytes([0x42, 0xf0, 0x00, 0x00]))
                        .with_pulse_repetition_frequency(f32::from_be_bytes([0x45, 0x9c, 0x40, 0x00]))
                        .with_pulse_width(f32::from_be_bytes([0x42, 0x48, 0x00, 0x00])))
                    .with_beam_data(BeamData::new()
                        .with_azimuth_center(f32::from_be_bytes([0x3e, 0x05, 0xd6, 0x14]))
                        .with_azimuth_sweep(f32::from_be_bytes([0x3c, 0x0e, 0xfa, 0x35]))
                        .with_elevation_center(f32::from_be_bytes([0x3f, 0x30, 0x95, 0x54]))
                        .with_elevation_sweep(f32::from_be_bytes([0x3c, 0x0e, 0xfa, 0x35]))
                        .with_sweep_sync(0f32))
                    .with_beam_function(ElectromagneticEmissionBeamFunction::Tracking)
                    .with_high_density_track_jam(HighDensityTrackJam::NotSelected)
                    .with_track_jam(TrackJam::new()
                        .with_beam(0)
                        .with_emitter(0)
                        .with_entity_id(EntityId::new(500,11111,71))))
                .with_beam(Beam::new()
                    .with_number(3)
                    .with_parameter_index(212)
                    .with_parameter_data(FundamentalParameterData::new()
                        .with_frequency(6000000000f32)
                        .with_frequency_range(0f32)
                        .with_effective_power(f32::from_be_bytes([0x42, 0xf0, 0x00, 0x00]))
                        .with_pulse_repetition_frequency(f32::from_be_bytes([0x43, 0xbb, 0x8c, 0x01]))
                        .with_pulse_width(f32::from_be_bytes([0x40, 0xa0, 0x00, 0x00])))
                    .with_beam_data(BeamData::new()
                        .with_azimuth_center(f32::from_be_bytes([0x3e, 0x05, 0xd6, 0x14]))
                        .with_azimuth_sweep(f32::from_be_bytes([0x3c, 0x0e, 0xfa, 0x35]))
                        .with_elevation_center(f32::from_be_bytes([0x3f, 0x30, 0x95, 0x54]))
                        .with_elevation_sweep(f32::from_be_bytes([0x3c, 0x0e, 0xfa, 0x35]))
                        .with_sweep_sync(0f32))
                    .with_beam_function(ElectromagneticEmissionBeamFunction::Commandguidance)
                    .with_high_density_track_jam(HighDensityTrackJam::NotSelected)
                    .with_track_jam(TrackJam::new()
                        .with_beam(0)
                        .with_emitter(0)
                        .with_entity_id(EntityId::new(500,11111,71))))
            ).into_pdu_body();

        let header = PduHeader::new_v6(1, body.body_type());
        let pdu = Pdu::finalize_from_parts(header, body, 0);

        let mut buf = BytesMut::with_capacity(228);

        let len = pdu.serialize(&mut buf);
        assert_eq!(len, 228);

        let expected: [u8; 228] =
            [0x06, 0x01, 0x17, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0xe4, 0x00, 0x00, 0x01, 0xf4, 0x2b, 0x67,
                0x00, 0x3e, 0x01, 0xf4, 0x2b, 0x67, 0x02, 0x41, 0x00, 0x01, 0x00, 0x00, 0x32, 0x03, 0x00, 0x00,
                0x7d, 0xaf, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0xa0, 0x00, 0x00,
                0x0f, 0x02, 0x00, 0x7a, 0x4f, 0xb2, 0xd0, 0x5e, 0x00, 0x00, 0x00, 0x00, 0x42, 0xf0, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3e, 0x05, 0xd5, 0xff, 0x3c, 0x0e, 0xfa, 0x35,
                0x3f, 0x30, 0x95, 0x12, 0x3c, 0x0e, 0xfa, 0x35, 0x00, 0x00, 0x00, 0x00, 0x07, 0x01, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x01, 0xf4, 0x2b, 0x67, 0x00, 0x47, 0x00, 0x00, 0x0f, 0x01, 0x00, 0x64,
                0x4f, 0xb2, 0xd0, 0x5e, 0x00, 0x00, 0x00, 0x00, 0x42, 0xf0, 0x00, 0x00, 0x45, 0x9c, 0x40, 0x00,
                0x42, 0x48, 0x00, 0x00, 0x3e, 0x05, 0xd6, 0x14, 0x3c, 0x0e, 0xfa, 0x35, 0x3f, 0x30, 0x95, 0x54,
                0x3c, 0x0e, 0xfa, 0x35, 0x00, 0x00, 0x00, 0x00, 0x04, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x01, 0xf4, 0x2b, 0x67, 0x00, 0x47, 0x00, 0x00, 0x0f, 0x03, 0x00, 0xd4, 0x4f, 0xb2, 0xd0, 0x5e,
                0x00, 0x00, 0x00, 0x00, 0x42, 0xf0, 0x00, 0x00, 0x43, 0xbb, 0x8c, 0x01, 0x40, 0xa0, 0x00, 0x00,
                0x3e, 0x05, 0xd6, 0x14, 0x3c, 0x0e, 0xfa, 0x35, 0x3f, 0x30, 0x95, 0x54, 0x3c, 0x0e, 0xfa, 0x35,
                0x00, 0x00, 0x00, 0x00, 0x06, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0xf4, 0x2b, 0x67,
                0x00, 0x47, 0x00, 0x00];

        assert_eq!(buf.as_ref(), expected.as_ref())
    }
}