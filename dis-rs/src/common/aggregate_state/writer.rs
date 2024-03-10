use bytes::BytesMut;
use crate::aggregate_state::model::AggregateState;
use crate::{SerializePdu, SupportedVersion};

impl SerializePdu for AggregateState {
    fn serialize_pdu(&self, version: SupportedVersion, buf: &mut BytesMut) -> u16 {
        todo!()
    }
}