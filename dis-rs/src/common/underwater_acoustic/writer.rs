use bytes::{BufMut, BytesMut};
use crate::{Serialize, SerializePdu, SupportedVersion};
use crate::common::BodyInfo;
use crate::underwater_acoustic::model::{AcousticEmitterSystem, APA, PropulsionPlantConfiguration, Shaft, UABeam, UAEmitterSystem, UAFundamentalParameterData, UnderwaterAcoustic};

impl SerializePdu for UnderwaterAcoustic {
    fn serialize_pdu(&self, _version: SupportedVersion, buf: &mut BytesMut) -> u16 {
        let _emitter_bytes = self.emitting_entity_id.serialize(buf);
        let _event_id_bytes = self.event_id.serialize(buf);
        buf.put_u8(self.state_change_update_indicator.into());
        buf.put_u8(0u8);
        buf.put_u16(self.passive_parameter_index.into());
        let _ppc_bytes = self.propulsion_plant_configuration.serialize(buf);

        buf.put_u8(self.shafts.len() as u8);
        buf.put_u8(self.apas.len() as u8);
        buf.put_u8(self.emitter_systems.len() as u8);

        let _shafts = self.shafts.iter()
            .map(|shaft| shaft.serialize(buf) )
            .sum::<u16>();

        let _apas = self.apas.iter()
            .map(|apa| apa.serialize(buf) )
            .sum::<u16>();

        let _systems = self.emitter_systems.iter()
            .map(|system| system.serialize(buf) )
            .sum::<u16>();

        self.body_length()
    }
}

impl Serialize for PropulsionPlantConfiguration {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        let configuration : u8 = self.configuration.into();
        let hull_mounted_masker_on : u8 = if self.hull_mounted_masker {1} else {0};
        let final_field = (configuration << 1) | hull_mounted_masker_on;
        buf.put_u8(final_field);

        self.record_length()
    }
}

impl Serialize for Shaft {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        buf.put_i16(self.current_rpm);
        buf.put_i16(self.ordered_rpm);
        buf.put_i32(self.rpm_rate_of_change);

        self.record_length()
    }
}

impl Serialize for APA {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        let parameter: u16 = self.parameter.into();
        let parameter_status: u8 = self.status.into();
        let parameter_index = (parameter << 2) & (parameter_status as u16);
        buf.put_u16(parameter_index);
        buf.put_i16(self.value);

        self.record_length()
    }
}

impl Serialize for UAEmitterSystem {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        buf.put_u8(self.record_length() as u8);
        buf.put_u8(self.beams.len() as u8);
        buf.put_u16(0u16);
        self.acoustic_emitter_system.serialize(buf);
        self.location.serialize(buf);
        self.beams.iter().map(|beam| beam.serialize(buf) )
            .sum::<u16>();

        self.record_length()
    }
}

impl Serialize for AcousticEmitterSystem {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        buf.put_u16(self.acoustic_system_name.into());
        buf.put_u8(self.function.into());
        buf.put_u8(self.acoustic_id_number);

        self.record_length()
    }
}

impl Serialize for UABeam {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        buf.put_u8(self.beam_data_length);
        buf.put_u8(self.beam_id_number);
        buf.put_u16(0u16);
        self.fundamental_parameters.serialize(buf);

        self.record_length()
    }
}

impl Serialize for UAFundamentalParameterData {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        buf.put_u16(self.active_emission_parameter_index.into());
        buf.put_u16(self.scan_pattern.into());
        buf.put_f32(self.beam_center_azimuth);
        buf.put_f32(self.azimuthal_beamwidth);
        buf.put_f32(self.beam_center_depression_elevation);
        buf.put_f32(self.depression_elevation_beamwidth);

        self.record_length()
    }
}