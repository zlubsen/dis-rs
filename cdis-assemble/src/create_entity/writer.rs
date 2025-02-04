use crate::create_entity::model::CreateEntity;
use crate::writing::SerializeCdis;
use crate::{BitBuffer, SerializeCdisPdu};

impl SerializeCdisPdu for CreateEntity {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = self.originating_id.serialize(buf, cursor);
        let cursor = self.receiving_id.serialize(buf, cursor);
        let cursor = self.request_id.serialize(buf, cursor);

        cursor
    }
}
