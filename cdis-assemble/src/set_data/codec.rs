use crate::codec::Codec;
use crate::records::model::EntityId;
use crate::set_data::model::SetData;
use crate::types::model::UVINT32;
use dis_rs::model::DatumSpecification;
use dis_rs::BodyRaw;

type Counterpart = dis_rs::set_data::model::SetData;

impl SetData {
    #[must_use]
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

    #[must_use]
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
