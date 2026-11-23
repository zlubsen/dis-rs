use crate::constants::{FOUR_BITS, ONE_BIT};
use crate::records::model::{
    CdisRecord, DescriptorQuantityRate, DescriptorWarheadFuse, EntityId, EntityType,
    LinearVelocity, UnitsDekameters, WorldCoordinates,
};
use crate::types::model::{UVINT32, VarInt};
use crate::{BodyProperties, CdisBody, CdisInteraction};
use bitflags::bitflags;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct Fire {
    pub units: UnitsDekameters,
    pub firing_entity_id: EntityId,
    pub target_entity_id: EntityId,
    pub munition_expendable_entity_id: EntityId,
    pub event_id: EntityId,
    pub fire_mission_index: Option<UVINT32>,
    pub location_world_coordinates: WorldCoordinates,
    pub descriptor_entity_type: EntityType,
    pub descriptor_warhead_fuse: Option<DescriptorWarheadFuse>,
    pub descriptor_quantity_rate: Option<DescriptorQuantityRate>,
    pub velocity: LinearVelocity,
    pub range: Option<UVINT32>,
}

impl BodyProperties for Fire {
    type FieldsPresentFlags = FireFieldsPresentFlags;
    type FieldsPresentField = u8;
    const FIELDS_PRESENT_FLAGS_LENGTH: usize = FOUR_BITS;

    fn fields_present_field(&self) -> Self::FieldsPresentField {
        let mut flags = Self::FieldsPresentFlags::empty();

        if self.fire_mission_index.is_some() {
            flags.insert(Self::FieldsPresentFlags::FIRE_MISSION_INDEX);
        }
        if self.descriptor_warhead_fuse.is_some() {
            flags.insert(Self::FieldsPresentFlags::DESCRIPTOR_WARHEAD_FUSE);
        }
        if self.descriptor_quantity_rate.is_some() {
            flags.insert(Self::FieldsPresentFlags::DESCRIPTOR_QUANTITY_RATE);
        }
        if self.range.is_some() {
            flags.insert(Self::FieldsPresentFlags::RANGE);
        }

        flags.bits()
    }

    fn body_length_bits(&self) -> usize {
        // 1 bit = Units
        let mut length = Self::FIELDS_PRESENT_FLAGS_LENGTH + ONE_BIT;

        length += self.firing_entity_id.record_length()
            + self.target_entity_id.record_length()
            + self.munition_expendable_entity_id.record_length()
            + self.event_id.record_length();

        if let Some(fire_mission_index) = self.fire_mission_index {
            length += fire_mission_index.record_length();
        }

        length += self.location_world_coordinates.record_length()
            + self.descriptor_entity_type.record_length();

        if let Some(descriptor_warhead_fuse) = self.descriptor_warhead_fuse {
            length += descriptor_warhead_fuse.record_length();
        }

        if let Some(descriptor_quantity_rate) = self.descriptor_quantity_rate {
            length += descriptor_quantity_rate.record_length();
        }

        length += self.velocity.record_length();

        if let Some(range) = self.range {
            length += range.record_length();
        }

        length
    }

    fn into_cdis_body(self) -> CdisBody {
        CdisBody::Fire(self)
    }
}

impl CdisInteraction for Fire {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.firing_entity_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        Some(&self.target_entity_id)
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct FireFieldsPresentFlags: <Fire as BodyProperties>::FieldsPresentField {
        const FIRE_MISSION_INDEX = 0x08;
        const DESCRIPTOR_WARHEAD_FUSE = 0x04;
        const DESCRIPTOR_QUANTITY_RATE = 0x02;
        const RANGE = 0x01;
    }
}
