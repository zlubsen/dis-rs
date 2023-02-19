use crate::common::{BodyInfo, Interaction};
use crate::constants::VARIABLE_PARAMETER_RECORD_LENGTH;
use crate::{EntityAppearance, EntityId, Location, Orientation, PduBody, PduType, VariableParameter, VectorF32};

const BASE_ENTITY_STATE_UPDATE_BODY_LENGTH : u16 = 60;

pub struct EntityStateUpdate {
    pub entity_id : EntityId, // struct
    pub entity_linear_velocity : VectorF32, // struct
    pub entity_location : Location, // struct
    pub entity_orientation : Orientation, // struct
    pub entity_appearance: EntityAppearance,
    pub variable_parameters: Vec<VariableParameter>,
}

impl EntityStateUpdate {
    pub fn new(entity_id: EntityId) -> Self {
        Self {
            entity_id,
            entity_linear_velocity: VectorF32::default(),
            entity_location: Location::default(),
            entity_orientation: Orientation::default(),
            entity_appearance: EntityAppearance::default(),
            variable_parameters: vec![]
        }
    }

    pub fn with_velocity(mut self, velocity: VectorF32) -> Self {
        self.entity_linear_velocity = velocity;
        self
    }

    pub fn with_location(mut self, location: Location) -> Self {
        self.entity_location = location;
        self
    }

    pub fn with_orientation(mut self, orientation: Orientation) -> Self {
        self.entity_orientation = orientation;
        self
    }

    pub fn with_appearance(mut self, appearance: EntityAppearance) -> Self {
        self.entity_appearance = appearance;
        self
    }

    pub fn with_variable_parameter(mut self, parameter: VariableParameter) -> Self {
        self.variable_parameters.push(parameter);
        self
    }

    pub fn with_variable_parameters(mut self, parameters: Vec<VariableParameter>) -> Self {
        self.variable_parameters = parameters;
        self
    }

    pub fn into_pdu_body(self) -> PduBody {
        PduBody::EntityStateUpdate(self)
    }
}

impl BodyInfo for EntityStateUpdate {
    fn body_length(&self) -> u16 {
        BASE_ENTITY_STATE_UPDATE_BODY_LENGTH + (VARIABLE_PARAMETER_RECORD_LENGTH * (self.variable_parameters.len() as u16))
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