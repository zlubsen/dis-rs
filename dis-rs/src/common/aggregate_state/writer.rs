use bytes::{BufMut, BytesMut};
use crate::aggregate_state::model::{aggregate_state_intermediate_length_padding, AggregateMarking, AggregateState, AggregateType, SilentAggregateSystem, SilentEntitySystem};
use crate::{Serialize, SerializePdu, SupportedVersion};
use crate::common::BodyInfo;

impl SerializePdu for AggregateState {
    fn serialize_pdu(&self, _version: SupportedVersion, buf: &mut BytesMut) -> u16 {
        self.aggregate_id.serialize(buf);
        self.force_id.serialize(buf);
        buf.put_u8(self.aggregate_state.into());
        self.aggregate_type.serialize(buf);
        buf.put_u32(self.formation.into());
        self.aggregate_marking.serialize(buf);
        self.dimensions.serialize(buf);
        self.orientation.serialize(buf);
        self.center_of_mass.serialize(buf);
        self.velocity.serialize(buf);

        buf.put_u16(self.aggregates.len() as u16);
        buf.put_u16(self.entities.len() as u16);
        buf.put_u16(self.silent_aggregate_systems.len() as u16);
        buf.put_u16(self.silent_entity_systems.len() as u16);

        self.aggregates.iter()
            .map(|record| record.serialize(buf) )
            .sum::<u16>();
        self.entities.iter()
            .map(|record| record.serialize(buf) )
            .sum::<u16>();

        let (_intermediate_length, padding_length) = aggregate_state_intermediate_length_padding(&self.aggregates, &self.entities);

        buf.put_bytes(0u8, padding_length.into());

        self.silent_aggregate_systems.iter()
            .map(|record| record.serialize(buf) )
            .sum::<u16>();

        self.silent_entity_systems.iter()
            .map(|record| record.serialize(buf) )
            .sum::<u16>();

        buf.put_u32(self.variable_datums.len() as u32 );
        self.variable_datums.iter()
            .map(|datum| datum.serialize(buf) )
            .sum::<u16>();

        self.body_length()
    }
}

impl Serialize for AggregateType {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        buf.put_u8(self.aggregate_kind.into());
        buf.put_u8(self.domain.into());
        buf.put_u16(self.country.into());
        buf.put_u8(self.category);
        buf.put_u8(self.subcategory.into());
        buf.put_u8(self.specific.into());
        buf.put_u8(self.extra);

        self.record_length()
    }
}

impl Serialize for AggregateMarking {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        buf.put_u8(self.marking_character_set.into());
        let num_pad = 31 - self.marking_string.len();
        let marking = self.marking_string.clone(); // clone necessary because into_bytes consumes self.

        buf.put_slice(&marking.into_bytes()[..]);
        (0..num_pad).for_each( |_i| buf.put_u8(0x20) );

        self.record_length()
    }
}

impl Serialize for SilentAggregateSystem {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        buf.put_u16(self.number_of_aggregates);
        buf.put_u16(0u16);
        self.aggregate_type.serialize(buf);

        self.record_length()
    }
}

impl Serialize for SilentEntitySystem {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        buf.put_u16(self.number_of_entities);
        buf.put_u16(self.appearances.len() as u16);
        self.entity_type.serialize(buf);
        self.appearances.iter()
            .map(|record | record.serialize(buf) )
            .sum::<u16>();

        self.record_length()
    }
}