use crate::common::entity_state::model::{DrParameters, EntityMarking, EntityState, EntityStateValidationError, VariableParameter};
use crate::common::model::{EntityId, EntityType, Location, Orientation, Pdu, PduBody, PduHeader, SimulationAddress, VectorF32};
use crate::EntityAppearance;
use crate::enumerations::{EntityMarkingCharacterSet, ForceId};
use crate::v6::entity_state::model::EntityCapabilities;

pub struct EntityStateBuilder {
    entity_id : Option<EntityId>,
    force_id : Option<ForceId>,
    entity_type : Option<EntityType>,
    alternative_entity_type : Option<EntityType>,
    entity_linear_velocity : Option<VectorF32>,
    entity_location : Option<Location>,
    entity_orientation : Option<Orientation>,
    entity_appearance : Option<EntityAppearance>,
    dead_reckoning_parameters : Option<DrParameters>,
    entity_marking : Option<EntityMarking>,
    entity_capabilities : Option<EntityCapabilities>,
    variable_parameters: Vec<VariableParameter>,
}

impl EntityStateBuilder {
    pub fn new() -> EntityStateBuilder {
        EntityStateBuilder {
            entity_id: None,
            force_id: None,
            entity_type: None,
            alternative_entity_type: None,
            entity_linear_velocity: None,
            entity_location: None,
            entity_orientation: None,
            entity_appearance: None,
            dead_reckoning_parameters: None,
            entity_marking: None,
            entity_capabilities: None,
            variable_parameters: vec![]
        }
    }

    pub fn entity_id(mut self, entity_id: EntityId) -> Self {
        self.entity_id = Some(entity_id);
        self
    }

    pub fn entity_id_triplet(mut self, site_id: u16, application_id: u16, entity_id: u16) -> Self {
        self.entity_id = Some(EntityId {
            simulation_address: SimulationAddress {
                site_id,
                application_id
            },
            entity_id
        });
        self
    }

    pub fn force_id(mut self, force_id: ForceId) -> Self {
        self.force_id = Some(force_id);
        self
    }

    pub fn entity_type(mut self, entity_type: EntityType) -> Self {
        self.entity_type = Some(entity_type);
        self
    }

    pub fn alt_entity_type(mut self, entity_type: EntityType) -> Self {
        self.alternative_entity_type = Some(entity_type);
        self
    }

    pub fn linear_velocity(mut self, velocity : VectorF32) -> Self {
        self.entity_linear_velocity = Some(velocity);
        self
    }

    pub fn linear_velocity_from_components(mut self, first_vector_component: f32, second_vector_component: f32, third_vector_component: f32) -> Self {
        self.entity_linear_velocity = Some(VectorF32 {
            first_vector_component,
            second_vector_component,
            third_vector_component,
        });
        self
    }

    pub fn location(mut self, location : Location) -> Self {
        self.entity_location = Some(location);
        self
    }

    pub fn location_from_coordinates(mut self, x_coordinate : f64, y_coordinate : f64, z_coordinate : f64) -> Self {
        self.entity_location = Some(Location {
            x_coordinate,
            y_coordinate,
            z_coordinate,
        });
        self
    }

    pub fn orientation(mut self, orientation : Orientation) -> Self {
        self.entity_orientation = Some(orientation);
        self
    }

    pub fn location_from_angles(mut self, psi : f32, theta : f32, phi : f32) -> Self {
        self.entity_orientation = Some(Orientation {
            psi,
            theta,
            phi,
        });
        self
    }

    pub fn appearance(mut self, appearance: EntityAppearance) -> Self {
        // TODO
        self.entity_appearance = Some(appearance);
        self
    }

    pub fn dead_reckoning(mut self, parameters : DrParameters) -> Self {
        self.dead_reckoning_parameters = Some(parameters);
        self
    }

    // TODO dead_reckoning building with separate variables

    pub fn marking(mut self, marking : EntityMarking) -> Self {
        self.entity_marking = Some(marking);
        self
    }

    pub fn marking_from_string_ascii(mut self, marking : String) -> Self {
        self.entity_marking = Some(EntityMarking {
            marking_character_set : EntityMarkingCharacterSet::ASCII,
            marking_string: marking.to_ascii_uppercase(),
        });
        self
    }

    pub fn capabilities(mut self, capabilities : EntityCapabilities) -> Self {
        self.entity_capabilities = Some(capabilities);
        self
    }

    pub fn capabilities_flags(mut self,
                              ammunition_supply : bool,
                              fuel_supply : bool,
                              recovery : bool,
                              repair : bool) -> Self {
        self.entity_capabilities = Some(EntityCapabilities {
            ammunition_supply,
            fuel_supply,
            recovery,
            repair,
        });
        self
    }

    pub fn add_articulation_parameters_vec(mut self, parameters : Vec<VariableParameter>) -> Self {
        self.variable_parameters = parameters;
        self
    }

    pub fn add_articulation_parameter(mut self, parameter : VariableParameter) -> Self {
        self.variable_parameters.push(parameter);
        self
    }

    // TODO fn to more simply add an attached part
    // TODO fn to more simply add an articulated part

    fn validate(&self) -> Result<(), EntityStateValidationError> {
        if self.entity_id.is_some() &&
            self.force_id.is_some() &&
            self.entity_type.is_some() &&
            self.alternative_entity_type.is_some() &&
            self.entity_linear_velocity.is_some() &&
            self.entity_location.is_some() &&
            self.entity_orientation.is_some() &&
            self.entity_appearance.is_some() &&
            self.dead_reckoning_parameters.is_some() &&
            self.entity_marking.is_some() &&
            self.entity_capabilities.is_some() {
            Ok(())
        } else { Err(EntityStateValidationError::SomeFieldNotOkError )}
    }

    pub fn build(self) -> Result<PduBody, ()> { // TODO sane error type
        if self.validate().is_err() {
            return Err(())
        }

        Ok(PduBody::EntityState(EntityState {
            entity_id: self.entity_id.expect("Value expected, but not found."),
            force_id: self.force_id.expect("Value expected, but not found."),
            entity_type: self.entity_type.expect("Value expected, but not found."),
            alternative_entity_type: self.alternative_entity_type.expect("Value expected, but not found."),
            entity_linear_velocity: self.entity_linear_velocity.expect("Value expected, but not found."),
            entity_location: self.entity_location.expect("Value expected, but not found."),
            entity_orientation: self.entity_orientation.expect("Value expected, but not found."),
            entity_appearance: self.entity_appearance.expect("Value expected, but not found."),
            dead_reckoning_parameters: self.dead_reckoning_parameters.expect("Value expected, but not found."),
            entity_marking: self.entity_marking.expect("Value expected, but not found."),
            entity_capabilities: self.entity_capabilities.expect("Value expected, but not found.").into(),
            variable_parameters: if !self.variable_parameters.is_empty() { self.variable_parameters } else { vec![] },
        }))
    }

    pub fn build_with_header(self, header: PduHeader) -> Result<Pdu, ()> { // TODO sane error type
        if self.validate().is_err() {
            return Err(())
        }

        Ok(Pdu{
            header,
            body : self.build()?})
    }
}