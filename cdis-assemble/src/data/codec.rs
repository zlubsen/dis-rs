use crate::codec::Codec;
use crate::data::model::Data;
use crate::records::model::EntityId;
use crate::types::model::UVINT32;
use dis_rs::model::DatumSpecification;

type Counterpart = dis_rs::data::model::Data;

impl Data {
    pub fn encode(item: &Counterpart) -> Self {
        Self {
            originating_id: EntityId::encode(&item.originating_id),
            receiving_id: EntityId::encode(&item.receiving_id),
            request_id: UVINT32::from(item.request_id),
            datum_specification: DatumSpecification::new(
                item.fixed_datum_records.clone(),
                item.variable_datum_records.clone(),
            ),
        }
    }

    pub fn decode(&self) -> Counterpart {
        Counterpart::builder()
            .with_origination_id(self.originating_id.decode())
            .with_receiving_id(self.receiving_id.decode())
            .with_request_id(self.request_id.value)
            .with_fixed_datums(self.datum_specification.fixed_datum_records.clone())
            .with_variable_datums(self.datum_specification.variable_datum_records.clone())
            .build()
    }
}
