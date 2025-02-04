use crate::sees::model::{PropulsionSystemData, VectoringNozzleSystemData, SEES};
use crate::{Serialize, SerializePdu, SupportedVersion};
use bytes::{BufMut, BytesMut};

impl SerializePdu for SEES {
    fn serialize_pdu(&self, _version: SupportedVersion, buf: &mut BytesMut) -> u16 {
        let originating_bytes = self.originating_entity_id.serialize(buf);
        buf.put_u16(self.infrared_signature_representation_index);
        buf.put_u16(self.acoustic_signature_representation_index);
        buf.put_u16(self.radar_cross_section_representation_index);
        buf.put_u16(self.propulsion_systems.len() as u16);
        buf.put_u16(self.vectoring_nozzle_systems.len() as u16);

        let propulsion_system_bytes = self
            .propulsion_systems
            .iter()
            .map(|system| system.serialize(buf))
            .sum::<u16>();
        let vectoring_nozzle_bytes = self
            .vectoring_nozzle_systems
            .iter()
            .map(|system| system.serialize(buf))
            .sum::<u16>();

        originating_bytes + 10 + propulsion_system_bytes + vectoring_nozzle_bytes
    }
}

impl Serialize for PropulsionSystemData {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        buf.put_f32(self.power_setting);
        buf.put_f32(self.engine_rpm);

        8
    }
}

impl Serialize for VectoringNozzleSystemData {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        buf.put_f32(self.horizontal_deflection_angle);
        buf.put_f32(self.vertical_deflection_angle);

        8
    }
}
