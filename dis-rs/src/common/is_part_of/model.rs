use crate::common::{BodyInfo, Interaction};
use crate::enumerations::{IsPartOfNature, IsPartOfPosition, PduType, StationName};
use crate::is_part_of::builder::IsPartOfBuilder;
use crate::model::{EntityId, PduBody, VectorF32};

const IS_PART_OF_BODY_LENGTH: u16 = 40;

/// 5.9.5 IsPartOf PDU
///
/// 7.8.5 IsPartOf PDU
#[derive(Debug, Default, PartialEq)]
pub struct IsPartOf {
    pub originating_simulation_id: EntityId,
    pub receiving_entity_id: EntityId,
    pub relationship: Relationship,
    pub part_location: VectorF32,
    pub named_location_id: NamedLocationId,
}

impl IsPartOf {
    pub fn builder() -> IsPartOfBuilder {
        IsPartOfBuilder::new()
    }

    pub fn into_builder(self) -> IsPartOfBuilder {
        IsPartOfBuilder::new_from_body(self)
    }

    pub fn into_pdu_body(self) -> PduBody {
        PduBody::IsPartOf(self)
    }
}

impl BodyInfo for IsPartOf {
    fn body_length(&self) -> u16 {
        IS_PART_OF_BODY_LENGTH
    }

    fn body_type(&self) -> PduType {
        PduType::IsPartOf
    }
}

impl Interaction for IsPartOf {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.originating_simulation_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        Some(&self.receiving_entity_id)
    }
}

/// 6.2.74 Relationship record
#[derive(Debug, Default, PartialEq)]
pub struct Relationship {
    pub nature: IsPartOfNature,
    pub position: IsPartOfPosition,
}

impl Relationship {
    pub fn with_nature(mut self, nature: IsPartOfNature) -> Self {
        self.nature = nature;
        self
    }

    pub fn with_position(mut self, position: IsPartOfPosition) -> Self {
        self.position = position;
        self
    }
}

/// 6.2.62 Named Location Identification record
#[derive(Debug, Default, PartialEq)]
pub struct NamedLocationId {
    pub station_name: StationName,
    pub station_number: u16,
}

impl NamedLocationId {
    pub fn with_station_name(mut self, station_name: StationName) -> Self {
        self.station_name = station_name;
        self
    }

    pub fn with_station_number(mut self, station_number: u16) -> Self {
        self.station_number = station_number;
        self
    }
}