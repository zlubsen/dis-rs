use crate::common::{BodyInfo, Interaction};
use crate::common::model::{DescriptorRecord, EntityId, EventId, Location, VectorF32};
use crate::{EntityType, MunitionDescriptor, PduBody};
use crate::enumerations::PduType;

const FIRE_BODY_LENGTH : u16 = 28;

#[derive(Debug, PartialEq)]
pub struct Fire {
    pub firing_entity_id : EntityId,
    pub target_entity_id : EntityId,
    pub entity_id: EntityId,
    pub event_id : EventId,
    pub fire_mission_index : u32,
    pub location_in_world : Location,
    pub descriptor: DescriptorRecord,
    pub velocity : VectorF32,
    pub range : f32,
}

impl Fire {
    pub fn new(firing_entity_id: EntityId, target_entity_id: EntityId, entity_id: EntityId, event_id: EventId) -> Self {
        Self {
            firing_entity_id,
            target_entity_id,
            entity_id,
            event_id,
            fire_mission_index: 0,
            location_in_world: Default::default(),
            descriptor: DescriptorRecord::default(),
            velocity: Default::default(),
            range: 0.0
        }
    }

    pub fn with_fire_mission_index(mut self, fire_mission_index: u32) -> Self {
        self.fire_mission_index = fire_mission_index;
        self
    }

    pub fn with_location_in_world(mut self, location_in_world: Location) -> Self {
        self.location_in_world = location_in_world;
        self
    }

    pub fn with_descriptor(mut self, descriptor: DescriptorRecord) -> Self {
        self.descriptor = descriptor;
        self
    }

    pub fn with_munition_descriptor(mut self, entity_type: EntityType, munition: MunitionDescriptor) -> Self {
        self.descriptor = DescriptorRecord::new_munition(entity_type, munition);
        self
    }

    pub fn with_expendable_descriptor(mut self, entity_type: EntityType) -> Self {
        self.descriptor = DescriptorRecord::Expendable { entity_type };
        self
    }

    pub fn with_velocity(mut self, velocity: VectorF32) -> Self {
        self.velocity = velocity;
        self
    }

    pub fn with_range(mut self, range: f32) -> Self {
        self.range = range;
        self
    }

    pub fn into_pdu_body(self) -> PduBody {
        PduBody::Fire(self)
    }
}

impl BodyInfo for Fire {
    fn body_length(&self) -> u16 {
        FIRE_BODY_LENGTH
    }

    fn body_type(&self) -> PduType {
        PduType::Fire
    }
}

impl Interaction for Fire {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.firing_entity_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        Some(&self.target_entity_id)
    }
}