use crate::aggregate_state::builder::AggregateStateBuilder;
use crate::common::{BodyInfo, Interaction};
use crate::constants::{EIGHT_OCTETS, FOUR_OCTETS, THIRTY_TWO_OCTETS, TWO_OCTETS};
use crate::entity_state::model::EntityAppearance;
use crate::enumerations::{
    AggregateStateAggregateKind, AggregateStateAggregateState, AggregateStateFormation,
    AggregateStateSpecific, AggregateStateSubcategory, Country, EntityMarkingCharacterSet, ForceId,
    PduType, PlatformDomain,
};
use crate::model::{
    length_padded_to_num, EntityId, EntityType, Location, Orientation, PduBody, VariableDatum,
    VectorF32, BASE_VARIABLE_DATUM_LENGTH,
};
use crate::DisError;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub(crate) const BASE_AGGREGATE_STATE_BODY_LENGTH: u16 = 124;

/// 5.9.2.2 Aggregate State PDU
///
/// 7.8.2 Aggregate State PDU
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
    #[must_use]
    pub fn builder() -> AggregateStateBuilder {
        AggregateStateBuilder::new()
    }

    #[must_use]
    pub fn into_builder(self) -> AggregateStateBuilder {
        AggregateStateBuilder::new_from_body(self)
    }

    #[must_use]
    pub fn into_pdu_body(self) -> PduBody {
        PduBody::AggregateState(self)
    }
}

/// Calculate the intermediate length and padding of an `AggregateState` PDU.
///
/// Returns a tuple consisting of the intermediate length including the padding,
/// and the length of the padding, in octets.
pub(crate) fn aggregate_state_intermediate_length_padding(
    aggregates: &[EntityId],
    entities: &[EntityId],
) -> (u16, u16) {
    let intermediate_length = BASE_AGGREGATE_STATE_BODY_LENGTH
        + aggregates.iter().map(crate::model::EntityId::record_length ).sum::<u16>() // number of aggregate ids
        + entities.iter().map(crate::model::EntityId::record_length ).sum::<u16>(); // number of entity ids
    let padding_length = intermediate_length % (FOUR_OCTETS as u16); // padding to 32-bits (4 octets) boundary
    (intermediate_length + padding_length, padding_length)
}

impl BodyInfo for AggregateState {
    fn body_length(&self) -> u16 {
        let (intermediate_length, _padding_length) =
            aggregate_state_intermediate_length_padding(&self.aggregates, &self.entities);
        intermediate_length
            // number of silent aggregate systems
            + self.silent_aggregate_systems.iter().map(SilentAggregateSystem::record_length ).sum::<u16>()
            // number of silent entity systems
            + self.silent_entity_systems.iter().map(SilentEntitySystem::record_length ).sum::<u16>()
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

impl From<AggregateState> for PduBody {
    #[inline]
    fn from(value: AggregateState) -> Self {
        value.into_pdu_body()
    }
}

/// 6.2.4 Aggregate Marking record
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AggregateMarking {
    pub marking_character_set: EntityMarkingCharacterSet,
    pub marking_string: String, // 31 byte String
}

impl AggregateMarking {
    #[must_use]
    pub fn new(marking: String, character_set: EntityMarkingCharacterSet) -> Self {
        Self {
            marking_character_set: character_set,
            marking_string: marking,
        }
    }

    pub fn new_ascii<S: Into<String>>(marking: S) -> Self {
        AggregateMarking::new(marking.into(), EntityMarkingCharacterSet::ASCII)
    }

    #[allow(clippy::return_self_not_must_use)]
    pub fn with_marking<S: Into<String>>(mut self, marking: S) -> Self {
        self.marking_string = marking.into();
        self
    }

    #[must_use]
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
                marking_string: s.to_string(),
            })
        } else {
            Err(DisError::ParseError(format!(
                "String is too long for AggregateMarking. Found {}, max 31 allowed.",
                s.len()
            )))
        }
    }
}

/// 6.2.5 Aggregate Type record
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
    #[must_use]
    pub fn with_aggregate_kind(mut self, aggregate_kind: AggregateStateAggregateKind) -> Self {
        self.aggregate_kind = aggregate_kind;
        self
    }

    #[must_use]
    pub fn with_domain(mut self, domain: PlatformDomain) -> Self {
        self.domain = domain;
        self
    }

    #[must_use]
    pub fn with_country(mut self, country: Country) -> Self {
        self.country = country;
        self
    }

    #[must_use]
    pub fn with_category(mut self, category: u8) -> Self {
        self.category = category;
        self
    }

    #[must_use]
    pub fn with_subcategory(mut self, subcategory: AggregateStateSubcategory) -> Self {
        self.subcategory = subcategory;
        self
    }

    #[must_use]
    pub fn with_specific(mut self, specific: AggregateStateSpecific) -> Self {
        self.specific = specific;
        self
    }

    #[must_use]
    pub fn with_extra(mut self, extra: u8) -> Self {
        self.extra = extra;
        self
    }

    #[must_use]
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

#[allow(clippy::get_first)]
impl FromStr for AggregateType {
    type Err = DisError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const NUM_DIGITS: usize = 7;
        let ss = s.split(':').collect::<Vec<&str>>();
        if ss.len() != NUM_DIGITS {
            return Err(DisError::ParseError(format!(
                "AggregateType string pattern does contain not precisely {NUM_DIGITS} digits"
            )));
        }
        Ok(Self {
            aggregate_kind: ss
                .get(0)
                .expect("Impossible - checked for correct number of digits")
                .parse::<u8>()
                .map_err(|_| DisError::ParseError("Invalid kind digit".to_string()))?
                .into(),
            domain: ss
                .get(1)
                .expect("Impossible - checked for correct number of digits")
                .parse::<u8>()
                .map_err(|_| DisError::ParseError("Invalid domain digit".to_string()))?
                .into(),
            country: ss
                .get(2)
                .expect("Impossible - checked for correct number of digits")
                .parse::<u16>()
                .map_err(|_| DisError::ParseError("Invalid country digit".to_string()))?
                .into(),
            category: ss
                .get(3)
                .expect("Impossible - checked for correct number of digits")
                .parse::<u8>()
                .map_err(|_| DisError::ParseError("Invalid category digit".to_string()))?,
            subcategory: ss
                .get(4)
                .expect("Impossible - checked for correct number of digits")
                .parse::<u8>()
                .map_err(|_| DisError::ParseError("Invalid subcategory digit".to_string()))?
                .into(),
            specific: ss
                .get(5)
                .expect("Impossible - checked for correct number of digits")
                .parse::<u8>()
                .map_err(|_| DisError::ParseError("Invalid specific digit".to_string()))?
                .into(),
            extra: ss
                .get(6)
                .expect("Impossible - checked for correct number of digits")
                .parse::<u8>()
                .map_err(|_| DisError::ParseError("Invalid extra digit".to_string()))?,
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SilentAggregateSystem {
    pub number_of_aggregates: u16,
    pub aggregate_type: AggregateType,
}

impl SilentAggregateSystem {
    #[must_use]
    pub fn with_number_of_aggregates(mut self, number_of_aggregates: u16) -> Self {
        self.number_of_aggregates = number_of_aggregates;
        self
    }

    #[must_use]
    pub fn with_aggregate_type(mut self, aggregate_type: AggregateType) -> Self {
        self.aggregate_type = aggregate_type;
        self
    }

    #[must_use]
    pub fn record_length(&self) -> u16 {
        FOUR_OCTETS as u16 + self.aggregate_type.record_length()
    }
}

/// 6.2.79 Silent Entity System record
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SilentEntitySystem {
    pub number_of_entities: u16,
    pub entity_type: EntityType,
    pub appearances: Vec<EntityAppearance>,
}

impl SilentEntitySystem {
    #[must_use]
    pub fn with_number_of_entities(mut self, number_of_entities: u16) -> Self {
        self.number_of_entities = number_of_entities;
        self
    }

    #[must_use]
    pub fn with_entity_type(mut self, entity_type: EntityType) -> Self {
        self.entity_type = entity_type;
        self
    }

    #[must_use]
    pub fn with_appearance(mut self, appearance: EntityAppearance) -> Self {
        self.appearances.push(appearance);
        self
    }

    #[must_use]
    pub fn with_appearances(mut self, appearances: Vec<EntityAppearance>) -> Self {
        self.appearances = appearances;
        self
    }

    #[must_use]
    pub fn record_length(&self) -> u16 {
        TWO_OCTETS as u16
            + self.entity_type.record_length()
            + self
                .appearances
                .iter()
                .map(crate::entity_state::model::EntityAppearance::record_length)
                .sum::<u16>()
    }
}
