use crate::common::{BodyInfo, Interaction};
use crate::common::model::BeamData;
use crate::common::model::{EntityId, EventId, VectorF32};
use crate::enumerations::{PduType, IffSystemType, IffSystemMode, IffSystemName};
use crate::{IffApplicableModes, Mode5IffMission, Mode5MessageFormatsStatus, SimulationAddress, VariableRecordType};

pub const FUNDAMENTAL_OPERATIONAL_DATA_LENGTH: u16 = 16;

pub struct Iff {                                                    // 7.6.5.1 - page 394
    pub emitting_entity_id: EntityId,
    pub event_id: EventId,
    pub relative_antenna_location: VectorF32,
    pub system_id: SystemId,
    pub system_designator: u8,                                      // [See item d2) in 5.7.6.1.] - page 143
    pub system_specific_data: u8, // TODO 8-bit record defined by system type - See Clause B.5. - page 627
    pub fundamental_operational_data: FundamentalOperationalData,   // see 6.2.39 - page 292.
    // Layer 1 up to here
    pub layer_2: Option<IffLayer2>, // TODO - Basic Emissions Data
    pub layer_3: Option<IffLayer3>, // TODO - Mode 5 Functional Data
    pub layer_4: Option<IffLayer4>, // TODO - Mode S Functional Data
    pub layer_5: Option<IffLayer5>, // TODO - Data Communications
}

impl Default for Iff {
    fn default() -> Self {
        Self::new()
    }
}

impl Iff {
    pub fn new() -> Self {
        Self {
            emitting_entity_id: Default::default(),
            event_id: Default::default(),
            relative_antenna_location: Default::default(),
            system_id: SystemId::default(),
            system_designator: 0,
            system_specific_data: 0,
            fundamental_operational_data: FundamentalOperationalData::default(),
            layer_2: None,
            layer_3: None,
            layer_4: None,
            layer_5: None,
        }
    }
}

impl BodyInfo for Iff {
    fn body_length(&self) -> u16 {
        todo!()
    }

    fn body_type(&self) -> PduType {
        PduType::IFF
    }
}

impl Interaction for Iff {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.emitting_entity_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        // TODO can we derive the receiving entity from a layer?
        None
    }
}

pub struct SystemId {
    pub system_type: IffSystemType,
    pub system_name: IffSystemName,
    pub system_mode: IffSystemMode,
    pub change_options: ChangeOptionsRecord,
}

impl Default for SystemId {
    fn default() -> Self {
        SystemId {
            system_type: Default::default(),
            system_name: Default::default(),
            system_mode: Default::default(),
            change_options: ChangeOptionsRecord::default(),
        }
    }
}

pub struct ChangeOptionsRecord {
    pub change_indicator: bool,
    pub system_specific_field_1: bool,
    pub system_specific_field_2: bool,
    pub heartbeat_indicator: bool,
    pub transponder_interrogator_indicator: bool,
    pub simulation_mode: bool,
    pub interactive_capable: bool,
    pub test_mode: bool,
}

impl Default for ChangeOptionsRecord {
    fn default() -> Self {
        ChangeOptionsRecord {
            change_indicator: false,
            system_specific_field_1: false,
            system_specific_field_2: false,
            heartbeat_indicator: false,
            transponder_interrogator_indicator: false,
            simulation_mode: false,
            interactive_capable: false,
            test_mode: false,
        }
    }
}

pub struct FundamentalOperationalData {
    pub system_status: SystemStatus,
    pub data_field_1: u8,
    pub information_layers: InformationLayers,
    pub data_field_2: u8,
    pub parameter_1: u16,
    pub parameter_2: u16,
    pub parameter_3: u16,
    pub parameter_4: u16,
    pub parameter_5: u16,
    pub parameter_6: u16,
    pub parameter_7: u16,
}

impl Default for FundamentalOperationalData {
    fn default() -> Self {
        FundamentalOperationalData {
            system_status: SystemStatus::default(),
            data_field_1: 0,
            information_layers: InformationLayers::default(),
            data_field_2: 0,
            parameter_1: 0,
            parameter_2: 0,
            parameter_3: 0,
            parameter_4: 0,
            parameter_5: 0,
            parameter_6: 0,
            parameter_7: 0,
        }
    }
}

pub enum ParameterCapable {
    Capable,
    NotCapable,
}

// TODO use ParameterCapable enum instead of bool
pub struct SystemStatus {
    pub system_on_off_status: bool,
    pub parameter_1_capable: bool,
    pub parameter_2_capable: bool,
    pub parameter_3_capable: bool,
    pub parameter_4_capable: bool,
    pub parameter_5_capable: bool,
    pub parameter_6_capable: bool,
    pub operational_status: bool,
}

impl Default for SystemStatus {
    fn default() -> Self {
        SystemStatus {
            system_on_off_status: false,
            parameter_1_capable: false,
            parameter_2_capable: false,
            parameter_3_capable: false,
            parameter_4_capable: false,
            parameter_5_capable: false,
            parameter_6_capable: false,
            operational_status: false,
        }
    }
}

pub enum LayersPresenceApplicability {
    NotPresentApplicable,
    PresentApplicable,
}

pub struct InformationLayers {
    pub layer_1: LayersPresenceApplicability,
    pub layer_2: LayersPresenceApplicability,
    pub layer_3: LayersPresenceApplicability,
    pub layer_4: LayersPresenceApplicability,
    pub layer_5: LayersPresenceApplicability,
    pub layer_6: LayersPresenceApplicability,
    pub layer_7: LayersPresenceApplicability,
}

impl Default for InformationLayers {
    fn default() -> Self {
        InformationLayers {
            layer_1: LayersPresenceApplicability::NotPresentApplicable,
            layer_2: LayersPresenceApplicability::NotPresentApplicable,
            layer_3: LayersPresenceApplicability::NotPresentApplicable,
            layer_4: LayersPresenceApplicability::NotPresentApplicable,
            layer_5: LayersPresenceApplicability::NotPresentApplicable,
            layer_6: LayersPresenceApplicability::NotPresentApplicable,
            layer_7: LayersPresenceApplicability::NotPresentApplicable,
        }
    }
}

pub struct IffLayer2 {
    pub layer_header: LayerHeader,
    pub beam_data: BeamData,
    pub operational_parameter_1: u8,
    pub operational_parameter_2: u8,
    pub iff_fundamental_parameters: Vec<IffFundamentalParameterData>, // see 6.2.44 - page 300
}

pub struct IffFundamentalParameterData {
    pub erp: f32,
    pub frequency: f32,
    pub pgrf: f32,
    pub pulse_width: f32,
    pub burst_length: f32,
    pub applicable_modes: IffApplicableModes,
    pub system_specific_data: SystemSpecificData,
}

pub struct IffLayer3 {
    pub layer_header: LayerHeader,
    pub reporting_simulation: SimulationAddress,
    pub mode_5_basic_data: Mode5BasicData,
    pub iff_data_records: Vec<IffDataSpecification>,                // see 6.2.43 - page 299
}

pub enum Mode5BasicData {
    Interrogator(Mode5InterrogatorBasicData),                       // 7.6.5.4.2 Layer 3 Mode 5 Interrogator Format
    Transponder(Mode5TransponderBasicData),                         // 7.6.5.4.3 Layer 3 Mode 5 Transponder Format
}

pub struct IffLayer4 {
    // TODO 7.6.5.5 Layer 4 Mode S formats
}

pub struct IffLayer5 {
    // TODO
}

pub struct LayerHeader {
    pub layer_number: u8,
    pub layer_specific_information: u8,
    pub length: u16,
}

// TODO placeholder for 24-bits - See Annex B.
pub struct SystemSpecificData {
    pub part_1: u8,
    pub part_2: u8,
    pub part_3: u8,
}

// see B.2.26 - page 591
pub struct Mode5InterrogatorBasicData {
    pub mode_5_interrogator_status: Mode5InterrogatorStatus,        // B.2.27 Mode 5 Interrogator Status record - page 592
    pub mode_5_message_formats_present: Mode5MessageFormats,        // B.2.28 Mode 5 Message Formats record - page 592
    pub interrogated_entity_id: EntityId,
}

// see B.2.29 - page 593
pub struct Mode5TransponderBasicData {

}

pub struct Mode5InterrogatorStatus {
    pub iff_mission: Mode5IffMission,
    pub mode_5_message_formats_status: Mode5MessageFormatsStatus,
    pub on_off_status: Mode5OnOffStatus,
    pub damage_status: Mode5DamageStatus,
    pub malfunction_status: Mode5MalfunctionStatus,
}

pub enum Mode5OnOffStatus {
    On,
    Off,
}

pub enum Mode5DamageStatus {
    NoDamage,       // 0
    Damaged,        // 1
}

pub enum Mode5MalfunctionStatus {
    NoMalfunction,  // 0
    Malfunction,    // 1
}

pub struct Mode5MessageFormats {
    pub message_format_0: bool, // 0 - Not Present, 1 - Present
    pub message_format_1: bool,
    pub message_format_2: bool,
    pub message_format_3: bool,
    pub message_format_4: bool,
    pub message_format_5: bool,
    pub message_format_6: bool,
    pub message_format_7: bool,
    pub message_format_8: bool,
    pub message_format_9: bool,
    pub message_format_10: bool,
    pub message_format_11: bool,
    pub message_format_12: bool,
    pub message_format_13: bool,
    pub message_format_14: bool,
    pub message_format_15: bool,
    pub message_format_16: bool,
    pub message_format_17: bool,
    pub message_format_18: bool,
    pub message_format_19: bool,
    pub message_format_20: bool,
    pub message_format_21: bool,
    pub message_format_22: bool,
    pub message_format_23: bool,
    pub message_format_24: bool,
    pub message_format_25: bool,
    pub message_format_26: bool,
    pub message_format_27: bool,
    pub message_format_28: bool,
    pub message_format_29: bool,
    pub message_format_30: bool,
    pub message_format_31: bool,
}
pub struct IffDataSpecification {
    pub iff_data_records: Vec<IffDataRecord>,
}

pub struct IffDataRecord {
    pub record_type: VariableRecordType,   // UID 66
    pub record_specific_fields: Vec<u8>,
}