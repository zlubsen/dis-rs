use crate::common_records::{
    EntityIdentifier, EntityType, EventIdentifier, PDUHeader, PDUStatus, SimulationAddress,
};
use crate::enumerations::{Country, DISPDUType, DISProtocolVersion, EntityKind};
use crate::errors::DisError;
use crate::fixed_parameters::NO_ENTITY;
use crate::PDU_HEADER_LEN_BYTES;
use std::fmt::Display;
use std::str::FromStr;

impl Default for PDUHeader {
    fn default() -> Self {
        Self {
            protocol_version: DISProtocolVersion::IEEE1278_1202X,
            compatibility_version: DISProtocolVersion::IEEE1278_1202X,
            exercise_identifier: 1,
            pdu_type: DISPDUType::default(),
            pdu_status: PDUStatus::default(),
            pdu_header_length: PDU_HEADER_LEN_BYTES as u8,
            pdu_length: PDU_HEADER_LEN_BYTES,
            timestamp: 0,
        }
    }
}

impl PDUHeader {
    #[must_use]
    pub fn new(
        protocol_version: DISProtocolVersion,
        exercise_identifier: u8,
        pdu_type: DISPDUType,
    ) -> Self {
        Self {
            protocol_version,
            compatibility_version: DISProtocolVersion::IEEE1278_1202X,
            pdu_type,
            pdu_length: 0u16,
            pdu_status: PDUStatus::default(),
            exercise_identifier,
            pdu_header_length: PDU_HEADER_LEN_BYTES as u8,
            timestamp: 0,
        }
    }

    #[must_use]
    pub fn with_protocol_version(mut self, version: DISProtocolVersion) -> Self {
        self.protocol_version = version;
        self
    }

    #[must_use]
    pub fn with_compatibility_version(mut self, version: DISProtocolVersion) -> Self {
        self.compatibility_version = version;
        self
    }

    #[must_use]
    pub fn with_exercise_identifier(mut self, id: u8) -> Self {
        self.exercise_identifier = id;
        self
    }

    #[must_use]
    pub fn with_pdu_type(mut self, pdu_type: DISPDUType) -> Self {
        self.pdu_type = pdu_type;
        self
    }

    #[must_use]
    pub fn with_pdu_status(mut self, status: PDUStatus) -> Self {
        self.pdu_status = status;
        self
    }

    /// Sets the length of the total PDU, thus header plus body.
    #[must_use]
    pub fn with_length(mut self, length: u16) -> Self {
        self.pdu_length = length;
        self
    }

    /// Sets the length of the PDU, based on _only_ the length of the body.
    /// The total PDU length is computed and set by this function.
    #[must_use]
    pub fn with_body_length(mut self, body_length: u16) -> Self {
        self.pdu_length = PDU_HEADER_LEN_BYTES + body_length;
        self
    }

    /// Sets the timestamp of the PDUHeader to the provided value.
    /// Whether the timestamp is set to synchronized or unsynchronized in the `PDUStatus` field
    /// is left unaffected.
    #[must_use]
    pub fn with_timestamp(mut self, timestamp: impl Into<i64>) -> Self {
        self.timestamp = timestamp.into();
        self
    }

    /// Sets the timestamp of the PDUHeader to the provided value, and updates the
    /// `PDUStatus` field to be _synchronized_. The LVC and TEI fields are left unaffected.
    #[must_use]
    pub fn with_timestamp_synchronized(mut self, timestamp: impl Into<i64>) -> Self {
        self.pdu_status = PDUStatus {
            timestamp_synchronized: true,
            lvc: self.pdu_status.lvc,
            transferred_entity_indicator: self.pdu_status.transferred_entity_indicator,
        };
        self.timestamp = timestamp.into();
        self
    }

    /// Sets the timestamp of the PDUHeader to the provided value, and updates the
    /// `PDUStatus` field to be _unsynchronized_. The LVC and TEI fields are left unaffected.
    #[must_use]
    pub fn with_timestamp_unsynchronized(mut self, timestamp: impl Into<i64>) -> Self {
        self.pdu_status = PDUStatus {
            timestamp_synchronized: false,
            lvc: self.pdu_status.lvc,
            transferred_entity_indicator: self.pdu_status.transferred_entity_indicator,
        };
        self.timestamp = timestamp.into();
        self
    }
}

impl SimulationAddress {
    #[must_use]
    pub fn new(site_number: u16, application_number: u16) -> Self {
        SimulationAddress {
            site_number,
            application_number,
        }
    }
}

impl Display for SimulationAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.site_number, self.application_number)
    }
}

#[allow(clippy::get_first)]
impl TryFrom<&[&str]> for SimulationAddress {
    type Error = DisError;

    fn try_from(value: &[&str]) -> Result<Self, Self::Error> {
        const NUM_DIGITS: usize = 2;
        if value.len() != NUM_DIGITS {
            return Err(DisError::ParseError(format!(
                "SimulationAddress string pattern does not contain precisely {NUM_DIGITS} digits"
            )));
        }
        Ok(Self {
            site_number: value
                .get(0)
                .expect("Impossible - checked for correct number of digits")
                .parse::<u16>()
                .map_err(|_| DisError::ParseError("Invalid site id digit".to_string()))?,
            application_number: value
                .get(1)
                .expect("Impossible - checked for correct number of digits")
                .parse::<u16>()
                .map_err(|_| DisError::ParseError("Invalid application id digit".to_string()))?,
        })
    }
}

impl FromStr for SimulationAddress {
    type Err = DisError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ss = s.split(':').collect::<Vec<&str>>();
        Self::try_from(ss.as_slice())
    }
}

impl TryFrom<&str> for SimulationAddress {
    type Error = DisError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        SimulationAddress::from_str(value)
    }
}

impl TryFrom<String> for SimulationAddress {
    type Error = DisError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        TryFrom::<&str>::try_from(&value)
    }
}

impl EntityIdentifier {
    #[must_use]
    pub fn new(site_number: u16, application_number: u16, entity_number: u16) -> Self {
        Self {
            simulation_address: SimulationAddress {
                site_number,
                application_number,
            },
            entity_number,
        }
    }

    #[must_use]
    pub fn new_sim_address(simulation_address: SimulationAddress, entity_number: u16) -> Self {
        Self {
            simulation_address,
            entity_number,
        }
    }

    #[must_use]
    pub fn new_simulation_identifier(simulation_address: SimulationAddress) -> Self {
        Self {
            simulation_address,
            entity_number: NO_ENTITY,
        }
    }
}

impl Display for EntityIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.simulation_address, self.entity_number)
    }
}

#[allow(clippy::get_first)]
impl FromStr for EntityIdentifier {
    type Err = DisError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const NUM_DIGITS: usize = 3;
        let mut ss = s.split(':').collect::<Vec<&str>>();
        if ss.len() != NUM_DIGITS {
            return Err(DisError::ParseError(format!(
                "EntityId string pattern does not contain precisely {NUM_DIGITS} digits"
            )));
        }
        let entity_number = ss
            .pop()
            .expect("Impossible - checked for correct number of digits")
            .parse::<u16>()
            .map_err(|_| DisError::ParseError("Invalid entity id digit".to_string()))?;
        Ok(Self {
            simulation_address: ss.as_slice().try_into()?,
            entity_number,
        })
    }
}

impl TryFrom<&str> for EntityIdentifier {
    type Error = DisError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        EntityIdentifier::from_str(value)
    }
}

impl TryFrom<String> for EntityIdentifier {
    type Error = DisError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        TryFrom::<&str>::try_from(&value)
    }
}

impl EventIdentifier {
    #[must_use]
    pub fn new(site_number: u16, application_number: u16, event_number: u16) -> Self {
        Self {
            simulation_address: SimulationAddress {
                site_number,
                application_number,
            },
            event_number,
        }
    }

    #[must_use]
    pub fn new_sim_address(simulation_address: SimulationAddress, event_number: u16) -> Self {
        Self {
            simulation_address,
            event_number,
        }
    }
}

impl Display for EventIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.simulation_address, self.event_number)
    }
}

#[allow(clippy::get_first)]
impl FromStr for EventIdentifier {
    type Err = DisError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const NUM_DIGITS: usize = 3;
        let mut ss = s.split(':').collect::<Vec<&str>>();
        if ss.len() != NUM_DIGITS {
            return Err(DisError::ParseError(format!(
                "EventId string pattern does not contain precisely {NUM_DIGITS} digits"
            )));
        }
        let event_number = ss
            .pop()
            .expect("Impossible - checked for correct number of digits")
            .parse::<u16>()
            .map_err(|_| DisError::ParseError("Invalid event id digit".to_string()))?;
        Ok(Self {
            simulation_address: ss.as_slice().try_into()?,
            event_number,
        })
    }
}

impl TryFrom<&str> for EventIdentifier {
    type Error = DisError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        EventIdentifier::from_str(value)
    }
}

impl TryFrom<String> for EventIdentifier {
    type Error = DisError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        TryFrom::<&str>::try_from(&value)
    }
}

impl EntityType {
    #[must_use]
    pub fn with_kind(mut self, kind: EntityKind) -> Self {
        self.entity_kind = kind;
        self
    }

    #[must_use]
    pub fn with_domain(mut self, domain: u8) -> Self {
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
    pub fn with_subcategory(mut self, subcategory: u8) -> Self {
        self.subcategory = subcategory;
        self
    }

    #[must_use]
    pub fn with_specific(mut self, specific: u8) -> Self {
        self.specific = specific;
        self
    }

    #[must_use]
    pub fn with_extra(mut self, extra: u8) -> Self {
        self.extra = extra;
        self
    }
}

impl Display for EntityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}:{}:{}:{}:{}:{}",
            u8::from(self.entity_kind),
            self.domain,
            u16::from(self.country),
            self.category,
            self.subcategory,
            self.specific,
            self.extra
        )
    }
}

#[allow(clippy::get_first)]
impl FromStr for EntityType {
    type Err = DisError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const NUM_DIGITS: usize = 7;
        let ss = s.split(':').collect::<Vec<&str>>();
        if ss.len() != NUM_DIGITS {
            return Err(DisError::ParseError(format!(
                "EntityType string pattern does not contain precisely {NUM_DIGITS} digits"
            )));
        }
        Ok(Self {
            entity_kind: ss
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
                .map_err(|_| DisError::ParseError("Invalid subcategory digit".to_string()))?,
            specific: ss
                .get(5)
                .expect("Impossible - checked for correct number of digits")
                .parse::<u8>()
                .map_err(|_| DisError::ParseError("Invalid specific digit".to_string()))?,
            extra: ss
                .get(6)
                .expect("Impossible - checked for correct number of digits")
                .parse::<u8>()
                .map_err(|_| DisError::ParseError("Invalid extra digit".to_string()))?,
        })
    }
}

impl TryFrom<&str> for EntityType {
    type Error = DisError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        EntityType::from_str(value)
    }
}

impl TryFrom<String> for EntityType {
    type Error = DisError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        TryFrom::<&str>::try_from(&value)
    }
}

// TODO same for AggregateType
