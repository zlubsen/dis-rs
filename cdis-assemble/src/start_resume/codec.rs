use crate::codec::Codec;
use crate::records::model::EntityId;
use crate::start_resume::model::StartResume;
use crate::types::model::UVINT32;

type Counterpart = dis_rs::start_resume::model::StartResume;

impl StartResume {
    pub fn encode(item: &Counterpart) -> Self {
        Self {
            originating_id: EntityId::encode(&item.originating_id),
            receiving_id: EntityId::encode(&item.receiving_id),
            real_world_time: item.real_world_time,
            simulation_time: item.simulation_time,
            request_id: UVINT32::from(item.request_id),
        }
    }

    pub fn decode(&self) -> Counterpart {
        Counterpart::builder()
            .with_origination_id(self.originating_id.decode())
            .with_receiving_id(self.receiving_id.decode())
            .with_real_world_time(self.real_world_time)
            .with_simulation_time(self.simulation_time)
            .with_request_id(self.request_id.value)
            .build()
    }
}