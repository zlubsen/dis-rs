use dis_rs::enumerations::EventType;
use dis_rs::model::DatumSpecification;
use crate::codec::Codec;
use crate::event_report::model::EventReport;
use crate::records::model::EntityId;
use crate::types::model::UVINT32;

type Counterpart = dis_rs::event_report::model::EventReport;

impl EventReport {
    pub fn encode(item: &Counterpart) -> Self {
        let event_type: u32 = item.event_type.into();
        Self {
            originating_id: EntityId::encode(&item.originating_id),
            receiving_id: EntityId::encode(&item.receiving_id),
            event_type: UVINT32::from(event_type),
            datum_specification: DatumSpecification::new(item.fixed_datum_records.clone(), item.variable_datum_records.clone())
        }
    }

    pub fn decode(&self) -> Counterpart {
        Counterpart::builder()
            .with_origination_id(self.originating_id.decode())
            .with_receiving_id(self.receiving_id.decode())
            .with_event_type(EventType::from(self.event_type.value))
            .with_fixed_datums(self.datum_specification.fixed_datum_records.clone())
            .with_variable_datums(self.datum_specification.variable_datum_records.clone())
            .build()
    }
}