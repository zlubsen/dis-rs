use crate::codec::Codec;
use crate::records::model::EntityId;
use crate::remove_entity::model::RemoveEntity;
use crate::types::model::UVINT32;

type Counterpart = dis_rs::remove_entity::model::RemoveEntity;

impl RemoveEntity {
    pub fn encode(item: &Counterpart) -> Self {
        Self {
            originating_id: EntityId::encode(&item.originating_id),
            receiving_id: EntityId::encode(&item.receiving_id),
            request_id: UVINT32::from(item.request_id),
        }
    }

    pub fn decode(&self) -> Counterpart {
        Counterpart::builder()
            .with_origination_id(self.originating_id.decode())
            .with_receiving_id(self.receiving_id.decode())
            .with_request_id(self.request_id.value)
            .build()
    }
}