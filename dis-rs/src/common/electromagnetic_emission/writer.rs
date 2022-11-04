use bytes::{BufMut, BytesMut};
use nom::multi::count;
use nom::number::complete::{be_u16, be_u8};
use crate::common::electromagnetic_emission::model::{Beam, BeamData, ElectromagneticEmission, EmitterSystem, FundamentalParameterData, JammingTechnique, TrackJam};
use crate::common::{Serialize, SerializePdu, SupportedVersion};
use crate::common::electromagnetic_emission::parser::{beam_data, fundamental_parameter_data, jamming_technique, track_jam};
use crate::common::parser::{entity_id, event_id, vec3_f32};

impl SerializePdu for ElectromagneticEmission {
    fn serialize_pdu(&self, version: SupportedVersion, buf: &mut BytesMut) -> u16 {
        let entity_bytes = self.emitting_entity_id.serialize(buf);
        let event_bytes = self.event_id.serialize(buf);
        buf.put_u8(self.state_update_indicator.into());
        buf.put_u8(self.emitter_systems.len() as u8);
        buf.put_u16(0u16);
        let systems_bytes = self.emitter_systems.iter()
            .map(|system| system.serialize(buf))
            .sum();

        entity_bytes + event_bytes + 4 + systems_bytes
    }
}

impl Serialize for EmitterSystem {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        buf.put_u8(self.system_data_length() as u8);
        buf.put_u8(self.beams.len() as u8);
        buf.put_u16(0u16);
        buf.put_u16(self.name.into());
        buf.put_u8(self.function.into());
        buf.put_u8(self.number.into());
        let location_bytes = self.location.serialize(buf);

        let beams_bytes = self.beams.iter()
            .map(|beam| beam.serialize(buf))
            .sum::<u16>();

        8 + location_bytes + beams_bytes
    }
}

impl Serialize for Beam {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        buf.put_u8(self.beam_data_length() as u8);
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

impl Serialize for BeamData {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        buf.put_f32(self.azimuth_center);
        buf.put_f32(self.azimuth_sweep);
        buf.put_f32(self.elevation_center);
        buf.put_f32(self.elevation_sweep);
        buf.put_f32(self.sweep_sync);

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