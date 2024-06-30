use crate::{BodyProperties, CdisBody, CdisInteraction};
use crate::constants::{EIGHT_BITS, SIXTEEN_BITS};
use crate::records::model::{CdisRecord, EntityId, EntityType, LinearVelocity, UnitsDekameters, WorldCoordinates};
use crate::types::model::{UVINT32, VarInt};

#[derive(Clone, Default, Debug, PartialEq)]
pub struct Fire {
    pub units: UnitsDekameters,
    pub firing_entity_id: EntityId,
    pub target_entity_id: EntityId,
    pub munition_expandable_entity_id: EntityId,
    pub event_id: EntityId,
    pub fire_mission_index: Option<UVINT32>,
    pub location_world_coordinates: WorldCoordinates,
    pub descriptor_entity_type: EntityType,
    pub descriptor_warhead: Option<u16>,
    pub descriptor_fuze: Option<u16>,
    pub descriptor_quantity: Option<u8>,
    pub descriptor_rate: Option<u8>,
    pub velocity: LinearVelocity,
    pub range: Option<UVINT32>,
}

impl BodyProperties for Fire {
    type FieldsPresent = FireFieldsPresent;
    type FieldsPresentOutput = u8;
    const FIELDS_PRESENT_LENGTH: usize = 4;

    fn fields_present_field(&self) -> Self::FieldsPresentOutput {
        (if self.fire_mission_index.is_some() { Self::FieldsPresent::FIRE_MISSION_INDEX_BIT } else { 0 })
        | (if self.descriptor_warhead.is_some() && self.descriptor_fuze.is_some() { Self::FieldsPresent::DESCRIPTOR_WARHEAD_FUZE_BIT } else { 0 })
        | (if self.descriptor_quantity.is_some() && self.descriptor_rate.is_some() { Self::FieldsPresent::DESCRIPTOR_QUANTITY_RATE_BIT } else { 0 })
        | (if self.range.is_some() { Self::FieldsPresent::RANGE_BIT } else { 0 })
    }

    fn body_length_bits(&self) -> usize {
        const CONST_BIT_SIZE: usize = 1; // Units flag
        Self::FIELDS_PRESENT_LENGTH + CONST_BIT_SIZE
        + self.firing_entity_id.record_length()
        + self.target_entity_id.record_length()
        + self.munition_expandable_entity_id.record_length()
        + self.event_id.record_length()
        + (if let Some(record) = &self.fire_mission_index { record.record_length() } else { 0 })
        + self.location_world_coordinates.record_length()
        + self.descriptor_entity_type.record_length()
        + (if self.descriptor_warhead.is_some() { SIXTEEN_BITS } else { 0 })
        + (if self.descriptor_fuze.is_some() { SIXTEEN_BITS } else { 0 })
        + (if self.descriptor_quantity.is_some() { EIGHT_BITS } else { 0 })
        + (if self.descriptor_rate.is_some() { EIGHT_BITS } else { 0 })
        + self.velocity.record_length()
        + (if let Some(record) = &self.range { record.record_length() } else { 0 })
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

pub struct FireFieldsPresent;

impl FireFieldsPresent {
    pub const FIRE_MISSION_INDEX_BIT: u8 = 0x0008;
    pub const DESCRIPTOR_WARHEAD_FUZE_BIT: u8 = 0x0004;
    pub const DESCRIPTOR_QUANTITY_RATE_BIT: u8 = 0x0002;
    pub const RANGE_BIT: u8 = 0x0001;
}