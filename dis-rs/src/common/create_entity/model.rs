use crate::common::create_entity::builder::CreateEntityBuilder;
use crate::common::model::{EntityId, PduBody};
use crate::common::{BodyInfo, Interaction};
use crate::enumerations::PduType;
use serde::{Deserialize, Serialize};

const CREATE_ENTITY_BODY_LENGTH: u16 = 16;

/// 5.6.5.2 Create Entity PDU
///
/// 7.5.2 Create Entity PDU
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CreateEntity {
    pub originating_id: EntityId,
    pub receiving_id: EntityId,
    pub request_id: u32,
}

impl CreateEntity {
    #[must_use]
    pub fn builder() -> CreateEntityBuilder {
        CreateEntityBuilder::new()
    }

    #[must_use]
    pub fn into_builder(self) -> CreateEntityBuilder {
        CreateEntityBuilder::new_from_body(self)
    }

    #[must_use]
    pub fn into_pdu_body(self) -> PduBody {
        PduBody::CreateEntity(self)
    }
}

impl BodyInfo for CreateEntity {
    fn body_length(&self) -> u16 {
        CREATE_ENTITY_BODY_LENGTH
    }

    fn body_type(&self) -> PduType {
        PduType::CreateEntity
    }
}

impl Interaction for CreateEntity {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.originating_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        Some(&self.receiving_id)
    }
}
