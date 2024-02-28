use crate::common::model::{EntityId, PduBody, SimulationAddress};
use crate::common::{BodyInfo, Interaction};
use crate::common::attribute::builder::AttributeBuilder;
use crate::enumerations::{PduType, ProtocolVersion, VariableRecordType, AttributeActionCode};

pub const BASE_ATTRIBUTE_BODY_LENGTH: u16 = 20;
pub const BASE_ATTRIBUTE_RECORD_SET_LENGTH : u16 = 8;
pub const BASE_ATTRIBUTE_RECORD_LENGTH_OCTETS: u16 = 6;

/// 5.3.6 Attribute PDU
///
/// 7.2.6 Attribute PDU
#[derive(Debug, Default, PartialEq)]
pub struct Attribute {
    pub originating_simulation_address : SimulationAddress,
    pub record_pdu_type: PduType,
    pub record_protocol_version: ProtocolVersion,
    pub master_attribute_record_type: VariableRecordType,
    pub action_code: AttributeActionCode,
    pub attribute_record_sets: Vec<AttributeRecordSet>
}

impl Attribute {
    pub fn builder() -> AttributeBuilder {
        AttributeBuilder::new()
    }

    pub fn into_builder(self) -> AttributeBuilder {
        AttributeBuilder::new_from_body(self)
    }

    pub fn into_pdu_body(self) -> PduBody {
        PduBody::Attribute(self)
    }
}

impl BodyInfo for Attribute {
    fn body_length(&self) -> u16 {
        BASE_ATTRIBUTE_BODY_LENGTH + self.attribute_record_sets.iter()
            .map(|set| BASE_ATTRIBUTE_RECORD_SET_LENGTH + set.attribute_records.iter()
                .map(|record|
                    BASE_ATTRIBUTE_RECORD_LENGTH_OCTETS + record.specific_fields.len() as u16)
                .sum::<u16>())
            .sum::<u16>()
    }

    fn body_type(&self) -> PduType {
        PduType::Attribute
    }
}

impl Interaction for Attribute {
    // TODO cannot assign to a specific entity using only the simulation address
    fn originator(&self) -> Option<&EntityId> {
        None
    }

    fn receiver(&self) -> Option<&EntityId> {
        None
    }
}

/// 5.3.6.3
#[derive(Debug, PartialEq)]
pub struct AttributeRecordSet {
    pub entity_id: EntityId,
    pub attribute_records: Vec<AttributeRecord>,
}

impl Default for AttributeRecordSet {
    fn default() -> Self {
        Self::new()
    }
}

impl AttributeRecordSet {
    pub fn new() -> Self {
        Self {
            entity_id: Default::default(),
            attribute_records: vec![],
        }
    }

    pub fn with_entity_id(mut self, entity_id: EntityId) -> Self {
        self.entity_id = entity_id;
        self
    }

    pub fn with_attribute_records(mut self, attribute_records: Vec<AttributeRecord>) -> Self {
        self.attribute_records = attribute_records;
        self
    }

    pub fn with_attribute_record(mut self, attribute_record: AttributeRecord) -> Self {
        self.attribute_records.push(attribute_record);
        self
    }
}

/// 6.2.10 Attribute record
#[derive(Debug, PartialEq)]
pub struct AttributeRecord {
    pub record_type: VariableRecordType,
    pub specific_fields: Vec<u8>,
}

impl Default for AttributeRecord {
    fn default() -> Self {
        Self::new()
    }
}

impl AttributeRecord {
    pub fn new() -> Self {
        Self {
            record_type: Default::default(),
            specific_fields: vec![],
        }
    }

    pub fn with_record_type(mut self, record_type: VariableRecordType) -> Self {
        self.record_type = record_type;
        self
    }

    pub fn with_specific_fields(mut self, specific_fields: Vec<u8>) -> Self {
        self.specific_fields = specific_fields;
        self
    }
}