use std::mem::size_of;
use crate::dis::v6::pdu::*;
use crate::dis::v6::pdu::entity_state::*;
use crate::dis::v6::pdu::PduType::EntityState;

pub struct EntityStateBuilder {
    header : PduHeader, // struct
    entity_id : EntityId, // struct
    force_id : ForceId, // enum
    articulated_parts_no : u8, // FIXME can be obtained from length of articulation_parameter field
    entity_type : EntityType, // struct
    alternative_entity_type : EntityType, // struct
    entity_linear_velocity : VectorF32, // struct
    entity_location : Location, // struct
    entity_orientation : Orientation, // struct
    entity_appearance : Appearance, // struct
    dead_reckoning_parameters : DrParameters, // struct
    entity_marking : EntityMarking, // struct
    entity_capabilities : EntityCapabilities, // struct
    articulation_parameter : Option<List<ArticulationParameter>>, // optional list of records
}

impl EntityStateBuilder {
    fn build() -> EntityState {
        size_of();
        EntityState {
            header: PduHeader {},
            entity_id: (),
            force_id: ForceId::Other,
            articulated_parts_no: 0,
            entity_type: (),
            alternative_entity_type: (),
            entity_linear_velocity: (),
            entity_location: (),
            entity_orientation: (),
            entity_appearance: (),
            dead_reckoning_parameters: (),
            entity_marking: (),
            entity_capabilities: (),
            articulation_parameter: None
        }
    }
}