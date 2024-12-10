use crate::codec::Codec;
use crate::data_query::model::DataQuery;
use crate::records::model::{cdis_to_dis_u32_timestamp, CdisTimeStamp, EntityId};
use crate::types::model::UVINT32;
use dis_rs::model::{DisTimeStamp, TimeStamp};

type Counterpart = dis_rs::data_query::model::DataQuery;

impl DataQuery {
    #[must_use]
    pub fn encode(item: &Counterpart) -> Self {
        let time_interval = CdisTimeStamp::from(DisTimeStamp::from(item.time_interval));
        Self {
            originating_id: EntityId::encode(&item.originating_id),
            receiving_id: EntityId::encode(&item.receiving_id),
            request_id: UVINT32::from(item.request_id),
            time_interval,
            fixed_datum_ids: item.fixed_datum_records.clone(),
            variable_datum_ids: item.variable_datum_records.clone(),
        }
    }

    #[must_use]
    pub fn decode(&self) -> Counterpart {
        let time_interval =
            cdis_to_dis_u32_timestamp(TimeStamp::from(self.time_interval).raw_timestamp);
        Counterpart::builder()
            .with_origination_id(self.originating_id.decode())
            .with_receiving_id(self.receiving_id.decode())
            .with_request_id(self.request_id.value)
            .with_time_interval(time_interval)
            .with_fixed_datums(self.fixed_datum_ids.clone())
            .with_variable_datums(self.variable_datum_ids.clone())
            .build()
    }
}
