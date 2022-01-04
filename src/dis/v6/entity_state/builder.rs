use crate::dis::v6::entity_state::model::*;
use crate::dis::v6::model::PduHeader;

pub struct EntityStateBuilder {
    header : Option<PduHeader>,
    entity_id : Option<EntityId>,
    force_id : Option<ForceId>,
    entity_type : Option<EntityType>,
    alternative_entity_type : Option<EntityType>,
    entity_linear_velocity : Option<VectorF32>,
    entity_location : Option<Location>,
    entity_orientation : Option<Orientation>,
    entity_appearance : Option<Appearance>,
    dead_reckoning_parameters : Option<DrParameters>,
    entity_marking : Option<EntityMarking>,
    entity_capabilities : Option<EntityCapabilities>,
    articulation_parameter : Vec<ArticulationParameter>,
}

impl EntityStateBuilder {
    pub(crate) fn new() -> EntityStateBuilder {
        EntityStateBuilder {
            header: None,
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
            articulation_parameter: vec![]
        }
    }

    pub fn header(mut self, header: PduHeader) -> Self {
        self.header = Some(header);
        self
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

    fn validate(&self) -> Result<(), EntityStateValidationError> {
        // TODO "Check if all fields are filled in properly; required are set, valid values..., based on the standard"

        return if self.header.is_some() &&
            self.entity_id.is_some() &&
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

    pub fn build(self) -> Result<EntityState, ()> { // TODO sane error type
        if let Err(err) = self.validate() {
            return Err(())
        }

        Ok(EntityState {
            header: self.header.expect("Value expected, but not found."),
            entity_id: self.entity_id.expect("Value expected, but not found."),
            force_id: self.force_id.expect("Value expected, but not found."),
            articulated_parts_no: self.articulation_parameter.len() as u8,
            entity_type: self.entity_type.expect("Value expected, but not found."),
            alternative_entity_type: self.alternative_entity_type.expect("Value expected, but not found."),
            entity_linear_velocity: self.entity_linear_velocity.expect("Value expected, but not found."),
            entity_location: self.entity_location.expect("Value expected, but not found."),
            entity_orientation: self.entity_orientation.expect("Value expected, but not found."),
            entity_appearance: self.entity_appearance.expect("Value expected, but not found."),
            dead_reckoning_parameters: self.dead_reckoning_parameters.expect("Value expected, but not found."),
            entity_marking: self.entity_marking.expect("Value expected, but not found."),
            entity_capabilities: self.entity_capabilities.expect("Value expected, but not found."),
            articulation_parameter: if self.articulation_parameter.is_empty() { Some(self.articulation_parameter) } else { None },
        })
    }
}