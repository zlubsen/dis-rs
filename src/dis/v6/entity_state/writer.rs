use bytes::BytesMut;
use crate::dis::common::Serialize;
use crate::dis::v6::entity_state::model::EntityState;

impl Serialize for EntityState {
    fn serialize(&self, buf: &mut BytesMut) -> usize {
        todo!();
    }
}