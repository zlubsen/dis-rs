use crate::fire::model::Fire;
use crate::model::{DescriptorRecord, EntityType, Location, MunitionDescriptor, VectorF32};

pub struct FireBuilder(Fire);

impl FireBuilder {
    pub fn new() -> Self {
        FireBuilder(Fire::default())
    }

    pub fn new_from_body(body: Fire) -> Self {
        FireBuilder(body)
    }

    pub fn build(self) -> Fire {
        self.0
    }

    pub fn with_fire_mission_index(mut self, fire_mission_index: u32) -> Self {
        self.0.fire_mission_index = fire_mission_index;
        self
    }

    pub fn with_location_in_world(mut self, location_in_world: Location) -> Self {
        self.0.location_in_world = location_in_world;
        self
    }

    pub fn with_descriptor(mut self, descriptor: DescriptorRecord) -> Self {
        self.0.descriptor = descriptor;
        self
    }

    pub fn with_munition_descriptor(mut self, entity_type: EntityType, munition: MunitionDescriptor) -> Self {
        self.0.descriptor = DescriptorRecord::new_munition(entity_type, munition);
        self
    }

    pub fn with_expendable_descriptor(mut self, entity_type: EntityType) -> Self {
        self.0.descriptor = DescriptorRecord::Expendable { entity_type };
        self
    }

    pub fn with_velocity(mut self, velocity: VectorF32) -> Self {
        self.0.velocity = velocity;
        self
    }

    pub fn with_range(mut self, range: f32) -> Self {
        self.0.range = range;
        self
    }
}
