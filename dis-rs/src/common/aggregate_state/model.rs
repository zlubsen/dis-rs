use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::aggregate_state::builder::AggregateStateBuilder;
use crate::common::{BodyInfo, Interaction};
use crate::constants::{EIGHT_OCTETS, FOUR_OCTETS, THIRTY_TWO_OCTETS, TWO_OCTETS};
use crate::DisError;
use crate::entity_state::model::EntityAppearance;
use crate::enumerations::{ForceId, PduType, AggregateStateAggregateState, AggregateStateAggregateKind, PlatformDomain, Country, AggregateStateSubcategory, AggregateStateSpecific, AggregateStateFormation, EntityMarkingCharacterSet};
use crate::model::{BASE_VARIABLE_DATUM_LENGTH, EntityId, EntityType, length_padded_to_num, Location, Orientation, PduBody, VariableDatum, VectorF32};

pub(crate) const BASE_AGGREGATE_STATE_BODY_LENGTH: u16 = 124;

/// 5.9.2.2 Aggregate State PDU
///
/// 7.8.2 Aggregate State PDU
#[derive(Clone, Debug, Default, PartialEq)]
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

/// Calculate the intermediate length and padding of an AggregateState PDU.
///
/// Returns a tuple consisting of the intermediate length including the padding,
/// and the length of the padding, in octets.
pub(crate) fn aggregate_state_intermediate_length_padding(aggregates: &Vec<EntityId>, entities: &Vec<EntityId>) -> (u16, u16) {
    let intermediate_length = BASE_AGGREGATE_STATE_BODY_LENGTH
        + aggregates.iter().map(|id| id.record_length() ).sum::<u16>() // number of aggregate ids
        + entities.iter().map(|id| id.record_length() ).sum::<u16>();  // number of entity ids
    let padding_length = intermediate_length % (FOUR_OCTETS as u16);       // padding to 32-bits (4 octets) boundary
    (intermediate_length + padding_length, padding_length)
}

impl BodyInfo for AggregateState {
    fn body_length(&self) -> u16 {
        let (intermediate_length, _padding_length) = aggregate_state_intermediate_length_padding(&self.aggregates, &self.entities);
        intermediate_length
            // number of silent aggregate systems
            + self.silent_aggregate_systems.iter().map(|system| system.record_length() ).sum::<u16>()
            // number of silent entity systems
            + self.silent_entity_systems.iter().map(|system| system.record_length() ).sum::<u16>()
            // number of variable datum records
            + (self.variable_datums.iter().map(|datum| {
                let padded_record = length_padded_to_num(
                    BASE_VARIABLE_DATUM_LENGTH as usize + datum.datum_value.len(),
                    EIGHT_OCTETS);
                padded_record.record_length as u16
            } ).sum::<u16>())
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
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AggregateMarking {
    pub marking_character_set : EntityMarkingCharacterSet,
    pub marking_string : String, // 31 byte String
}

impl AggregateMarking {
    pub fn new(marking: String, character_set: EntityMarkingCharacterSet) -> Self {
        Self {
            marking_character_set: character_set,
            marking_string: marking
        }
    }

    pub fn new_ascii<S: Into<String>>(marking: S) -> Self {
        AggregateMarking::new(marking.into(), EntityMarkingCharacterSet::ASCII)
    }

    pub fn with_marking<S: Into<String>>(mut self, marking: S) -> Self {
        self.marking_string = marking.into();
        self
    }

    pub fn record_length(&self) -> u16 {
        THIRTY_TWO_OCTETS as u16
    }
}

impl Display for AggregateMarking {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.marking_string.as_str())
    }
}

impl FromStr for AggregateMarking {
    type Err = DisError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() <= 31 {
            Ok(Self {
                marking_character_set: EntityMarkingCharacterSet::ASCII,
                marking_string: s.to_string()
            })
        } else {
            Err(DisError::ParseError(format!("String is too long for AggregateMarking. Found {}, max 31 allowed.", s.len())))
        }
    }
}

/// 6.2.5 Aggregate Type record
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AggregateType {
    pub aggregate_kind: AggregateStateAggregateKind,
    pub domain: PlatformDomain,
    pub country: Country,
    pub category: u8,
    pub subcategory: AggregateStateSubcategory,
    pub specific: AggregateStateSpecific,
    pub extra: u8,
}

impl AggregateType {
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

    pub fn with_subcategory(mut self, subcategory: AggregateStateSubcategory) -> Self {
        self.subcategory = subcategory;
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

    pub fn record_length(&self) -> u16 {
        EIGHT_OCTETS as u16
    }
}

impl Display for AggregateType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}:{}:{}:{}:{}:{}",
            u8::from(self.aggregate_kind),
            u8::from(self.domain),
            u16::from(self.country),
            self.category,
            u8::from(self.subcategory),
            u8::from(self.specific),
            self.extra
        )
    }
}

impl FromStr for AggregateType {
    type Err = DisError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const NUM_DIGITS: usize = 7;
        let ss = s.split(':').collect::<Vec<&str>>();
        if ss.len() != NUM_DIGITS {
            return Err(DisError::ParseError(format!("Digits are not precisely {NUM_DIGITS}")));
        }
        Ok(Self {
            aggregate_kind: ss
                .get(0)
                .unwrap_or(&"0")
                .parse::<u8>()
                .map_err(|_| DisError::ParseError("Invalid kind digit".to_string()))?
                .into(),
            domain: ss
                .get(1)
                .unwrap_or(&"0")
                .parse::<u8>()
                .map_err(|_| DisError::ParseError("Invalid domain digit".to_string()))?
                .into(),
            country: ss
                .get(2)
                .unwrap_or(&"0")
                .parse::<u16>()
                .map_err(|_| DisError::ParseError("Invalid country digit".to_string()))?
                .into(),
            category: ss
                .get(3)
                .unwrap_or(&"0")
                .parse::<u8>()
                .map_err(|_| DisError::ParseError("Invalid category digit".to_string()))?,
            subcategory: ss
                .get(4)
                .unwrap_or(&"0")
                .parse::<u8>()
                .map_err(|_| DisError::ParseError("Invalid subcategory digit".to_string()))?
                .into(),
            specific: ss
                .get(5)
                .unwrap_or(&"0")
                .parse::<u8>()
                .map_err(|_| DisError::ParseError("Invalid specific digit".to_string()))?
                .into(),
            extra: ss
                .get(6)
                .unwrap_or(&"0")
                .parse::<u8>()
                .map_err(|_| DisError::ParseError("Invalid extra digit".to_string()))?
        })
    }
}

impl TryFrom<&str> for AggregateType {
    type Error = DisError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        AggregateType::from_str(value)
    }
}

impl TryFrom<String> for AggregateType {
    type Error = DisError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        TryFrom::<&str>::try_from(&value)
    }
}

/// Custom record for `SilentAggregateSystem`
#[derive(Clone, Debug, Default, PartialEq)]
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

    pub fn record_length(&self) -> u16 {
        FOUR_OCTETS as u16 + self.aggregate_type.record_length()
    }
}

/// 6.2.79 Silent Entity System record
#[derive(Clone, Debug, Default, PartialEq)]
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

    pub fn record_length(&self) -> u16 {
        TWO_OCTETS as u16
            + self.entity_type.record_length()
            + self.appearances.iter()
            .map(|app| app.record_length() )
            .sum::<u16>()
    }
}

