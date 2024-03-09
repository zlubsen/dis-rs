use crate::aggregate_state::builder::AggregateStateBuilder;
use crate::common::{BodyInfo, Interaction};
use crate::entity_state::model::EntityAppearance;
use crate::enumerations::{ForceId, PduType, AggregateStateAggregateState, AggregateStateAggregateKind, PlatformDomain, Country, AggregateStateSubcategory, AggregateStateSpecific, AggregateStateFormation, EntityMarkingCharacterSet};
use crate::model::{EntityId, EntityType, Location, Orientation, PduBody, VariableDatum, VectorF32};

/// 5.9.2.2 Aggregate State PDU
///
/// 7.8.2 Aggregate State PDU
#[derive(Debug, Default, PartialEq)]
pub struct AggregateState {
    pub aggregate_id: EntityId,
    pub force_id: ForceId,
    pub aggregate_state: AggregateStateAggregateState,
    pub aggregate_type: AggregateType,
    pub formation: AggregateStateFormation,
    pub aggregate_marking: AggregateMarking,
    pub dimensions: VectorF32,
    pub orientation: Orientation,
    pub center_of_mass: Location,
    pub velocity: VectorF32,
    pub aggregates: Vec<EntityId>,
    pub entities: Vec<EntityId>,
    pub silent_aggregate_systems: Vec<SilentAggregateSystem>,
    pub silent_entity_systems: Vec<SilentEntitySystem>,
    pub variable_datums: Vec<VariableDatum>,
}


impl AggregateState {
    pub fn builder() -> AggregateStateBuilder {
        AggregateStateBuilder::new()
    }

    pub fn into_builder(self) -> AggregateStateBuilder {
        AggregateStateBuilder::new_from_body(self)
    }

    pub fn into_pdu_body(self) -> PduBody {
        PduBody::AggregateState(self)
    }
}

impl BodyInfo for AggregateState {
    fn body_length(&self) -> u16 {

    }

    fn body_type(&self) -> PduType {
        PduType::AggregateState
    }
}

impl Interaction for AggregateState {
    fn originator(&self) -> Option<&EntityId> {
        None
    }

    fn receiver(&self) -> Option<&EntityId> {
        None
    }
}

/// 6.2.4 Aggregate Marking record
#[derive(Debug, Default, PartialEq)]
pub struct AggregateMarking {
    pub marking_character_set : EntityMarkingCharacterSet,
    pub marking_string : String, // 31 byte String
}

impl AggregateMarking {
    // TODO - builder, Display, FromStr...
}

/// 6.2.5 Aggregate Type record
#[derive(Debug, Default, PartialEq)]
pub struct AggregateType {
    pub aggregate_kind: AggregateStateAggregateKind,
    pub domain: PlatformDomain,
    pub country: Country,
    pub category: u8,
    pub sub_category: AggregateStateSubcategory,
    pub specific: AggregateStateSpecific,
    pub extra: u8,
}

impl AggregateType {
    // TODO - Display, FromStr
    pub fn with_aggregate_kind(mut self, aggregate_kind: AggregateStateAggregateKind) -> Self {
        self.aggregate_kind = aggregate_kind;
        self
    }

    pub fn with_domain(mut self, domain: PlatformDomain) -> Self {
        self.domain = domain;
        self
    }

    pub fn with_country(mut self, country: Country) -> Self {
        self.country = country;
        self
    }

    pub fn with_category(mut self, category: u8) -> Self {
        self.category = category;
        self
    }

    pub fn with_sub_category(mut self, sub_category: AggregateStateSubcategory) -> Self {
        self.sub_category = sub_category;
        self
    }

    pub fn with_specific(mut self, specific: AggregateStateSpecific) -> Self {
        self.specific = specific;
        self
    }

    pub fn with_extra(mut self, extra: u8) -> Self {
        self.extra = extra;
        self
    }
}

/// Custom record for `SilentAggregateSystem`
#[derive(Debug, Default, PartialEq)]
pub struct SilentAggregateSystem {
    pub number_of_aggregates: u16,
    pub aggregate_type: AggregateType,
}

impl SilentAggregateSystem {
    pub fn with_number_of_aggregates(mut self, number_of_aggregates: u16) -> Self {
        self.number_of_aggregates = number_of_aggregates;
        self
    }

    pub fn with_aggregate_type(mut self, aggregate_type: AggregateType) -> Self {
        self.aggregate_type = aggregate_type;
        self
    }
}

/// 6.2.79 Silent Entity System record
#[derive(Debug, Default, PartialEq)]
pub struct SilentEntitySystem {
    pub number_of_entities: u16,
    pub entity_type: EntityType,
    pub appearances: Vec<EntityAppearance>,
}

impl SilentEntitySystem {
    pub fn with_number_of_entities(mut self, number_of_entities: u16) -> Self {
        self.number_of_entities = number_of_entities;
        self
    }

    pub fn with_entity_type(mut self, entity_type: EntityType) -> Self {
        self.entity_type = entity_type;
        self
    }

    pub fn with_appearance(mut self, appearance: EntityAppearance) -> Self {
        self.appearances.push(appearance);
        self
    }

    pub fn with_appearances(mut self, appearances: Vec<EntityAppearance>) -> Self {
        self.appearances = appearances;
        self
    }
}

