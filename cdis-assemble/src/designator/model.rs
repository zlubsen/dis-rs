use dis_rs::enumerations::{DeadReckoningAlgorithm, DesignatorSystemName};
use crate::{BodyProperties, CdisBody, CdisInteraction};
use crate::constants::{FOUR_BITS, SIXTEEN_BITS};
use crate::records::model::{CdisRecord, EntityCoordinateVector, EntityId, LinearAcceleration, UnitsDekameters, UnitsMeters, WorldCoordinates};
use crate::types::model::{UVINT16, UVINT32, VarInt};

#[derive(Clone, Default, Debug, PartialEq)]
pub struct Designator {
    pub units: DesignatorUnits,
    pub full_update_flag: bool,
    pub designating_entity_id: EntityId,
    pub code_name: Option<DesignatorSystemName>,
    pub designated_entity_id: Option<EntityId>,
    pub designator_code: Option<UVINT16>,
    pub designator_power: Option<UVINT32>,
    pub designator_wavelength: Option<UVINT32>,
    pub spot_wrt_designated_entity: Option<EntityCoordinateVector>,
    pub designator_spot_location: Option<WorldCoordinates>,
    pub dr_algorithm: Option<DeadReckoningAlgorithm>,
    pub dr_entity_linear_acceleration: Option<LinearAcceleration>,
}

impl BodyProperties for Designator {
    type FieldsPresent = DesignatorFieldsPresent;
    type FieldsPresentOutput = u8;
    const FIELDS_PRESENT_LENGTH: usize = 4;

    fn fields_present_field(&self) -> Self::FieldsPresentOutput {
        (if self.designated_entity_id.is_some() && self.spot_wrt_designated_entity.is_some() {
            Self::FieldsPresent::DESIGNATED_ENTITY_ID_AND_SPOT_LOCATION_WRT_ENTITY_BIT
        } else { 0 })
        | (if self.code_name.is_some() && self.designator_code.is_some() && self.designator_power.is_some() && self.designator_wavelength.is_some() {
            Self::FieldsPresent::DESIGNATOR_DETAILS_BIT } else { 0 })
        | (if self.designator_spot_location.is_some() { Self::FieldsPresent::DESIGNATOR_SPOT_LOCATION_BIT } else { 0 })
        | (if self.dr_algorithm.is_some() & self.dr_entity_linear_acceleration.is_some() { Self::FieldsPresent::ENTITY_DR_AND_LINEAR_ACCELERATION_BIT } else { 0 })
    }

    fn body_length_bits(&self) -> usize {
        const CONST_BIT_SIZE: usize = 3; // units flags, full update flag
        Self::FIELDS_PRESENT_LENGTH + CONST_BIT_SIZE
            + self.designating_entity_id.record_length()
            + if self.code_name.is_some() { SIXTEEN_BITS } else { 0 }
            + if let Some(record) = self.designated_entity_id { record.record_length() } else { 0 }
            + if let Some(record) = self.designator_code { record.record_length() } else { 0 }
            + if let Some(record) = self.designator_power { record.record_length() } else { 0 }
            + if let Some(record) = self.designator_wavelength { record.record_length() } else { 0 }
            + if let Some(record) = self.spot_wrt_designated_entity { record.record_length() } else { 0 }
            + if let Some(record) = self.designator_spot_location { record.record_length() } else { 0 }
            + if self.dr_algorithm.is_some() { FOUR_BITS } else { 0 }
            + if let Some(record) = self.dr_entity_linear_acceleration { record.record_length() } else { 0 }
    }

    fn into_cdis_body(self) -> CdisBody {
        CdisBody::Designator(self)
    }
}

impl CdisInteraction for Designator {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.designating_entity_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        self.designated_entity_id.as_ref()
    }
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct DesignatorUnits {
    pub location_wrt_entity_units: UnitsMeters,
    pub world_location_altitude: UnitsDekameters,
}

impl From<u8> for DesignatorUnits {
    fn from(value: u8) -> Self {
        const LOCATION_WRT_ENTITY_BIT: u8 = 0x02;
        const WORLD_LOCATION_ALTITUDE_BIT: u8 = 0x01;
        Self {
            location_wrt_entity_units: UnitsMeters::from((value & LOCATION_WRT_ENTITY_BIT) >> 1),
            world_location_altitude: UnitsDekameters::from(value & WORLD_LOCATION_ALTITUDE_BIT),
        }
    }
}

pub struct DesignatorFieldsPresent;

impl DesignatorFieldsPresent {
    pub const DESIGNATED_ENTITY_ID_AND_SPOT_LOCATION_WRT_ENTITY_BIT: u8 = 0x08;
    pub const DESIGNATOR_DETAILS_BIT: u8 = 0x04;
    pub const DESIGNATOR_SPOT_LOCATION_BIT: u8 = 0x02;
    pub const ENTITY_DR_AND_LINEAR_ACCELERATION_BIT: u8 = 0x01;
}