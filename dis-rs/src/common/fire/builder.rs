use crate::fire::model::Fire;
use crate::model::{
    DescriptorRecord, EntityId, EntityType, EventId, Location, MunitionDescriptor, VectorF32,
};

pub struct FireBuilder(Fire);

impl FireBuilder {
    #[must_use]
    pub fn new() -> Self {
        FireBuilder(Fire::default())
    }

    #[must_use]
    pub fn new_from_body(body: Fire) -> Self {
        FireBuilder(body)
    }

    #[must_use]
    pub fn build(self) -> Fire {
        self.0
    }

    #[must_use]
    pub fn with_firing_entity_id(mut self, firing_entity_id: EntityId) -> Self {
        self.0.firing_entity_id = firing_entity_id;
        self
    }

    #[must_use]
    pub fn with_target_entity_id(mut self, target_entity_id: EntityId) -> Self {
        self.0.target_entity_id = target_entity_id;
        self
    }

    #[must_use]
    pub fn with_entity_id(mut self, entity_id: EntityId) -> Self {
        self.0.entity_id = entity_id;
        self
    }

    #[must_use]
    pub fn with_event_id(mut self, event_id: EventId) -> Self {
        self.0.event_id = event_id;
        self
    }

    #[must_use]
    pub fn with_fire_mission_index(mut self, fire_mission_index: u32) -> Self {
        self.0.fire_mission_index = fire_mission_index;
        self
    }

    #[must_use]
    pub fn with_location_in_world(mut self, location_in_world: Location) -> Self {
        self.0.location_in_world = location_in_world;
        self
    }

    #[must_use]
    pub fn with_descriptor(mut self, descriptor: DescriptorRecord) -> Self {
        self.0.descriptor = descriptor;
        self
    }

    #[must_use]
    pub fn with_munition_descriptor(
        mut self,
        entity_type: EntityType,
        munition: MunitionDescriptor,
    ) -> Self {
        self.0.descriptor = DescriptorRecord::new_munition(entity_type, munition);
        self
    }

    #[must_use]
    pub fn with_expendable_descriptor(mut self, entity_type: EntityType) -> Self {
        self.0.descriptor = DescriptorRecord::Expendable { entity_type };
        self
    }

    #[must_use]
    pub fn with_velocity(mut self, velocity: VectorF32) -> Self {
        self.0.velocity = velocity;
        self
    }

    #[must_use]
    pub fn with_range(mut self, range: f32) -> Self {
        self.0.range = range;
        self
    }
}
