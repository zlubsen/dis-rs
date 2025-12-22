use crate::common::detonation::builder::DetonationBuilder;
use crate::common::model::{
    EntityId, EventId, ExpendableDescriptor, ExplosionDescriptor, Location, MunitionDescriptor,
    PduBody, VariableParameter, VectorF32,
};
use crate::common::{BodyInfo, Interaction};
use crate::constants::VARIABLE_PARAMETER_RECORD_LENGTH;
use crate::enumerations::{DetonationResult, PduType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

const BASE_DETONATION_BODY_LENGTH: u16 = 92;

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Detonation {
    pub source_entity_id: EntityId,
    pub target_entity_id: EntityId,
    pub exploding_entity_id: EntityId,
    pub event_id: EventId,
    pub velocity: VectorF32,
    pub location_in_world_coordinates: Location,
    pub descriptor: DetonationDescriptor,
    pub location_in_entity_coordinates: VectorF32,
    pub detonation_result: DetonationResult,
    pub variable_parameters: Vec<VariableParameter>,
}

impl Detonation {
    #[must_use]
    pub fn builder() -> DetonationBuilder {
        DetonationBuilder::new()
    }

    #[must_use]
    pub fn into_builder(self) -> DetonationBuilder {
        DetonationBuilder::new_from_body(self)
    }

    #[must_use]
    pub fn into_pdu_body(self) -> PduBody {
        PduBody::Detonation(self)
    }
}

impl BodyInfo for Detonation {
    fn body_length(&self) -> u16 {
        BASE_DETONATION_BODY_LENGTH
            + (VARIABLE_PARAMETER_RECORD_LENGTH * (self.variable_parameters.len() as u16))
    }

    fn body_type(&self) -> PduType {
        PduType::Detonation
    }
}

impl Interaction for Detonation {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.source_entity_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        Some(&self.target_entity_id)
    }
}

impl From<Detonation> for PduBody {
    #[inline]
    fn from(value: Detonation) -> Self {
        value.into_pdu_body()
    }
}

/// 6.2.19 Detonation Descriptor record
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DetonationDescriptor {
    #[cfg_attr(feature = "serde", serde(rename = "munition"))]
    Munition(MunitionDescriptor),
    #[cfg_attr(feature = "serde", serde(rename = "explosion"))]
    Explosion(ExplosionDescriptor),
    #[cfg_attr(feature = "serde", serde(rename = "expendable"))]
    Expendable(ExpendableDescriptor),
}

impl Default for DetonationDescriptor {
    fn default() -> Self {
        Self::Munition(MunitionDescriptor::default())
    }
}

impl From<MunitionDescriptor> for DetonationDescriptor {
    #[inline]
    fn from(value: MunitionDescriptor) -> Self {
        Self::Munition(value)
    }
}

impl From<ExplosionDescriptor> for DetonationDescriptor {
    #[inline]
    fn from(value: ExplosionDescriptor) -> Self {
        Self::Explosion(value)
    }
}

impl From<ExpendableDescriptor> for DetonationDescriptor {
    #[inline]
    fn from(value: ExpendableDescriptor) -> Self {
        Self::Expendable(value)
    }
}
