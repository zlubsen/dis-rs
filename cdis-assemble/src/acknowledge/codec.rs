use crate::acknowledge::model::Acknowledge;
use crate::codec::Codec;
use crate::records::model::EntityId;
use crate::types::model::UVINT32;

type Counterpart = dis_rs::acknowledge::model::Acknowledge;

impl Acknowledge {
    pub fn encode(item: &Counterpart) -> Self {
        Self {
            originating_id: EntityId::encode(&item.originating_id),
            receiving_id: EntityId::encode(&item.receiving_id),
            acknowledge_flag: item.acknowledge_flag,
            response_flag: item.response_flag,
            request_id: UVINT32::from(item.request_id),
        }
    }

    pub fn decode(&self) -> Counterpart {
        Counterpart::builder()
            .with_origination_id(self.originating_id.decode())
            .with_receiving_id(self.receiving_id.decode())
            .with_acknowledge_flag(self.acknowledge_flag)
            .with_response_flag(self.response_flag)
            .with_request_id(self.request_id.value)
            .build()
    }
}