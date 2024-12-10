use crate::action_response::model::ActionResponse;
use crate::codec::Codec;
use crate::records::model::EntityId;
use crate::types::model::UVINT32;
use dis_rs::enumerations::RequestStatus;
use dis_rs::model::DatumSpecification;

type Counterpart = dis_rs::action_response::model::ActionResponse;

impl ActionResponse {
    #[must_use]
    pub fn encode(item: &Counterpart) -> Self {
        let request_status: u32 = item.request_status.into();
        Self {
            originating_id: EntityId::encode(&item.originating_id),
            receiving_id: EntityId::encode(&item.receiving_id),
            request_id: UVINT32::from(item.request_id),
            request_status: UVINT32::from(request_status),
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
            .with_request_status(RequestStatus::from(self.request_status.value))
            .with_fixed_datums(self.datum_specification.fixed_datum_records.clone())
            .with_variable_datums(self.datum_specification.variable_datum_records.clone())
            .build()
    }
}
