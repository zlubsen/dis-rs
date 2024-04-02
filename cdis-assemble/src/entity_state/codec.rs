use crate::codec::Codec;
use crate::entity_state::model::EntityState;

impl Codec for EntityState {
    type Counterpart = dis_rs::entity_state::model::EntityState;

    fn encode(item: Self::Counterpart) -> Self {
        todo!()
    }

    fn decode(&self) -> Self::Counterpart {
        todo!()
    }
}
