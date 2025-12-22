use crate::common::entity_state::model::EntityAppearance;
use crate::common::model::{
    EntityId, Location, Orientation, PduBody, VariableParameter, VectorF32,
};
use crate::common::{BodyInfo, Interaction};
use crate::constants::VARIABLE_PARAMETER_RECORD_LENGTH;
use crate::entity_state_update::builder::EntityStateUpdateBuilder;
use crate::enumerations::PduType;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

const BASE_ENTITY_STATE_UPDATE_BODY_LENGTH: u16 = 60;

/// 5.3.5 Entity State Update PDU
///
/// 7.2.5 Entity State Update PDU
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EntityStateUpdate {
    pub entity_id: EntityId,
    pub entity_linear_velocity: VectorF32,
    pub entity_location: Location,
    pub entity_orientation: Orientation,
    pub entity_appearance: EntityAppearance,
    pub variable_parameters: Vec<VariableParameter>,
}

impl EntityStateUpdate {
    #[must_use]
    pub fn builder() -> EntityStateUpdateBuilder {
        EntityStateUpdateBuilder::new()
    }

    #[must_use]
    pub fn into_builder(self) -> EntityStateUpdateBuilder {
        EntityStateUpdateBuilder::new_from_body(self)
    }

    #[must_use]
    pub fn into_pdu_body(self) -> PduBody {
        PduBody::EntityStateUpdate(self)
    }
}

impl BodyInfo for EntityStateUpdate {
    fn body_length(&self) -> u16 {
        BASE_ENTITY_STATE_UPDATE_BODY_LENGTH
            + (VARIABLE_PARAMETER_RECORD_LENGTH * (self.variable_parameters.len() as u16))
    }

    fn body_type(&self) -> PduType {
        PduType::EntityStateUpdate
    }
}

impl Interaction for EntityStateUpdate {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.entity_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        None
    }
}

impl From<EntityStateUpdate> for PduBody {
    #[inline]
    fn from(value: EntityStateUpdate) -> Self {
        value.into_pdu_body()
    }
}
