use crate::constants::{FOUR_BITS, SIXTEEN_BITS, THREE_BITS};
use crate::records::model::{
    CdisRecord, EntityCoordinateVector, EntityId, LinearAcceleration, UnitsDekameters, UnitsMeters,
    WorldCoordinates,
};
use crate::types::model::{UVINT16, UVINT32, VarInt};
use crate::{BodyProperties, CdisBody, CdisInteraction};
use bitflags::bitflags;
use dis_rs::enumerations::{DeadReckoningAlgorithm, DesignatorSystemName};

#[derive(Clone, Default, Debug, PartialEq)]
pub struct Designator {
    pub units: DesignatorUnits,
    pub full_update_flag: bool,
    pub designating_entity_id: EntityId,
    pub designated_entity_id_and_spot_location_wrt_entity:
        Option<DesignatedEntityIdAndSpotLocationWrtEntity>,
    pub designator_details: Option<DesignatorDetails>,
    pub designator_spot_location: Option<WorldCoordinates>,
    pub entity_dr_and_linear_acceleration: Option<EntityDrAndLinearAcceleration>,
}

impl BodyProperties for Designator {
    type FieldsPresentFlags = DesignatorFieldsPresentFlags;
    type FieldsPresentField = u8;
    const FIELDS_PRESENT_FLAGS_LENGTH: usize = FOUR_BITS;

    fn fields_present_field(&self) -> Self::FieldsPresentField {
        let mut flags = Self::FieldsPresentFlags::empty();

        if self
            .designated_entity_id_and_spot_location_wrt_entity
            .is_some()
        {
            flags.insert(
                Self::FieldsPresentFlags::DESIGNATED_ENTITY_ID_AND_SPOT_LOCATION_WRT_ENTITY,
            );
        }
        if self.designator_details.is_some() {
            flags.insert(Self::FieldsPresentFlags::DESIGNATOR_DETAILS);
        }
        if self.designator_spot_location.is_some() {
            flags.insert(Self::FieldsPresentFlags::DESIGNATOR_SPOT_LOCATION);
        }
        if self.entity_dr_and_linear_acceleration.is_some() {
            flags.insert(Self::FieldsPresentFlags::ENTITY_DR_AND_LINEAR_ACCELERATION);
        }

        flags.bits()
    }

    fn body_length_bits(&self) -> usize {
        // 3 bits = Units + Full Update Flag
        let mut length = Self::FIELDS_PRESENT_FLAGS_LENGTH + THREE_BITS;

        length += self.designating_entity_id.record_length();

        if let Some(designator_details) = self.designator_details {
            // 16 bits = Designator System Name (aka Code Name)
            length += SIXTEEN_BITS
                + designator_details.designator_code.record_length()
                + designator_details.designator_power.record_length()
                + designator_details.designator_wavelength.record_length();
        }

        if let Some(target_details) = self.designated_entity_id_and_spot_location_wrt_entity {
            length += target_details.designated_entity_id.record_length()
                + target_details
                    .designator_spot_location_wrt_designated_entity
                    .record_length();
        }

        if let Some(spot_location) = self.designator_spot_location {
            length += spot_location.record_length();
        }

        if let Some(dr_details) = self.entity_dr_and_linear_acceleration {
            // 4 bits = DR Algorithm
            length += FOUR_BITS + dr_details.dr_entity_linear_acceleration.record_length();
        }

        length
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
        self.designated_entity_id_and_spot_location_wrt_entity
            .as_ref()
            .map(|s| &s.designated_entity_id)
    }
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct DesignatorUnits {
    pub location_wrt_entity: UnitsMeters,
    pub world_location_altitude: UnitsDekameters,
}

impl From<u8> for DesignatorUnits {
    fn from(value: u8) -> Self {
        const LOCATION_WRT_ENTITY_BIT: u8 = 0x02;
        const WORLD_LOCATION_ALTITUDE_BIT: u8 = 0x01;
        Self {
            location_wrt_entity: UnitsMeters::from((value & LOCATION_WRT_ENTITY_BIT) >> 1),
            world_location_altitude: UnitsDekameters::from(value & WORLD_LOCATION_ALTITUDE_BIT),
        }
    }
}

#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct DesignatedEntityIdAndSpotLocationWrtEntity {
    pub designated_entity_id: EntityId,
    pub designator_spot_location_wrt_designated_entity: EntityCoordinateVector,
}

impl DesignatedEntityIdAndSpotLocationWrtEntity {
    #[must_use]
    pub fn new(
        entity_id: Option<EntityId>,
        spot_location: Option<EntityCoordinateVector>,
    ) -> Option<Self> {
        Some(Self {
            designated_entity_id: entity_id?,
            designator_spot_location_wrt_designated_entity: spot_location?,
        })
    }
}

#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct DesignatorDetails {
    pub code_name: DesignatorSystemName,
    pub designator_code: UVINT16,
    pub designator_power: UVINT32,
    pub designator_wavelength: UVINT32,
}

impl DesignatorDetails {
    #[must_use]
    pub fn new(
        code_name: Option<DesignatorSystemName>,
        code: Option<UVINT16>,
        power: Option<UVINT32>,
        wavelength: Option<UVINT32>,
    ) -> Option<Self> {
        Some(Self {
            code_name: code_name?,
            designator_code: code?,
            designator_power: power?,
            designator_wavelength: wavelength?,
        })
    }
}

#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct EntityDrAndLinearAcceleration {
    pub dr_algorithm: DeadReckoningAlgorithm,
    pub dr_entity_linear_acceleration: LinearAcceleration,
}

impl EntityDrAndLinearAcceleration {
    #[must_use]
    pub fn new(
        algorithm: Option<DeadReckoningAlgorithm>,
        linear_acc: Option<LinearAcceleration>,
    ) -> Option<Self> {
        Some(Self {
            dr_algorithm: algorithm?,
            dr_entity_linear_acceleration: linear_acc?,
        })
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct DesignatorFieldsPresentFlags: <Designator as BodyProperties>::FieldsPresentField {
        const DESIGNATED_ENTITY_ID_AND_SPOT_LOCATION_WRT_ENTITY = 0x08;
        const DESIGNATOR_DETAILS = 0x04;
        const DESIGNATOR_SPOT_LOCATION = 0x02;
        const ENTITY_DR_AND_LINEAR_ACCELERATION = 0x01;
    }
}
