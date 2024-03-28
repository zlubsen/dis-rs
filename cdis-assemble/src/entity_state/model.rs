use dis_rs::entity_state::model::{DrOtherParameters, EntityAppearance};
use dis_rs::enumerations::{DeadReckoningAlgorithm, ForceId};
use crate::records::model::{AngularVelocity, CdisEntityMarking, CdisVariableParameter, EntityId, EntityType, LinearVelocity, Orientation, Units, WorldCoordinates};
use crate::types::model::{SVINT14, UVINT32};

#[derive(Clone, Debug, PartialEq)]
pub struct EntityState {
    pub units: Units,
    pub full_update_flag: bool,
    pub entity_id: EntityId,
    pub force_id: Option<ForceId>,
    pub entity_type: Option<EntityType>,
    pub alternate_entity_type: Option<EntityType>,
    pub entity_linear_velocity: Option<LinearVelocity>,
    pub entity_location: Option<WorldCoordinates>,
    pub entity_orientation: Option<Orientation>,
    pub entity_appearance: Option<EntityAppearance>,
    pub dr_algorithm: DeadReckoningAlgorithm,
    pub dr_params_other: Option<DrOtherParameters>,
    pub dr_params_entity_linear_acceleration: Option<DrEntityLinearAcceleration>,
    pub dr_params_entity_angular_velocity: Option<AngularVelocity>,
    pub entity_marking: Option<CdisEntityMarking>,
    pub capabilities: Option<UVINT32>, // field not explicitly modeled because the interpretation depends on the EntityType, which is not yet known when that field is not present.
    pub variable_parameters: Option<Vec<CdisVariableParameter>>
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct EntityStateFieldsPresent {
    pub entity_appearance: bool,
    pub dr_params_other: bool,
    pub dr_params_entity_linear_acceleration: bool,
    pub dr_params_entity_angular_velocity: bool,
    pub entity_marking: bool,
    pub capabilities: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DrEntityLinearAcceleration {
    pub x: SVINT14,
    pub y: SVINT14,
    pub z: SVINT14,
}

impl DrEntityLinearAcceleration {
    pub fn new(x: SVINT14, y: SVINT14, z: SVINT14) -> Self {
        Self {
            x,
            y,
            z,
        }
    }
}