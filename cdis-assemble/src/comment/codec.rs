use dis_rs::model::DatumSpecification;
use crate::codec::Codec;
use crate::comment::model::Comment;
use crate::records::model::EntityId;

type Counterpart = dis_rs::comment::model::Comment;

impl Comment {
    pub fn encode(item: &Counterpart) -> Self {
        Self {
            originating_id: EntityId::encode(&item.originating_id),
            receiving_id: EntityId::encode(&item.receiving_id),
            datum_specification: DatumSpecification::new(vec![], item.variable_datum_records.clone())
        }
    }

    pub fn decode(&self) -> Counterpart {
        Counterpart::builder()
            .with_origination_id(self.originating_id.decode())
            .with_receiving_id(self.receiving_id.decode())
            .with_variable_datums(self.datum_specification.variable_datum_records.clone())
            .build()
    }
}