use crate::codec::Codec;
use crate::records::model::EntityId;
use crate::stop_freeze::model::StopFreeze;
use crate::types::model::UVINT32;

type Counterpart = dis_rs::stop_freeze::model::StopFreeze;

impl StopFreeze {
    pub fn encode(item: &Counterpart) -> Self {
        Self {
            originating_id: EntityId::encode(&item.originating_id),
            receiving_id: EntityId::encode(&item.receiving_id),
            real_world_time: item.real_world_time,
            reason: item.reason,
            frozen_behavior: item.frozen_behavior,
            request_id: UVINT32::from(item.request_id),
        }
    }

    pub fn decode(&self) -> Counterpart {
        Counterpart::builder()
            .with_origination_id(self.originating_id.decode())
            .with_receiving_id(self.receiving_id.decode())
            .with_real_world_time(self.real_world_time)
            .with_reason(self.reason)
            .with_frozen_behavior(self.frozen_behavior)
            .with_request_id(self.request_id.value)
            .build()
    }
}
