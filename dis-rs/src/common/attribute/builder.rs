use crate::common::attribute::model::{Attribute, AttributeRecordSet};
use crate::common::model::SimulationAddress;
use crate::enumerations::{AttributeActionCode, PduType, ProtocolVersion, VariableRecordType};

pub struct AttributeBuilder(Attribute);

impl AttributeBuilder {
    pub fn new() -> Self {
        AttributeBuilder(Attribute::default())
    }

    pub fn new_from_body(body: Attribute) -> Self {
        AttributeBuilder(body)
    }

    pub fn build(self) -> Attribute {
        self.0
    }

    pub fn with_originating_simulation_address(mut self, originating_simulation_address: SimulationAddress) -> Self {
        self.0.originating_simulation_address = originating_simulation_address;
        self
    }

    pub fn with_record_pdu_type(mut self, record_pdu_type: PduType) -> Self {
        self.0.record_pdu_type = record_pdu_type;
        self
    }

    pub fn with_record_protocol_version(mut self, record_protocol_version: ProtocolVersion) -> Self {
        self.0.record_protocol_version = record_protocol_version;
        self
    }

    pub fn with_master_attribute_record_type(mut self, master_attribute_record_type: VariableRecordType) -> Self {
        self.0.master_attribute_record_type = master_attribute_record_type;
        self
    }

    pub fn with_action_code(mut self, action_code: AttributeActionCode) -> Self {
        self.0.action_code = action_code;
        self
    }

    pub fn with_attribute_record_set(mut self, attribute_record_set: AttributeRecordSet) -> Self {
        self.0.attribute_record_sets.push(attribute_record_set);
        self
    }

    pub fn with_attribute_record_sets(mut self, attribute_record_sets: Vec<AttributeRecordSet>) -> Self {
        self.0.attribute_record_sets = attribute_record_sets;
        self
    }
}
