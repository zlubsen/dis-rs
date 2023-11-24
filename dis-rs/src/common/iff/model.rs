use crate::common::{BodyInfo, Interaction};
use crate::common::model::{BeamData, EntityId, EventId, VectorF32, SimulationAddress};
use crate::constants::{FOUR_OCTETS, SIX_OCTETS};
use crate::enumerations::{PduType, AircraftIdentificationType, AircraftPresentDomain, CapabilityReport, DataCategory, IffSystemType, IffSystemMode, IffSystemName, IffApplicableModes, NavigationSource, Mode5IffMission, Mode5MessageFormatsStatus, Mode5SAltitudeResolution, ModeSTransmitState, ModeSSquitterType, ModeSSquitterRecordSource, VariableRecordType};
use crate::length_padded_to_num_bytes;

pub const IFF_PDU_LAYER_1_DATA_LENGTH_OCTETS: u16 = 60;
pub const FUNDAMENTAL_OPERATIONAL_DATA_LENGTH: u16 = 16;

/// 7.6.5 Identification Friend or Foe (IFF) PDU
///
/// 7.6.5.1 General
///
/// 7.6.5.2 Layer 1 basic system data
pub struct Iff {
    pub emitting_entity_id: EntityId,
    pub event_id: EventId,
    pub relative_antenna_location: VectorF32,
    pub system_id: SystemId,
    pub system_designator: u8,                                      // [See item d2) in 5.7.6.1.] - page 143
    pub system_specific_data: u8, // TODO 8-bit record defined by system type - See Clause B.5. - page 627
    pub fundamental_operational_data: FundamentalOperationalData,   // see 6.2.39 - page 292.
    // Layer 1 up to here
    pub layer_2: Option<IffLayer2>, // 7.6.5.3 Layer 2 emissions data
    pub layer_3: Option<IffLayer3>, // Mode 5 Functional Data
    pub layer_4: Option<IffLayer4>, // Mode S Functional Data
    pub layer_5: Option<IffLayer5>, // Data Communications
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
        IFF_PDU_LAYER_1_DATA_LENGTH_OCTETS +
            if let Some(layer_2) = &self.layer_2 {
                layer_2.data_length()
            } else { 0 } +
            if let Some(layer_3) = &self.layer_3 {
                layer_3.data_length()
            } else { 0 } +
            if let Some(layer_4) = &self.layer_4 {
                layer_4.data_length()
            } else { 0 } +
            if let Some(layer_5) = &self.layer_5 {
                layer_5.data_length()
            } else { 0 }
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

/// 6.2.39 Fundamental Operational Data record
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
        }
    }
}

pub enum ParameterCapable {
    Capable,
    NotCapable,
}

impl Default for ParameterCapable {
    fn default() -> Self {
        ParameterCapable::Capable
    }
}

impl Default for SystemStatus {
    fn default() -> Self {
        SystemStatus {
            system_on_off_status: false,
            parameter_1_capable: ParameterCapable::default(),
            parameter_2_capable: ParameterCapable::default(),
            parameter_3_capable: ParameterCapable::default(),
            parameter_4_capable: ParameterCapable::default(),
            parameter_5_capable: ParameterCapable::default(),
            parameter_6_capable: ParameterCapable::default(),
            operational_status: ParameterCapable::default(),
        }
    }
}

pub enum LayersPresenceApplicability {
    NotPresentApplicable,
    PresentApplicable,
}

impl Default for LayersPresenceApplicability {
    fn default() -> Self {
        LayersPresenceApplicability::NotPresentApplicable
    }
}

/// 6.2.43 IFF Data Specification record
#[derive(Default)]
pub struct IffDataSpecification {
    pub iff_data_records: Vec<IffDataRecord>,
}

/// 6.2.45 Information Layers record
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
            layer_1: LayersPresenceApplicability::default(),
            layer_2: LayersPresenceApplicability::default(),
            layer_3: LayersPresenceApplicability::default(),
            layer_4: LayersPresenceApplicability::default(),
            layer_5: LayersPresenceApplicability::default(),
            layer_6: LayersPresenceApplicability::default(),
            layer_7: LayersPresenceApplicability::default(),
        }
    }
}

/// 7.6.5.3 Layer 2 emissions data
///
/// The Secondary Operational Data record (6.2.76) has been flattened in the IffLayer2 struct, as it only
/// contains two 8-bit records.
pub struct IffLayer2 {
    pub layer_header: LayerHeader,
    pub beam_data: BeamData,
    pub operational_parameter_1: u8,
    pub operational_parameter_2: u8,
    pub iff_fundamental_parameters: Vec<IffFundamentalParameterData>,
}

impl Default for IffLayer2 {
    fn default() -> Self {
        Self {
            layer_header: LayerHeader { layer_number: 2, ..Default::default() },
            beam_data: Default::default(),
            operational_parameter_1: 0,
            operational_parameter_2: 0,
            iff_fundamental_parameters: vec![],
        }
    }
}

impl IffLayer2 {
    fn data_length(&self) -> u16 {
        const LAYER_2_BASE_DATA_LENGTH_OCTETS: u16 = 28;
        const IFF_FUNDAMENTAL_PARAMETER_DATA_LENGTH_OCTETS: u16 = 24;
        LAYER_2_BASE_DATA_LENGTH_OCTETS
            + (self.iff_fundamental_parameters.len() as u16 * IFF_FUNDAMENTAL_PARAMETER_DATA_LENGTH_OCTETS)
    }
}

/// 6.2.44 IFF Fundamental Parameter Data Record
pub struct IffFundamentalParameterData {
    pub erp: f32,
    pub frequency: f32,
    pub pgrf: f32,
    pub pulse_width: f32,
    pub burst_length: f32,
    pub applicable_modes: IffApplicableModes,
    pub system_specific_data: SystemSpecificData,
}

impl Default for IffFundamentalParameterData {
    fn default() -> Self {
        Self {
            erp: 0.0,
            frequency: 0.0,
            pgrf: 0.0,
            pulse_width: 0.0,
            burst_length: 0.0,
            applicable_modes: IffApplicableModes::default(),
            system_specific_data: SystemSpecificData::default(),
        }
    }
}

/// 7.6.5.4 Layer 3 Mode 5 formats
/// 7.6.5.4.2 Layer 3 Mode 5 Interrogator Format
/// 7.6.5.4.3 Layer 3 Mode 5 Transponder Format
pub struct IffLayer3 {
    pub layer_header: LayerHeader,
    pub reporting_simulation: SimulationAddress,
    pub mode_5_basic_data: Mode5BasicData,
    pub iff_data_specification: IffDataSpecification,                // see 6.2.43 - page 299
}

impl Default for IffLayer3 {
    fn default() -> Self {
        Self {
            layer_header: LayerHeader { layer_number: 3, ..Default::default() },
            reporting_simulation: SimulationAddress::default(),
            mode_5_basic_data: Mode5BasicData::default(),
            iff_data_specification: IffDataSpecification::default(),
        }
    }
}

impl IffLayer3 {
    pub fn data_length(&self) -> u16 {
        const LAYER_3_BASE_DATA_LENGTH_OCTETS: u16 = 26;
        LAYER_3_BASE_DATA_LENGTH_OCTETS + self.iff_data_specification.data_length()
    }
}

pub enum Mode5BasicData {
    Interrogator(Mode5InterrogatorBasicData),                       // 7.6.5.4.2 Layer 3 Mode 5 Interrogator Format
    Transponder(Mode5TransponderBasicData),                         // 7.6.5.4.3 Layer 3 Mode 5 Transponder Format
}

impl Default for Mode5BasicData {
    fn default() -> Self {
        Self::Interrogator(Mode5InterrogatorBasicData::default())
    }
}

/// 7.6.5.5 Layer 4 Mode S formats
pub struct IffLayer4 {
    pub layer_header: LayerHeader,
    pub reporting_simulation: SimulationAddress,
    pub mode_s_basic_data: ModeSBasicData,
    pub iff_data_records: IffDataSpecification,                // see 6.2.43 - page 299
}

impl Default for IffLayer4 {
    fn default() -> Self {
        Self {
            layer_header: LayerHeader { layer_number: 4, ..Default::default() },
            reporting_simulation: Default::default(),
            mode_s_basic_data: Default::default(),
            iff_data_records: IffDataSpecification::default(),
        }
    }
}

impl IffLayer4 {
    pub fn data_length(&self) -> u16 {
        const LAYER_4_BASE_DATA_LENGTH_OCTETS: u16 = 34;
        LAYER_4_BASE_DATA_LENGTH_OCTETS + self.iff_data_records.data_length()
    }
}

pub enum ModeSBasicData {
    Interrogator(ModeSInterrogatorBasicData),                       // 7.6.5.5.2 Layer 4 Mode S Interrogator Format
    Transponder(ModeSTransponderBasicData),                         // 7.6.5.5.3 Layer 4 Mode S Transponder Format
}

impl Default for ModeSBasicData {
    fn default() -> Self {
        Self::Interrogator(ModeSInterrogatorBasicData::default())
    }
}

/// 6.2.51 Layer Header
#[derive(Default)]
pub struct LayerHeader {
    pub layer_number: u8,
    pub layer_specific_information: u8,
    pub length: u16,
}

// TODO placeholder for 24-bits - See Annex B.
#[derive(Default)]
pub struct SystemSpecificData {
    pub part_1: u8,
    pub part_2: u8,
    pub part_3: u8,
}

/// B.2.9 Enhanced Mode 1 Code record
pub struct EnhancedMode1Code {
    pub code_element_1_D: u16,
    pub code_element_2_C: u16,
    pub code_element_3_B: u16,
    pub code_element_4_A: u16,
    pub on_off_status: OnOffStatus,
    pub damage_status: DamageStatus,
    pub malfunction_status: MalfunctionStatus,
}

/// B.2.26 Mode 5 Interrogator Basic Data record
#[derive(Default)]
pub struct Mode5InterrogatorBasicData {
    pub status: Mode5InterrogatorStatus,                            // B.2.27 Mode 5 Interrogator Status record - page 592
    pub mode_5_message_formats_present: Mode5MessageFormats,        // B.2.28 Mode 5 Message Formats record - page 592
    pub interrogated_entity_id: EntityId,
}

/// B.2.27 Mode 5 Interrogator Status record
#[derive(Default)]
pub struct Mode5InterrogatorStatus {
    pub iff_mission: Mode5IffMission,
    pub mode_5_message_formats_status: Mode5MessageFormatsStatus,
    pub on_off_status: OnOffStatus,
    pub damage_status: DamageStatus,
    pub malfunction_status: MalfunctionStatus,
}

/// B.2.29 Mode 5 Transponder Basic Data record
#[derive(Default)]
pub struct Mode5TransponderBasicData {
    pub status: Mode5TransponderStatus,
    pub pin: u16,
    pub mode_5_message_formats_present: Mode5MessageFormats,        // B.2.28 Mode 5 Message Formats record
    pub enhanced_mode_1: EnhancedMode1Code,                         // B.2.9 Enhanced Mode 1 Code record
    pub national_origin: u16,                                       // 16-bit undefined enumeration
    pub supplemental_data: u8, // TODO                              // B.2.31 Mode 5 Transponder SD record
    pub navigation_source: NavigationSource,                        // UID 359
    pub figure_of_merit: u8,                                        // 8-bit uint between 0 and 31 decimal
}

#[derive(Default)]
pub enum OnOffStatus {
    #[default]
    On,
    Off,
}

#[derive(Default)]
pub enum DamageStatus {
    #[default]
    NoDamage,       // 0
    Damaged,        // 1
}

#[derive(Default)]
pub enum MalfunctionStatus {
    #[default]
    NoMalfunction,  // 0
    Malfunction,    // 1
}

/// B.2.28 Mode 5 Message Formats record
#[derive(Default)]
pub struct Mode5MessageFormats {
    pub message_format_0: IffPresence, // 0 - Not Present, 1 - Present
    pub message_format_1: IffPresence,
    pub message_format_2: IffPresence,
    pub message_format_3: IffPresence,
    pub message_format_4: IffPresence,
    pub message_format_5: IffPresence,
    pub message_format_6: IffPresence,
    pub message_format_7: IffPresence,
    pub message_format_8: IffPresence,
    pub message_format_9: IffPresence,
    pub message_format_11: IffPresence,
    pub message_format_12: IffPresence,
    pub message_format_13: IffPresence,
    pub message_format_10: IffPresence,
    pub message_format_14: IffPresence,
    pub message_format_15: IffPresence,
    pub message_format_16: IffPresence,
    pub message_format_17: IffPresence,
    pub message_format_18: IffPresence,
    pub message_format_19: IffPresence,
    pub message_format_20: IffPresence,
    pub message_format_21: IffPresence,
    pub message_format_22: IffPresence,
    pub message_format_23: IffPresence,
    pub message_format_24: IffPresence,
    pub message_format_25: IffPresence,
    pub message_format_26: IffPresence,
    pub message_format_27: IffPresence,
    pub message_format_28: IffPresence,
    pub message_format_29: IffPresence,
    pub message_format_30: IffPresence,
    pub message_format_31: IffPresence,
}


/// B.2.32 Mode 5 Transponder Status record
#[derive(Default)]
pub struct Mode5TransponderStatus {
    // TODO
}

/// B.2.52 System Status record
pub struct SystemStatus {
    pub system_on_off_status: bool,
    pub parameter_1_capable: ParameterCapable,
    pub parameter_2_capable: ParameterCapable,
    pub parameter_3_capable: ParameterCapable,
    pub parameter_4_capable: ParameterCapable,
    pub parameter_5_capable: ParameterCapable,
    pub parameter_6_capable: ParameterCapable,
    pub operational_status: ParameterCapable,
}

impl IffDataSpecification {
    pub fn new() -> Self {
        Self {
            iff_data_records: vec![],
        }
    }

    pub fn data_length(&self) -> u16 {
        const NUMBER_OF_DATA_RECORDS_OCTETS: u16 = 2;
        let iff_data_records_data_length: u16 = self.iff_data_records.iter().map(|record|record.data_length()).sum();
        NUMBER_OF_DATA_RECORDS_OCTETS + iff_data_records_data_length
    }
}

/// 6.2.43 IFF Data Specification record
#[derive(Default)]
pub struct IffDataRecord {
    pub record_type: VariableRecordType,   // UID 66
    pub record_specific_fields: Vec<u8>,
}

impl IffDataRecord {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn data_length(&self) -> u16 {
        length_padded_to_num_bytes(
            SIX_OCTETS + self.record_specific_fields.len(),
            FOUR_OCTETS)
            .record_length_bytes as u16
    }
}

// B.2.37
#[derive(Default)]
pub struct ModeSInterrogatorBasicData {
    pub mode_s_interrogator_status: ModeSInterrogatorStatus,
    pub mode_s_levels_present: ModeSLevelsPresent,
}

#[derive(Default)]
pub struct ModeSInterrogatorStatus {
    pub on_off_status: OnOffStatus,
    pub transmit_state: ModeSTransmitState,
    pub damage_status: DamageStatus,
    pub malfunction_status: MalfunctionStatus,
}

#[derive(Default)]
pub struct ModeSLevelsPresent {
    pub level_1: IffPresence,
    pub level_2_els: IffPresence,
    pub level_2_ehs: IffPresence,
    pub level_3: IffPresence,
    pub level_4: IffPresence,
}

#[derive(Default)]
pub enum IffPresence {
    #[default]
    NotPresent, // 0
    Present,    // 1
}

// B.2.41
pub struct ModeSTransponderBasicData {
    pub status: ModeSTransponderStatus,
    pub levels_present: ModeSLevelsPresent,
    pub aircraft_present_domain: AircraftPresentDomain,
    pub aircraft_identification: String,        // B.2.35 - String of length 8, in ASCII.
    pub aircraft_address: u32,
    pub aircraft_identification_type: AircraftIdentificationType,
    pub dap_source: DapSource,                  // B.2.6
    pub altitude: ModeSAltitude,                // B.2.36
    pub capability_record: CapabilityReport,
}

// B.2.6
// Downlink of Aircraft Parameters
#[derive(Default)]
pub struct DapSource {
    pub indicated_air_speed: DapValue,
    pub mach_number: DapValue,
    pub ground_speed: DapValue,
    pub magnetic_heading: DapValue,
    pub track_angle_rate: DapValue,
    pub true_track_angle: DapValue,
    pub true_airspeed: DapValue,
    pub vertical_rate: DapValue,
}

#[derive(Default)]
pub struct ModeSAltitude {
    pub altitude: u16,
    pub resolution: Mode5SAltitudeResolution,
}

#[derive(Default)]
pub enum DapValue {
    #[default]
    ComputeLocally,         // 0
    DataRecordAvailable,    // 1
}

// B.2.42
#[derive(Default)]
pub struct ModeSTransponderStatus {
    pub squitter_status: SquitterStatus,
    pub squitter_type: ModeSSquitterType,
    pub squitter_record_source: ModeSSquitterRecordSource,
    pub airborne_position_report_indicator: IffPresence,
    pub airborne_velocity_report_indicator: IffPresence,
    pub surface_position_report_indicator: IffPresence,
    pub identification_report_indicator: IffPresence,
    pub event_driven_report_indicator: IffPresence,
    pub on_off_status: OnOffStatus,
    pub damage_status: DamageStatus,
    pub malfunction_status: MalfunctionStatus,
}

#[derive(Default)]
pub enum SquitterStatus {
    #[default]
    Off,    // 0
    On,     // 1
}

pub struct IffLayer5 {
    pub layer_header: LayerHeader,
    pub reporting_simulation: SimulationAddress,
    pub applicable_layers: InformationLayers,
    pub data_category: DataCategory,
    pub data_records: IffDataSpecification,
}

impl Default for IffLayer5 {
    fn default() -> Self {
        Self {
            layer_header: LayerHeader { layer_number: 5, ..Default::default() },
            reporting_simulation: Default::default(),
            applicable_layers: Default::default(),
            data_category: Default::default(),
            data_records: Default::default(),
        }
    }
}

impl IffLayer5 {
    pub fn data_length(&self) -> u16 {
        const LAYER_5_BASE_DATA_LENGTH_OCTETS: u16 = 14;
        LAYER_5_BASE_DATA_LENGTH_OCTETS + self.data_records.data_length()
    }
}