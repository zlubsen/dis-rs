use crate::is_part_of::model::{IsPartOf, NamedLocationId, Relationship};
use crate::{Serialize, SerializePdu, SupportedVersion};
use bytes::{BufMut, BytesMut};

impl SerializePdu for IsPartOf {
    fn serialize_pdu(&self, _version: SupportedVersion, buf: &mut BytesMut) -> u16 {
        let originating_bytes = self.originating_simulation_id.serialize(buf);
        let receiving_bytes = self.receiving_entity_id.serialize(buf);
        let relationship_bytes = self.relationship.serialize(buf);
        let part_location_bytes = self.part_location.serialize(buf);
        let named_location_bytes = self.named_location_id.serialize(buf);
        let part_type_bytes = self.part_type.serialize(buf);

        originating_bytes
            + receiving_bytes
            + relationship_bytes
            + part_location_bytes
            + named_location_bytes
            + part_type_bytes
    }
}

impl Serialize for Relationship {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        buf.put_u16(self.nature.into());
        buf.put_u16(self.position.into());

        4
    }
}

impl Serialize for NamedLocationId {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        buf.put_u16(self.station_name.into());
        buf.put_u16(self.station_number);

        4
    }
}
