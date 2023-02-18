use bytes::{BufMut, BytesMut};
use crate::common::entity_state_update::model::EntityStateUpdate;
use crate::common::{Serialize, SerializePdu, SupportedVersion};

impl SerializePdu for EntityStateUpdate {
    fn serialize_pdu(&self, _version: SupportedVersion, buf: &mut BytesMut) -> u16 {
        let entity_id_bytes = self.entity_id.serialize(buf);
        buf.put_u8(0u8);
        buf.put_u8(self.variable_parameters.len() as u8);
        let linear_velocity_bytes = self.entity_linear_velocity.serialize(buf);
        let location_bytes = self.entity_location.serialize(buf);
        let orientation_bytes = self.entity_orientation.serialize(buf);
        let appearance_bytes = self.entity_appearance.serialize(buf);
        let variable_params_bytes : u16 = self.variable_parameters.iter()
            .map(|param| param.serialize(buf))
            .sum();

        entity_id_bytes + 2 + linear_velocity_bytes + location_bytes
            + orientation_bytes + appearance_bytes + variable_params_bytes
    }
}