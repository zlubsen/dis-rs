use bytes::{BufMut, BytesMut};
use crate::dis::common::Serialize;
use crate::dis::common::entity_state::model::{EntityId, EntityMarking, EntityType, ForceId, Location, Orientation, SimulationAddress, VectorF32};

impl Serialize for EntityId {
    fn serialize(&self, buf: &mut BytesMut) -> usize {
        let num_bytes = self.simulation_address.serialize(buf);
        buf.put_u16(self.entity_id);
        num_bytes + 2
    }
}

impl Serialize for SimulationAddress {
    fn serialize(&self, buf: &mut BytesMut) -> usize {
        buf.put_u16(self.site_id);
        buf.put_u16(self.application_id);
        4
    }
}

impl Serialize for ForceId {
    fn serialize(&self, buf: &mut BytesMut) -> usize {
        let force_id = *self;
        buf.put_u8(force_id.into());
        1
    }
}

impl Serialize for EntityType {
    fn serialize(&self, buf: &mut BytesMut) -> usize {
        buf.put_u8(self.kind.into());
        buf.put_u8(self.domain);
        buf.put_u16(self.country.into()); // TODO: country Into<u16>;
        buf.put_u8(self.category);
        buf.put_u8(self.subcategory);
        buf.put_u8(self.specific);
        buf.put_u8(self.extra);
        8
    }
}

impl Serialize for VectorF32 {
    fn serialize(&self, buf: &mut BytesMut) -> usize {
        buf.put_f32(self.first_vector_component);
        buf.put_f32(self.second_vector_component);
        buf.put_f32(self.third_vector_component);
        12
    }
}

impl Serialize for Location {
    fn serialize(&self, buf: &mut BytesMut) -> usize {
        buf.put_f64(self.x_coordinate);
        buf.put_f64(self.y_coordinate);
        buf.put_f64(self.z_coordinate);
        24
    }
}

impl Serialize for Orientation {
    fn serialize(&self, buf: &mut BytesMut) -> usize {
        buf.put_f32(self.psi);
        buf.put_f32(self.theta);
        buf.put_f32(self.phi);
        12
    }
}

impl Serialize for EntityMarking {
    fn serialize(&self, buf: &mut BytesMut) -> usize {
        buf.put_u8(self.marking_character_set.into());
        let num_pad = 11 - self.marking_string.len();
        let marking = self.marking_string.clone(); // TODO is this clone necessary?

        buf.put_slice(&marking.into_bytes()[..]);
        (0..num_pad).for_each( |_i| buf.put_u8(0x20) );
        12
    }
}
