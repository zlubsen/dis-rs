use crate::codec::Codec;
use crate::create_entity::model::CreateEntity;
use crate::records::model::EntityId;
use crate::types::model::UVINT32;
use dis_rs::BodyRaw;

type Counterpart = dis_rs::create_entity::model::CreateEntity;

impl CreateEntity {
    #[must_use]
    pub fn encode(item: &Counterpart) -> Self {
        Self {
            originating_id: EntityId::encode(&item.originating_id),
            receiving_id: EntityId::encode(&item.receiving_id),
            request_id: UVINT32::from(item.request_id),
        }
    }

    #[must_use]
    pub fn decode(&self) -> Counterpart {
        Counterpart::builder()
            .with_origination_id(self.originating_id.decode())
            .with_receiving_id(self.receiving_id.decode())
            .with_request_id(self.request_id.value)
            .build()
    }
}
