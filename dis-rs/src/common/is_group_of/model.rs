use crate::common::{BodyInfo, Interaction};
use crate::entity_state::model::EntityAppearance;
use crate::enumerations::{IsGroupOfGroupedEntityCategory, PduType};
use crate::is_group_of::builder::IsGroupOfBuilder;
use crate::model::{EntityId, PduBody};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

const BASE_IS_GROUP_OF_BODY_LENGTH: u16 = 28;

/// 5.9.3 `IsGroupOf` PDU
///
/// 7.8.3 `IsGroupOf` PDU
///
/// The `Vec` `groups` of `GroupEntityDescription` must be of the
/// same enum value as indicated by `grouped_entity_category`.
/// This is not enforced and thus left up to the user.
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct IsGroupOf {
    pub group_id: EntityId,
    pub grouped_entity_category: IsGroupOfGroupedEntityCategory,
    pub group_reference_point: GroupReferencePoint,
    pub descriptions: Vec<GroupEntityDescription>,
}

impl IsGroupOf {
    #[must_use]
    pub fn builder() -> IsGroupOfBuilder {
        IsGroupOfBuilder::new()
    }

    #[must_use]
    pub fn into_builder(self) -> IsGroupOfBuilder {
        IsGroupOfBuilder::new_from_body(self)
    }

    #[must_use]
    pub fn into_pdu_body(self) -> PduBody {
        PduBody::IsGroupOf(self)
    }
}

impl BodyInfo for IsGroupOf {
    fn body_length(&self) -> u16 {
        BASE_IS_GROUP_OF_BODY_LENGTH
            + self
                .descriptions
                .iter()
                .map(GroupEntityDescription::record_length)
                .sum::<u16>()
    }

    fn body_type(&self) -> PduType {
        PduType::IsGroupOf
    }
}

impl Interaction for IsGroupOf {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.group_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        None
    }
}

impl From<IsGroupOf> for PduBody {
    #[inline]
    fn from(value: IsGroupOf) -> Self {
        value.into_pdu_body()
    }
}

/// Custom defined record.
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GroupReferencePoint {
    pub latitude: f64,
    pub longitude: f64,
}

impl GroupReferencePoint {
    #[must_use]
    pub fn with_latitude(mut self, latitude: f64) -> Self {
        self.latitude = latitude;
        self
    }

    #[must_use]
    pub fn with_longitude(mut self, longitude: f64) -> Self {
        self.longitude = longitude;
        self
    }

    #[must_use]
    pub const fn record_length(&self) -> u16 {
        16
    }
}

/// Wrapper enum for UID 213 and the respective
/// Group Entity Description (GED) records
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum GroupEntityDescription {
    #[default]
    Undefined,
    BasicGroundCombatVehicle(GEDRecord1),
    EnhancedGroundCombatVehicle(GEDRecord2),
    BasicGroundCombatSoldier(GEDRecord3),
    EnhancedGroundCombatSoldier(GEDRecord4),
    BasicRotorWingAircraft(GEDRecord5),
    EnhancedRotorWingAircraft(GEDRecord6),
    BasicFixedWingAircraft(GEDRecord7),
    EnhancedFixedWingAircraft(GEDRecord8),
    GroundLogisticsVehicle(GEDRecord9),
}

impl GroupEntityDescription {
    #[must_use]
    pub const fn record_length(&self) -> u16 {
        match self {
            GroupEntityDescription::Undefined => 0,
            GroupEntityDescription::BasicGroundCombatVehicle(ged) => ged.record_length(),
            GroupEntityDescription::EnhancedGroundCombatVehicle(ged) => ged.record_length(),
            GroupEntityDescription::BasicGroundCombatSoldier(ged) => ged.record_length(),
            GroupEntityDescription::EnhancedGroundCombatSoldier(ged) => ged.record_length(),
            GroupEntityDescription::BasicRotorWingAircraft(ged) => ged.record_length(),
            GroupEntityDescription::EnhancedRotorWingAircraft(ged) => ged.record_length(),
            GroupEntityDescription::BasicFixedWingAircraft(ged) => ged.record_length(),
            GroupEntityDescription::EnhancedFixedWingAircraft(ged) => ged.record_length(),
            GroupEntityDescription::GroundLogisticsVehicle(ged) => ged.record_length(),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GEDEntityLocation {
    pub x_offset: u16,
    pub y_offset: u16,
    pub z_offset: u16,
}

impl GEDEntityLocation {
    #[must_use]
    pub const fn record_length(&self) -> u16 {
        6
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GEDEntityOrientation {
    pub psi: u8,
    pub theta: u8,
    pub phi: u8,
}

impl GEDEntityOrientation {
    #[must_use]
    pub const fn record_length(&self) -> u16 {
        3
    }
}

/// UID 215
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GEDRecord1 {
    pub entity_id: u16,
    pub location: GEDEntityLocation,
    pub appearance: EntityAppearance,
    pub orientation: GEDEntityOrientation,
    pub speed: u8,
    pub turret_azimuth: u8,
    pub gun_elevation: u8,
    pub turret_slew_rate: u8,
    pub gun_elevation_rate: u8,
}

impl GEDRecord1 {
    #[must_use]
    pub const fn record_length(&self) -> u16 {
        self.location.record_length()
            + self.orientation.record_length()
            + self.appearance.record_length()
            + 7
    }
}

/// UID 216
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GEDRecord2 {
    pub basic_ground_combat_vehicle: GEDRecord1,
    pub fuel_status: u8,
    pub ground_maintenance_status: u8,
    pub primary_ammunition: u8,
    pub secondary_ammunition: u8,
}

impl GEDRecord2 {
    #[must_use]
    pub const fn record_length(&self) -> u16 {
        self.basic_ground_combat_vehicle.record_length() + 4
    }
}

/// UID 217
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GEDRecord3 {
    pub entity_id: u16,
    pub location: GEDEntityLocation,
    pub appearance: EntityAppearance,
    pub orientation: GEDEntityOrientation,
    pub speed: u8,
    pub head_azimuth: u8,
    pub head_elevation: u8,
    pub head_scan_rate: u8,
    pub head_elevation_rate: u8,
}

impl GEDRecord3 {
    #[must_use]
    pub const fn record_length(&self) -> u16 {
        self.location.record_length()
            + self.orientation.record_length()
            + self.appearance.record_length()
            + 7
    }
}

/// UID 218
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GEDRecord4 {
    pub basic_ground_combat_soldier: GEDRecord3,
    pub water_status: u8,
    pub reset_status: u8,
    pub primary_ammunition: u8,
    pub secondary_ammunition: u8,
}

impl GEDRecord4 {
    #[must_use]
    pub const fn record_length(&self) -> u16 {
        self.basic_ground_combat_soldier.record_length() + 4
    }
}

/// UID 219
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GEDRecord5 {
    pub entity_id: u16,
    pub location: GEDEntityLocation,
    pub appearance: EntityAppearance,
    pub orientation: GEDEntityOrientation,
    pub fuel_status: u8,
    pub movement_horizontal_deviation: u8,
    pub movement_vertical_deviation: u8,
    pub movement_speed: u16,
    pub turret_azimuth: u8,
    pub gun_elevation: u8,
    pub turret_scan_rate: u8,
    pub gun_elevation_rate: u8,
}

impl GEDRecord5 {
    #[must_use]
    pub const fn record_length(&self) -> u16 {
        self.location.record_length()
            + self.orientation.record_length()
            + self.appearance.record_length()
            + 11
    }
}

/// UID 220
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GEDRecord6 {
    pub basic_rotor_wing_aircraft: GEDRecord5,
    pub supplemental_fuel_status: u8,
    pub air_maintenance_status: u8,
    pub primary_ammunition: u8,
    pub secondary_ammunition: u8,
}

impl GEDRecord6 {
    #[must_use]
    pub const fn record_length(&self) -> u16 {
        self.basic_rotor_wing_aircraft.record_length() + 4
    }
}

/// UID 221
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GEDRecord7 {
    pub entity_id: u16,
    pub location: GEDEntityLocation,
    pub appearance: EntityAppearance,
    pub orientation: GEDEntityOrientation,
    pub fuel_status: u8,
    pub movement_horizontal_deviation: u8,
    pub movement_vertical_deviation: u8,
    pub movement_speed: u16,
}

impl GEDRecord7 {
    #[must_use]
    pub const fn record_length(&self) -> u16 {
        self.location.record_length()
            + self.orientation.record_length()
            + self.appearance.record_length()
            + 7
    }
}

/// UID 222
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GEDRecord8 {
    pub basic_fixed_wing_aircraft: GEDRecord7,
    pub supplemental_fuel_status: u8,
    pub air_maintenance_status: u8,
    pub primary_ammunition: u8,
    pub secondary_ammunition: u8,
}

impl GEDRecord8 {
    #[must_use]
    pub const fn record_length(&self) -> u16 {
        self.basic_fixed_wing_aircraft.record_length() + 4
    }
}

/// UID 223
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GEDRecord9 {
    pub entity_id: u16,
    pub location: GEDEntityLocation,
    pub appearance: EntityAppearance,
    pub orientation: GEDEntityOrientation,
    pub speed: u16,
}

impl GEDRecord9 {
    #[must_use]
    pub const fn record_length(&self) -> u16 {
        self.location.record_length()
            + self.orientation.record_length()
            + self.appearance.record_length()
            + 4
    }
}
