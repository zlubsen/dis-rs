use crate::common::iff::builder::{
    ChangeOptionsRecordBuilder, DapSourceBuilder, EnhancedMode1CodeBuilder,
    FundamentalOperationalDataBuilder, IffBuilder, IffDataRecordBuilder,
    IffDataSpecificationBuilder, IffFundamentalParameterDataBuilder, IffLayer2Builder,
    IffLayer3Builder, IffLayer4Builder, IffLayer5Builder, InformationLayersBuilder,
    LayerHeaderBuilder, Mode5InterrogatorBasicDataBuilder, Mode5InterrogatorStatusBuilder,
    Mode5MessageFormatsBuilder, Mode5TransponderBasicDataBuilder, Mode5TransponderStatusBuilder,
    Mode5TransponderSupplementalDataBuilder, ModeSAltitudeBuilder,
    ModeSInterrogatorBasicDataBuilder, ModeSInterrogatorStatusBuilder, ModeSLevelsPresentBuilder,
    ModeSTransponderBasicDataBuilder, ModeSTransponderStatusBuilder, SystemIdBuilder,
    SystemSpecificDataBuilder, SystemStatusBuilder,
};
use crate::common::model::{
    length_padded_to_num, BeamData, EntityId, EventId, PduBody, SimulationAddress, VectorF32,
};
use crate::common::{BodyInfo, Interaction};
use crate::constants::{
    BIT_0_IN_BYTE, BIT_1_IN_BYTE, BIT_2_IN_BYTE, BIT_3_IN_BYTE, BIT_4_IN_BYTE, BIT_5_IN_BYTE,
    BIT_6_IN_BYTE, BIT_7_IN_BYTE, FOUR_OCTETS, SIX_OCTETS,
};
use crate::enumerations::{
    AircraftIdentificationType, AircraftPresentDomain, AntennaSelection, CapabilityReport,
    DataCategory, IffApplicableModes, IffSystemMode, IffSystemName, IffSystemType,
    Level2SquitterStatus, Mode5IffMission, Mode5LevelSelection, Mode5LocationErrors,
    Mode5MessageFormatsStatus, Mode5PlatformType, Mode5Reply, Mode5SAltitudeResolution,
    ModeSSquitterRecordSource, ModeSSquitterType, ModeSTransmitState, NavigationSource, PduType,
    VariableRecordType,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub const IFF_PDU_LAYER_1_DATA_LENGTH_OCTETS: u16 = 48;
pub const BASE_IFF_DATA_RECORD_LENGTH_OCTETS: u16 = 6;

/// 7.6.5 Identification Friend or Foe (IFF) PDU
///
/// 7.6.5.1 General
///
/// 7.6.5.2 Layer 1 basic system data
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Iff {
    pub emitting_entity_id: EntityId,
    pub event_id: EventId,
    pub relative_antenna_location: VectorF32,
    pub system_id: SystemId,
    pub system_designator: u8,    // See item d2) in 5.7.6.1.
    pub system_specific_data: u8, // 8-bit record defined by system type - See B.5
    pub fundamental_operational_data: FundamentalOperationalData, // see 6.2.39
    // Layer 1 up to here
    pub layer_2: Option<IffLayer2>, // 7.6.5.3 Layer 2 emissions data
    pub layer_3: Option<IffLayer3>, // Mode 5 Functional Data
    pub layer_4: Option<IffLayer4>, // Mode S Functional Data
    pub layer_5: Option<IffLayer5>, // Data Communications
}

impl Iff {
    #[must_use]
    pub fn builder() -> IffBuilder {
        IffBuilder::new()
    }

    #[must_use]
    pub fn into_builder(self) -> IffBuilder {
        IffBuilder::new_from_body(self)
    }

    #[must_use]
    pub fn into_pdu_body(self) -> PduBody {
        PduBody::IFF(self)
    }
}

impl BodyInfo for Iff {
    fn body_length(&self) -> u16 {
        IFF_PDU_LAYER_1_DATA_LENGTH_OCTETS
            + if let Some(layer_2) = &self.layer_2 {
                layer_2.data_length()
            } else {
                0
            }
            + if let Some(layer_3) = &self.layer_3 {
                layer_3.data_length()
            } else {
                0
            }
            + if let Some(layer_4) = &self.layer_4 {
                layer_4.data_length()
            } else {
                0
            }
            + if let Some(layer_5) = &self.layer_5 {
                layer_5.data_length()
            } else {
                0
            }
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
        None
    }
}

/// 7.6.5.3 Layer 2 emissions data
///
/// The Secondary Operational Data record (6.2.76) has been flattened in the `IffLayer2` struct, as it only
/// contains two 8-bit records.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
            layer_header: LayerHeader {
                layer_number: 2,
                ..Default::default()
            },
            beam_data: BeamData::default(),
            operational_parameter_1: 0,
            operational_parameter_2: 0,
            iff_fundamental_parameters: vec![IffFundamentalParameterData::default()],
        }
    }
}

impl IffLayer2 {
    #[must_use]
    pub fn builder() -> IffLayer2Builder {
        IffLayer2Builder::new()
    }

    #[must_use]
    pub fn data_length(&self) -> u16 {
        const LAYER_2_BASE_DATA_LENGTH_OCTETS: u16 = 28;
        const IFF_FUNDAMENTAL_PARAMETER_DATA_LENGTH_OCTETS: u16 = 24;
        LAYER_2_BASE_DATA_LENGTH_OCTETS
            + (self.iff_fundamental_parameters.len() as u16
                * IFF_FUNDAMENTAL_PARAMETER_DATA_LENGTH_OCTETS)
    }

    #[must_use]
    pub fn finalize_layer_header_length(mut self) -> Self {
        self.layer_header.length = self.data_length();
        self
    }
}

/// 7.6.5.4 Layer 3 Mode 5 formats
/// 7.6.5.4.2 Layer 3 Mode 5 Interrogator Format
/// 7.6.5.4.3 Layer 3 Mode 5 Transponder Format
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct IffLayer3 {
    pub layer_header: LayerHeader,
    pub reporting_simulation: SimulationAddress,
    pub mode_5_basic_data: Mode5BasicData,
    pub data_records: IffDataSpecification, // see 6.2.43 - page 299
}

impl Default for IffLayer3 {
    fn default() -> Self {
        Self {
            layer_header: LayerHeader {
                layer_number: 3,
                ..Default::default()
            },
            reporting_simulation: SimulationAddress::default(),
            mode_5_basic_data: Mode5BasicData::default(),
            data_records: IffDataSpecification::default(),
        }
    }
}

impl IffLayer3 {
    #[must_use]
    pub fn builder() -> IffLayer3Builder {
        IffLayer3Builder::new()
    }

    #[must_use]
    pub fn data_length(&self) -> u16 {
        const LAYER_3_BASE_DATA_LENGTH_OCTETS: u16 = 26;
        LAYER_3_BASE_DATA_LENGTH_OCTETS + self.data_records.data_length()
    }

    #[must_use]
    pub fn finalize_layer_header_length(mut self) -> Self {
        self.layer_header.length = self.data_length();
        self
    }
}

/// Custom defined enum to model having either an
/// Interrogator or a Transponder in an IFF Layer 3 Mode 5 PDU
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Mode5BasicData {
    Interrogator(Mode5InterrogatorBasicData), // 7.6.5.4.2 Layer 3 Mode 5 Interrogator Format
    Transponder(Mode5TransponderBasicData),   // 7.6.5.4.3 Layer 3 Mode 5 Transponder Format
}

impl Default for Mode5BasicData {
    fn default() -> Self {
        Self::Interrogator(Mode5InterrogatorBasicData::default())
    }
}

impl Mode5BasicData {
    #[must_use]
    pub fn new_interrogator(basic_data: Mode5InterrogatorBasicData) -> Self {
        Self::Interrogator(basic_data)
    }

    #[must_use]
    pub fn new_transponder(basic_data: Mode5TransponderBasicData) -> Self {
        Self::Transponder(basic_data)
    }
}

/// 7.6.5.5 Layer 4 Mode S formats
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct IffLayer4 {
    pub layer_header: LayerHeader,
    pub reporting_simulation: SimulationAddress,
    pub mode_s_basic_data: ModeSBasicData,
    pub data_records: IffDataSpecification, // see 6.2.43 - page 299
}

impl Default for IffLayer4 {
    fn default() -> Self {
        Self {
            layer_header: LayerHeader {
                layer_number: 4,
                ..Default::default()
            },
            reporting_simulation: SimulationAddress::default(),
            mode_s_basic_data: ModeSBasicData::default(),
            data_records: IffDataSpecification::default(),
        }
    }
}

impl IffLayer4 {
    #[must_use]
    pub fn builder() -> IffLayer4Builder {
        IffLayer4Builder::new()
    }

    #[must_use]
    pub fn data_length(&self) -> u16 {
        const LAYER_4_BASE_DATA_LENGTH_OCTETS: u16 = 34;
        LAYER_4_BASE_DATA_LENGTH_OCTETS + self.data_records.data_length()
    }

    #[must_use]
    pub fn finalize_layer_header_length(mut self) -> Self {
        self.layer_header.length = self.data_length();
        self
    }
}

/// Custom defined enum to model having either an
/// Interrogator or a Transponder in an IFF Layer 4 Mode S PDU
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum ModeSBasicData {
    Interrogator(ModeSInterrogatorBasicData), // 7.6.5.5.2 Layer 4 Mode S Interrogator Format
    Transponder(ModeSTransponderBasicData),   // 7.6.5.5.3 Layer 4 Mode S Transponder Format
}

impl Default for ModeSBasicData {
    fn default() -> Self {
        Self::Interrogator(ModeSInterrogatorBasicData::default())
    }
}

impl ModeSBasicData {
    #[must_use]
    pub fn new_interrogator(basic_data: ModeSInterrogatorBasicData) -> Self {
        Self::Interrogator(basic_data)
    }

    #[must_use]
    pub fn new_transponder(basic_data: ModeSTransponderBasicData) -> Self {
        Self::Transponder(basic_data)
    }
}

/// 7.6.5.6 Layer 5 data communications
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
            layer_header: LayerHeader {
                layer_number: 5,
                ..Default::default()
            },
            reporting_simulation: SimulationAddress::default(),
            applicable_layers: InformationLayers::default(),
            data_category: DataCategory::default(),
            data_records: IffDataSpecification::default(),
        }
    }
}

impl IffLayer5 {
    #[must_use]
    pub fn builder() -> IffLayer5Builder {
        IffLayer5Builder::new()
    }

    #[must_use]
    pub fn data_length(&self) -> u16 {
        const LAYER_5_BASE_DATA_LENGTH_OCTETS: u16 = 14;
        LAYER_5_BASE_DATA_LENGTH_OCTETS + self.data_records.data_length()
    }

    #[must_use]
    pub fn finalize_layer_header_length(mut self) -> Self {
        self.layer_header.length = self.data_length();
        self
    }
}

/// 6.2.13 Change/Options record
#[allow(clippy::struct_excessive_bools)]
#[derive(Copy, Clone, Default, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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

impl ChangeOptionsRecord {
    #[must_use]
    pub fn builder() -> ChangeOptionsRecordBuilder {
        ChangeOptionsRecordBuilder::new()
    }
}

impl From<u8> for ChangeOptionsRecord {
    fn from(record: u8) -> Self {
        let builder = ChangeOptionsRecord::builder();
        let builder = if ((record & BIT_0_IN_BYTE) >> 7) != 0 {
            builder.set_change_indicator()
        } else {
            builder
        };

        let builder = if ((record & BIT_1_IN_BYTE) >> 6) != 0 {
            builder.set_system_specific_field_1()
        } else {
            builder
        };

        let builder = if ((record & BIT_2_IN_BYTE) >> 5) != 0 {
            builder.set_system_specific_field_2()
        } else {
            builder
        };

        let builder = if ((record & BIT_3_IN_BYTE) >> 4) != 0 {
            builder.set_heartbeat_indicator()
        } else {
            builder
        };

        let builder = if ((record & BIT_4_IN_BYTE) >> 3) != 0 {
            builder.set_transponder_interrogator_indicator()
        } else {
            builder
        };

        let builder = if ((record & BIT_5_IN_BYTE) >> 2) != 0 {
            builder.set_simulation_mode()
        } else {
            builder
        };

        let builder = if ((record & BIT_6_IN_BYTE) >> 1) != 0 {
            builder.set_interactive_capable()
        } else {
            builder
        };

        let builder = if (record & BIT_7_IN_BYTE) != 0 {
            builder.set_test_mode()
        } else {
            builder
        };

        builder.build()
    }
}

impl From<&ChangeOptionsRecord> for u8 {
    fn from(value: &ChangeOptionsRecord) -> Self {
        let mut byte = 0u8;
        if value.change_indicator {
            byte += BIT_0_IN_BYTE;
        }
        if value.system_specific_field_1 {
            byte += BIT_1_IN_BYTE;
        }
        if value.system_specific_field_2 {
            byte += BIT_2_IN_BYTE;
        }
        if value.heartbeat_indicator {
            byte += BIT_3_IN_BYTE;
        }
        if value.transponder_interrogator_indicator {
            byte += BIT_4_IN_BYTE;
        }
        if value.simulation_mode {
            byte += BIT_5_IN_BYTE;
        }
        if value.interactive_capable {
            byte += BIT_6_IN_BYTE;
        }
        if value.test_mode {
            byte += BIT_7_IN_BYTE;
        }
        byte
    }
}

/// 6.2.39 Fundamental Operational Data record
#[derive(Clone, Default, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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

impl FundamentalOperationalData {
    #[must_use]
    pub fn builder() -> FundamentalOperationalDataBuilder {
        FundamentalOperationalDataBuilder::new()
    }
}

/// Custom defined enum to model the capability of a parameter in the
/// `FundamentalOperationalData` record.
#[derive(Copy, Clone, Default, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ParameterCapable {
    #[default]
    Capable,
    NotCapable,
}

/// Custom defined enum to model the capability of a parameter in the
/// `FundamentalOperationalData` record.
#[derive(Copy, Clone, Default, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum OperationalStatus {
    #[default]
    Operational,
    SystemFailed,
}

/// Custom defined enum to model the presence or applicability of an IFF layer
/// as used in IFF Layer 1.
#[derive(Copy, Clone, Default, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum LayersPresenceApplicability {
    #[default]
    NotPresentApplicable, // 0
    PresentApplicable, // 1
}

/// 6.2.43 IFF Data Specification record
#[derive(Clone, Default, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct IffDataRecord {
    pub record_type: VariableRecordType, // UID 66
    pub record_specific_fields: Vec<u8>,
}

impl IffDataRecord {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn builder() -> IffDataRecordBuilder {
        IffDataRecordBuilder::new()
    }

    #[must_use]
    pub fn data_length(&self) -> u16 {
        length_padded_to_num(SIX_OCTETS + self.record_specific_fields.len(), FOUR_OCTETS)
            .record_length as u16
    }
}

/// 6.2.43 IFF Data Specification record
#[derive(Clone, Default, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct IffDataSpecification {
    pub iff_data_records: Vec<IffDataRecord>,
}

impl IffDataSpecification {
    #[must_use]
    pub fn new() -> Self {
        Self {
            iff_data_records: vec![],
        }
    }

    #[must_use]
    pub fn builder() -> IffDataSpecificationBuilder {
        IffDataSpecificationBuilder::new()
    }

    #[must_use]
    pub fn data_length(&self) -> u16 {
        const NUMBER_OF_DATA_RECORDS_OCTETS: u16 = 2;
        let iff_data_records_data_length: u16 = self
            .iff_data_records
            .iter()
            .map(IffDataRecord::data_length)
            .sum();
        NUMBER_OF_DATA_RECORDS_OCTETS + iff_data_records_data_length
    }
}

/// 6.2.45 Information Layers record
#[derive(Copy, Clone, Default, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct InformationLayers {
    pub layer_1: LayersPresenceApplicability,
    pub layer_2: LayersPresenceApplicability,
    pub layer_3: LayersPresenceApplicability,
    pub layer_4: LayersPresenceApplicability,
    pub layer_5: LayersPresenceApplicability,
    pub layer_6: LayersPresenceApplicability,
    pub layer_7: LayersPresenceApplicability,
}

impl InformationLayers {
    #[must_use]
    pub fn builder() -> InformationLayersBuilder {
        InformationLayersBuilder::new()
    }
}

impl From<u8> for InformationLayers {
    fn from(record: u8) -> Self {
        let builder = InformationLayers::builder()
            .with_layer_1(LayersPresenceApplicability::from(
                (record & BIT_1_IN_BYTE) >> 6,
            ))
            .with_layer_2(LayersPresenceApplicability::from(
                (record & BIT_2_IN_BYTE) >> 5,
            ))
            .with_layer_3(LayersPresenceApplicability::from(
                (record & BIT_3_IN_BYTE) >> 4,
            ))
            .with_layer_4(LayersPresenceApplicability::from(
                (record & BIT_4_IN_BYTE) >> 3,
            ))
            .with_layer_5(LayersPresenceApplicability::from(
                (record & BIT_5_IN_BYTE) >> 2,
            ))
            .with_layer_6(LayersPresenceApplicability::from(
                (record & BIT_6_IN_BYTE) >> 1,
            ))
            .with_layer_7(LayersPresenceApplicability::from(record & BIT_7_IN_BYTE));

        builder.build()
    }
}

impl From<&InformationLayers> for u8 {
    fn from(value: &InformationLayers) -> Self {
        let layer_1 = u8::from(&value.layer_1) << 6;
        let layer_2 = u8::from(&value.layer_2) << 5;
        let layer_3 = u8::from(&value.layer_3) << 4;
        let layer_4 = u8::from(&value.layer_4) << 3;
        let layer_5 = u8::from(&value.layer_5) << 2;
        let layer_6 = u8::from(&value.layer_6) << 1;
        let layer_7 = u8::from(&value.layer_7);

        layer_1 | layer_2 | layer_3 | layer_4 | layer_5 | layer_6 | layer_7
    }
}

/// 6.2.44 IFF Fundamental Parameter Data Record
#[derive(Clone, Default, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct IffFundamentalParameterData {
    pub erp: f32,
    pub frequency: f32,
    pub pgrf: f32,
    pub pulse_width: f32,
    pub burst_length: f32,
    pub applicable_modes: IffApplicableModes,
    pub system_specific_data: SystemSpecificData,
}

impl IffFundamentalParameterData {
    #[must_use]
    pub fn builder() -> IffFundamentalParameterDataBuilder {
        IffFundamentalParameterDataBuilder::new()
    }
}

/// 6.2.51 Layer Header
#[derive(Clone, Default, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LayerHeader {
    pub layer_number: u8,
    pub layer_specific_information: u8,
    pub length: u16,
}

impl LayerHeader {
    #[must_use]
    pub fn new() -> Self {
        LayerHeader::default()
    }

    #[must_use]
    pub fn builder() -> LayerHeaderBuilder {
        LayerHeaderBuilder::new()
    }
}

// TODO placeholder for 24-bits - See Annex B.
#[derive(Copy, Clone, Default, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SystemSpecificData {
    pub part_1: u8,
    pub part_2: u8,
    pub part_3: u8,
}

impl SystemSpecificData {
    #[must_use]
    pub fn new() -> Self {
        SystemSpecificData::default()
    }

    #[must_use]
    pub fn builder() -> SystemSpecificDataBuilder {
        SystemSpecificDataBuilder::new()
    }
}

/// 6.2.87 System Identifier record
#[derive(Copy, Clone, Default, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SystemId {
    pub system_type: IffSystemType,
    pub system_name: IffSystemName,
    pub system_mode: IffSystemMode,
    pub change_options: ChangeOptionsRecord,
}

impl SystemId {
    #[must_use]
    pub fn new() -> Self {
        SystemId::default()
    }

    #[must_use]
    pub fn builder() -> SystemIdBuilder {
        SystemIdBuilder::new()
    }
}

/// B.2.6 DAP Source record
/// Downlink of Aircraft Parameters
#[derive(Clone, Default, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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

impl DapSource {
    #[must_use]
    pub fn new() -> Self {
        DapSource::default()
    }

    #[must_use]
    pub fn builder() -> DapSourceBuilder {
        DapSourceBuilder::new()
    }
}

impl From<u8> for DapSource {
    fn from(record: u8) -> Self {
        let indicated_air_speed = DapValue::from((record & BIT_0_IN_BYTE) >> 7);
        let mach_number = DapValue::from((record & BIT_1_IN_BYTE) >> 6);
        let ground_speed = DapValue::from((record & BIT_2_IN_BYTE) >> 5);
        let magnetic_heading = DapValue::from((record & BIT_3_IN_BYTE) >> 4);
        let track_angle_rate = DapValue::from((record & BIT_4_IN_BYTE) >> 3);
        let true_track_angle = DapValue::from((record & BIT_5_IN_BYTE) >> 2);
        let true_airspeed = DapValue::from((record & BIT_6_IN_BYTE) >> 1);
        let vertical_rate = DapValue::from(record & BIT_7_IN_BYTE);

        DapSource::builder()
            .with_indicated_air_speed(indicated_air_speed)
            .with_mach_number(mach_number)
            .with_ground_speed(ground_speed)
            .with_magnetic_heading(magnetic_heading)
            .with_track_angle_rate(track_angle_rate)
            .with_true_track_angle(true_track_angle)
            .with_true_airspeed(true_airspeed)
            .with_vertical_rate(vertical_rate)
            .build()
    }
}

impl From<&DapSource> for u8 {
    fn from(value: &DapSource) -> Self {
        let indicated_air_speed = u8::from(&value.indicated_air_speed) << 7;
        let mach_number = u8::from(&value.mach_number) << 6;
        let ground_speed = u8::from(&value.ground_speed) << 5;
        let magnetic_heading = u8::from(&value.magnetic_heading) << 4;
        let track_angle_rate = u8::from(&value.track_angle_rate) << 3;
        let true_track_angle = u8::from(&value.true_track_angle) << 2;
        let true_airspeed = u8::from(&value.true_airspeed) << 1;
        let vertical_rate = u8::from(&value.vertical_rate);

        indicated_air_speed
            | mach_number
            | ground_speed
            | magnetic_heading
            | track_angle_rate
            | true_track_angle
            | true_airspeed
            | vertical_rate
    }
}

/// Custom defined enum to model values in the DAP Source record
#[derive(Clone, Default, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DapValue {
    #[default]
    ComputeLocally, // 0
    DataRecordAvailable, // 1
}

/// B.2.9 Enhanced Mode 1 Code record
#[derive(Clone, Default, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EnhancedMode1Code {
    pub code_element_1_d: u16,
    pub code_element_2_c: u16,
    pub code_element_3_b: u16,
    pub code_element_4_a: u16,
    pub on_off_status: OnOffStatus,
    pub damage_status: DamageStatus,
    pub malfunction_status: MalfunctionStatus,
}

impl EnhancedMode1Code {
    #[must_use]
    pub fn new() -> Self {
        EnhancedMode1Code::default()
    }

    #[must_use]
    pub fn builder() -> EnhancedMode1CodeBuilder {
        EnhancedMode1CodeBuilder::new()
    }
}

impl From<u16> for EnhancedMode1Code {
    fn from(record: u16) -> Self {
        const BITS_0_2: u16 = 0xE000;
        const BITS_3_5: u16 = 0x1C00;
        const BITS_6_8: u16 = 0x0380;
        const BITS_9_11: u16 = 0x0070;
        const BITS_13: u16 = 0x0004;
        const BITS_14: u16 = 0x0002;
        const BITS_15: u16 = 0x0001;

        let code_element_1_d = (record & BITS_0_2) >> 13;
        let code_element_2_c = (record & BITS_3_5) >> 10;
        let code_element_3_b = (record & BITS_6_8) >> 7;
        let code_element_4_a = (record & BITS_9_11) >> 4;
        let on_off_status = OnOffStatus::from(((record & BITS_13) >> 2) as u8);
        let damage_status = DamageStatus::from(((record & BITS_14) >> 1) as u8);
        let malfunction_status = MalfunctionStatus::from((record & BITS_15) as u8);

        EnhancedMode1Code::builder()
            .with_code_element_1_d(code_element_1_d)
            .with_code_element_2_c(code_element_2_c)
            .with_code_element_3_b(code_element_3_b)
            .with_code_element_4_a(code_element_4_a)
            .with_on_off_status(on_off_status)
            .with_damage_status(damage_status)
            .with_malfunction_status(malfunction_status)
            .build()
    }
}

impl From<&EnhancedMode1Code> for u16 {
    fn from(value: &EnhancedMode1Code) -> Self {
        let code_element_1: u16 = value.code_element_1_d << 13;
        let code_element_2: u16 = value.code_element_2_c << 10;
        let code_element_3: u16 = value.code_element_3_b << 7;
        let code_element_4: u16 = value.code_element_4_a << 4;
        let on_off_status: u16 = u16::from(u8::from(&value.on_off_status)) << 2;
        let damage_status: u16 = u16::from(u8::from(&value.damage_status)) << 1;
        let malfunction_status: u16 = u16::from(u8::from(&value.malfunction_status));

        code_element_1
            | code_element_2
            | code_element_3
            | code_element_4
            | on_off_status
            | damage_status
            | malfunction_status
    }
}

/// B.2.26 Mode 5 Interrogator Basic Data record
#[derive(Clone, Default, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Mode5InterrogatorBasicData {
    pub status: Mode5InterrogatorStatus, // B.2.27 Mode 5 Interrogator Status record - page 592
    pub mode_5_message_formats_present: Mode5MessageFormats, // B.2.28 Mode 5 Message Formats record - page 592
    pub interrogated_entity_id: EntityId,
}

impl Mode5InterrogatorBasicData {
    #[must_use]
    pub fn new() -> Self {
        Mode5InterrogatorBasicData::default()
    }

    #[must_use]
    pub fn builder() -> Mode5InterrogatorBasicDataBuilder {
        Mode5InterrogatorBasicDataBuilder::new()
    }
}

/// B.2.27 Mode 5 Interrogator Status record
#[derive(Clone, Default, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Mode5InterrogatorStatus {
    pub iff_mission: Mode5IffMission,
    pub mode_5_message_formats_status: Mode5MessageFormatsStatus,
    pub on_off_status: OnOffStatus,
    pub damage_status: DamageStatus,
    pub malfunction_status: MalfunctionStatus,
}

impl Mode5InterrogatorStatus {
    #[must_use]
    pub fn new() -> Self {
        Mode5InterrogatorStatus::default()
    }

    #[must_use]
    pub fn builder() -> Mode5InterrogatorStatusBuilder {
        Mode5InterrogatorStatusBuilder::new()
    }
}

impl From<u8> for Mode5InterrogatorStatus {
    fn from(record: u8) -> Self {
        const BITS_0_2: u8 = 0xE0;
        let iff_mission = Mode5IffMission::from((record & BITS_0_2) >> 5);
        let mode_5_message_formats_status =
            Mode5MessageFormatsStatus::from((record & BIT_3_IN_BYTE) >> 4);
        let on_off_status = OnOffStatus::from((record & BIT_5_IN_BYTE) >> 2);
        let damage_status = DamageStatus::from((record & BIT_6_IN_BYTE) >> 1);
        let malfunction_status = MalfunctionStatus::from(record & BIT_7_IN_BYTE);

        Mode5InterrogatorStatus::builder()
            .with_iff_mission(iff_mission)
            .with_mode_5_message_formats_status(mode_5_message_formats_status)
            .with_on_off_status(on_off_status)
            .with_damage_status(damage_status)
            .with_malfunction_status(malfunction_status)
            .build()
    }
}

impl From<&Mode5InterrogatorStatus> for u8 {
    fn from(value: &Mode5InterrogatorStatus) -> Self {
        let iff_mission: u8 = u8::from(value.iff_mission) << 5;
        let message_formats_status: u8 = u8::from(value.mode_5_message_formats_status) << 4;
        let on_off_status: u8 = u8::from(&value.on_off_status) << 2;
        let damage_status: u8 = u8::from(&value.damage_status) << 1;
        let malfunction_status: u8 = u8::from(&value.malfunction_status);

        iff_mission | message_formats_status | on_off_status | damage_status | malfunction_status
    }
}

/// B.2.28 Mode 5 Message Formats record
#[derive(Clone, Default, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
    pub message_format_10: IffPresence,
    pub message_format_11: IffPresence,
    pub message_format_12: IffPresence,
    pub message_format_13: IffPresence,
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

impl Mode5MessageFormats {
    #[must_use]
    pub fn new() -> Self {
        Mode5MessageFormats::default()
    }

    #[must_use]
    pub fn builder() -> Mode5MessageFormatsBuilder {
        Mode5MessageFormatsBuilder::new()
    }
}

impl From<u32> for Mode5MessageFormats {
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::similar_names)]
    fn from(record: u32) -> Self {
        let format_0 = IffPresence::from(((record >> 31) as u8) & BIT_7_IN_BYTE);
        let format_1 = IffPresence::from(((record >> 30) as u8) & BIT_7_IN_BYTE);
        let format_2 = IffPresence::from(((record >> 29) as u8) & BIT_7_IN_BYTE);
        let format_3 = IffPresence::from(((record >> 28) as u8) & BIT_7_IN_BYTE);
        let format_4 = IffPresence::from(((record >> 27) as u8) & BIT_7_IN_BYTE);
        let format_5 = IffPresence::from(((record >> 26) as u8) & BIT_7_IN_BYTE);
        let format_6 = IffPresence::from(((record >> 25) as u8) & BIT_7_IN_BYTE);
        let format_7 = IffPresence::from(((record >> 24) as u8) & BIT_7_IN_BYTE);
        let format_8 = IffPresence::from(((record >> 23) as u8) & BIT_7_IN_BYTE);
        let format_9 = IffPresence::from(((record >> 22) as u8) & BIT_7_IN_BYTE);
        let format_10 = IffPresence::from(((record >> 21) as u8) & BIT_7_IN_BYTE);
        let format_11 = IffPresence::from(((record >> 20) as u8) & BIT_7_IN_BYTE);
        let format_12 = IffPresence::from(((record >> 19) as u8) & BIT_7_IN_BYTE);
        let format_13 = IffPresence::from(((record >> 18) as u8) & BIT_7_IN_BYTE);
        let format_14 = IffPresence::from(((record >> 17) as u8) & BIT_7_IN_BYTE);
        let format_15 = IffPresence::from(((record >> 16) as u8) & BIT_7_IN_BYTE);
        let format_16 = IffPresence::from(((record >> 15) as u8) & BIT_7_IN_BYTE);
        let format_17 = IffPresence::from(((record >> 14) as u8) & BIT_7_IN_BYTE);
        let format_18 = IffPresence::from(((record >> 13) as u8) & BIT_7_IN_BYTE);
        let format_19 = IffPresence::from(((record >> 12) as u8) & BIT_7_IN_BYTE);
        let format_20 = IffPresence::from(((record >> 11) as u8) & BIT_7_IN_BYTE);
        let format_21 = IffPresence::from(((record >> 10) as u8) & BIT_7_IN_BYTE);
        let format_22 = IffPresence::from(((record >> 9) as u8) & BIT_7_IN_BYTE);
        let format_23 = IffPresence::from(((record >> 8) as u8) & BIT_7_IN_BYTE);
        let format_24 = IffPresence::from(((record >> 7) as u8) & BIT_7_IN_BYTE);
        let format_25 = IffPresence::from(((record >> 6) as u8) & BIT_7_IN_BYTE);
        let format_26 = IffPresence::from(((record >> 5) as u8) & BIT_7_IN_BYTE);
        let format_27 = IffPresence::from(((record >> 4) as u8) & BIT_7_IN_BYTE);
        let format_28 = IffPresence::from(((record >> 3) as u8) & BIT_7_IN_BYTE);
        let format_29 = IffPresence::from(((record >> 2) as u8) & BIT_7_IN_BYTE);
        let format_30 = IffPresence::from(((record >> 1) as u8) & BIT_7_IN_BYTE);
        let format_31 = IffPresence::from((record as u8) & BIT_7_IN_BYTE);

        Mode5MessageFormats::builder()
            .with_message_format_0(format_0)
            .with_message_format_1(format_1)
            .with_message_format_2(format_2)
            .with_message_format_3(format_3)
            .with_message_format_4(format_4)
            .with_message_format_5(format_5)
            .with_message_format_6(format_6)
            .with_message_format_7(format_7)
            .with_message_format_8(format_8)
            .with_message_format_9(format_9)
            .with_message_format_10(format_10)
            .with_message_format_11(format_11)
            .with_message_format_12(format_12)
            .with_message_format_13(format_13)
            .with_message_format_14(format_14)
            .with_message_format_15(format_15)
            .with_message_format_16(format_16)
            .with_message_format_17(format_17)
            .with_message_format_18(format_18)
            .with_message_format_19(format_19)
            .with_message_format_20(format_20)
            .with_message_format_21(format_21)
            .with_message_format_22(format_22)
            .with_message_format_23(format_23)
            .with_message_format_24(format_24)
            .with_message_format_25(format_25)
            .with_message_format_26(format_26)
            .with_message_format_27(format_27)
            .with_message_format_28(format_28)
            .with_message_format_29(format_29)
            .with_message_format_30(format_30)
            .with_message_format_31(format_31)
            .build()
    }
}

impl From<&Mode5MessageFormats> for u32 {
    #[allow(clippy::similar_names)]
    fn from(value: &Mode5MessageFormats) -> Self {
        let mf_0 = u32::from(&value.message_format_0) << 31;
        let mf_1 = u32::from(&value.message_format_1) << 30;
        let mf_2 = u32::from(&value.message_format_2) << 29;
        let mf_3 = u32::from(&value.message_format_3) << 28;
        let mf_4 = u32::from(&value.message_format_4) << 27;
        let mf_5 = u32::from(&value.message_format_5) << 26;
        let mf_6 = u32::from(&value.message_format_6) << 25;
        let mf_7 = u32::from(&value.message_format_7) << 24;
        let mf_8 = u32::from(&value.message_format_8) << 23;
        let mf_9 = u32::from(&value.message_format_9) << 22;
        let mf_10 = u32::from(&value.message_format_10) << 21;
        let mf_11 = u32::from(&value.message_format_11) << 20;
        let mf_12 = u32::from(&value.message_format_12) << 19;
        let mf_13 = u32::from(&value.message_format_13) << 18;
        let mf_14 = u32::from(&value.message_format_14) << 17;
        let mf_15 = u32::from(&value.message_format_15) << 16;
        let mf_16 = u32::from(&value.message_format_16) << 15;
        let mf_17 = u32::from(&value.message_format_17) << 14;
        let mf_18 = u32::from(&value.message_format_18) << 13;
        let mf_19 = u32::from(&value.message_format_19) << 12;
        let mf_20 = u32::from(&value.message_format_20) << 11;
        let mf_21 = u32::from(&value.message_format_21) << 10;
        let mf_22 = u32::from(&value.message_format_22) << 9;
        let mf_23 = u32::from(&value.message_format_23) << 8;
        let mf_24 = u32::from(&value.message_format_24) << 7;
        let mf_25 = u32::from(&value.message_format_25) << 6;
        let mf_26 = u32::from(&value.message_format_26) << 5;
        let mf_27 = u32::from(&value.message_format_27) << 4;
        let mf_28 = u32::from(&value.message_format_28) << 3;
        let mf_29 = u32::from(&value.message_format_29) << 2;
        let mf_30 = u32::from(&value.message_format_30) << 1;
        let mf_31 = u32::from(&value.message_format_31);

        mf_0 | mf_1
            | mf_2
            | mf_3
            | mf_4
            | mf_5
            | mf_6
            | mf_7
            | mf_8
            | mf_9
            | mf_10
            | mf_11
            | mf_12
            | mf_13
            | mf_14
            | mf_15
            | mf_16
            | mf_17
            | mf_18
            | mf_19
            | mf_20
            | mf_21
            | mf_22
            | mf_23
            | mf_24
            | mf_25
            | mf_26
            | mf_27
            | mf_28
            | mf_29
            | mf_30
            | mf_31
    }
}

/// B.2.29 Mode 5 Transponder Basic Data record
#[derive(Clone, Default, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Mode5TransponderBasicData {
    pub status: Mode5TransponderStatus,
    pub pin: u16,
    pub mode_5_message_formats_present: Mode5MessageFormats, // B.2.28 Mode 5 Message Formats record
    pub enhanced_mode_1: EnhancedMode1Code,                  // B.2.9 Enhanced Mode 1 Code record
    pub national_origin: u16,                                // 16-bit undefined enumeration
    pub supplemental_data: Mode5TransponderSupplementalData, // B.2.31 Mode 5 Transponder SD record
    pub navigation_source: NavigationSource,                 // UID 359
    pub figure_of_merit: u8,                                 // 8-bit uint between 0 and 31 decimal
}

impl Mode5TransponderBasicData {
    #[must_use]
    pub fn new() -> Self {
        Mode5TransponderBasicData::default()
    }

    #[must_use]
    pub fn builder() -> Mode5TransponderBasicDataBuilder {
        Mode5TransponderBasicDataBuilder::new()
    }
}

/// Custom defined enum to model a system being On or Off.
#[derive(Copy, Clone, Default, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum OnOffStatus {
    #[default]
    Off, // 0
    On, // 1
}

/// Custom defined enum to model a system being Not Damaged or Damaged.
#[derive(Clone, Default, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DamageStatus {
    #[default]
    NoDamage, // 0
    Damaged, // 1
}

/// Custom defined enum to model a system being Not Malfunctioning or Malfunctioning.
#[derive(Clone, Default, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MalfunctionStatus {
    #[default]
    NoMalfunction, // 0
    Malfunction, // 1
}

/// Custom defined enum to model a system being Not Enabled or Enabled.
#[derive(Clone, Default, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum EnabledStatus {
    #[default]
    NotEnabled, // 0
    Enabled, // 1
}

/// Custom defined enum to model the source of
/// Mode 5 latitude, longitude, and altitude information.
#[derive(Clone, Default, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum LatLonAltSource {
    #[default]
    ComputeLocally, // 0
    TransponderLocationDataRecordPresent, // 1
}

/// B.2.31 Mode 5 Transponder Supplemental Data (SD) record
#[derive(Clone, Default, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Mode5TransponderSupplementalData {
    pub squitter_on_off_status: SquitterStatus,
    pub level_2_squitter_status: Level2SquitterStatus,
    pub iff_mission: Mode5IffMission,
}

impl Mode5TransponderSupplementalData {
    #[must_use]
    pub fn new() -> Self {
        Mode5TransponderSupplementalData::default()
    }

    #[must_use]
    pub fn builder() -> Mode5TransponderSupplementalDataBuilder {
        Mode5TransponderSupplementalDataBuilder::new()
    }
}

impl From<u8> for Mode5TransponderSupplementalData {
    fn from(record: u8) -> Self {
        const BITS_2_4: u8 = 0x38;
        let squitter_status = SquitterStatus::from((record & BIT_0_IN_BYTE) >> 7);
        let level_2_squitter_status = Level2SquitterStatus::from((record & BIT_1_IN_BYTE) >> 6);
        let iff_mission = Mode5IffMission::from((record & BITS_2_4) >> 3);

        Mode5TransponderSupplementalData::builder()
            .with_squitter_on_off_status(squitter_status)
            .with_level_2_squitter_status(level_2_squitter_status)
            .with_iff_mission(iff_mission)
            .build()
    }
}

impl From<&Mode5TransponderSupplementalData> for u8 {
    fn from(value: &Mode5TransponderSupplementalData) -> Self {
        let squitter_status: u8 = u8::from(&value.squitter_on_off_status) << 7;
        let level_2_squitter_status: u8 = u8::from(value.level_2_squitter_status) << 6;
        let iff_mission: u8 = u8::from(value.iff_mission) << 3;

        squitter_status | level_2_squitter_status | iff_mission
    }
}

/// B.2.32 Mode 5 Transponder Status record
#[derive(Clone, Default, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Mode5TransponderStatus {
    pub mode_5_reply: Mode5Reply,
    pub line_test: EnabledStatus,
    pub antenna_selection: AntennaSelection,
    pub crypto_control: IffPresence,
    pub lat_lon_alt_source: LatLonAltSource,
    pub location_errors: Mode5LocationErrors,
    pub platform_type: Mode5PlatformType,
    pub mode_5_level_selection: Mode5LevelSelection,
    pub on_off_status: OnOffStatus,
    pub damage_status: DamageStatus,
    pub malfunction_status: MalfunctionStatus,
}

impl Mode5TransponderStatus {
    #[must_use]
    pub fn new() -> Self {
        Mode5TransponderStatus::default()
    }

    #[must_use]
    pub fn builder() -> Mode5TransponderStatusBuilder {
        Mode5TransponderStatusBuilder::new()
    }
}

impl From<u16> for Mode5TransponderStatus {
    fn from(record: u16) -> Self {
        const BITS_0_3: u16 = 0xF000;
        const BIT_4: u16 = 0x0800;
        const BITS_5_6: u16 = 0x0600;
        const BIT_7: u16 = 0x0100;
        const BIT_8: u16 = 0x0080;
        const BIT_9: u16 = 0x0040;
        const BIT_10: u16 = 0x0020;
        const BIT_11: u16 = 0x0010;
        const BIT_13: u16 = 0x0004;
        const BIT_14: u16 = 0x0002;
        const BIT_15: u16 = 0x0001;
        let mode_5_reply = Mode5Reply::from(((record & BITS_0_3) >> 12) as u8);
        let line_test = EnabledStatus::from(((record & BIT_4) >> 11) as u8);
        let antenna_selection = AntennaSelection::from(((record & BITS_5_6) >> 9) as u8);
        let crypto_control = IffPresence::from(((record & BIT_7) >> 8) as u8);
        let lat_lon_alt_source = LatLonAltSource::from(((record & BIT_8) >> 7) as u8);
        let location_errors = Mode5LocationErrors::from(((record & BIT_9) >> 6) as u8);
        let platform_type = Mode5PlatformType::from(((record & BIT_10) >> 5) as u8);
        let mode_5_level_selection = Mode5LevelSelection::from(((record & BIT_11) >> 4) as u8);
        let on_off_status = OnOffStatus::from(((record & BIT_13) >> 2) as u8);
        let damage_status = DamageStatus::from(((record & BIT_14) >> 1) as u8);
        let malfunction_status = MalfunctionStatus::from((record & BIT_15) as u8);

        Mode5TransponderStatus::builder()
            .with_mode_5_reply(mode_5_reply)
            .with_line_test(line_test)
            .with_antenna_selection(antenna_selection)
            .with_crypto_control(crypto_control)
            .with_lat_lon_alt_source(lat_lon_alt_source)
            .with_location_errors(location_errors)
            .with_platform_type(platform_type)
            .with_mode_5_level_selection(mode_5_level_selection)
            .with_on_off_status(on_off_status)
            .with_damage_status(damage_status)
            .with_malfunction_status(malfunction_status)
            .build()
    }
}

impl From<&Mode5TransponderStatus> for u16 {
    fn from(value: &Mode5TransponderStatus) -> Self {
        let mode_5_reply: u8 = u8::from(value.mode_5_reply) << 4;
        let line_test: u8 = u8::from(&value.line_test) << 3;
        let antenna_selection: u8 = u8::from(value.antenna_selection) << 1;
        let crypto_control: u8 = u32::from(&value.crypto_control) as u8;

        let byte_1 = mode_5_reply | line_test | antenna_selection | crypto_control;

        let lat_lon_alt_source: u8 = u8::from(&value.lat_lon_alt_source) << 7;
        let location_errors: u8 = u8::from(value.location_errors) << 6;
        let platform_type: u8 = u8::from(value.platform_type) << 5;
        let mode_5_level_selection: u8 = u8::from(value.mode_5_level_selection) << 4;
        let on_off_status: u8 = u8::from(&value.on_off_status) << 2;
        let damage_status: u8 = u8::from(&value.damage_status) << 1;
        let malfunction_status: u8 = (&value.malfunction_status).into();

        let byte_2 = lat_lon_alt_source
            | location_errors
            | platform_type
            | mode_5_level_selection
            | on_off_status
            | damage_status
            | malfunction_status;

        (u16::from(byte_1) << 8) | u16::from(byte_2)
    }
}

/// B.2.36 Mode S Altitude record
#[derive(Clone, Default, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ModeSAltitude {
    pub altitude: u16,
    pub resolution: Mode5SAltitudeResolution,
}

impl ModeSAltitude {
    #[must_use]
    pub fn new() -> Self {
        ModeSAltitude::default()
    }

    #[must_use]
    pub fn builder() -> ModeSAltitudeBuilder {
        ModeSAltitudeBuilder::new()
    }
}

impl From<u16> for ModeSAltitude {
    fn from(record: u16) -> Self {
        const BITS_0_10: u16 = 0xFFE0;
        const BIT_11: u16 = 0x0010;
        let altitude = (record & BITS_0_10) >> 5;
        let resolution = Mode5SAltitudeResolution::from(((record & BIT_11) as u8) >> 4);

        ModeSAltitude::builder()
            .with_altitude(altitude)
            .with_resolution(resolution)
            .build()
    }
}

impl From<&ModeSAltitude> for u16 {
    fn from(value: &ModeSAltitude) -> Self {
        let resolution: u8 = value.resolution.into();
        let resolution: u16 = u16::from(resolution);

        (value.altitude << 5) | (resolution << 4)
    }
}

/// B.2.37 Mode S Interrogator Basic Data record
#[derive(Clone, Default, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ModeSInterrogatorBasicData {
    pub mode_s_interrogator_status: ModeSInterrogatorStatus,
    pub mode_s_levels_present: ModeSLevelsPresent,
}

impl ModeSInterrogatorBasicData {
    #[must_use]
    pub fn new() -> Self {
        ModeSInterrogatorBasicData::default()
    }

    #[must_use]
    pub fn builder() -> ModeSInterrogatorBasicDataBuilder {
        ModeSInterrogatorBasicDataBuilder::new()
    }
}

/// B.2.39 Mode S Interrogator Status record
#[derive(Clone, Default, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ModeSInterrogatorStatus {
    pub on_off_status: OnOffStatus,
    pub transmit_state: ModeSTransmitState,
    pub damage_status: DamageStatus,
    pub malfunction_status: MalfunctionStatus,
}

impl ModeSInterrogatorStatus {
    #[must_use]
    pub fn new() -> Self {
        ModeSInterrogatorStatus::default()
    }

    #[must_use]
    pub fn builder() -> ModeSInterrogatorStatusBuilder {
        ModeSInterrogatorStatusBuilder::new()
    }
}

impl From<u8> for ModeSInterrogatorStatus {
    fn from(record: u8) -> Self {
        const BITS_1_3: u8 = 0x70;
        let on_off_status = OnOffStatus::from((record & BIT_0_IN_BYTE) >> 7);
        let transmit_state = ModeSTransmitState::from((record & BITS_1_3) >> 4);
        let damage_status = DamageStatus::from((record & BIT_4_IN_BYTE) >> 3);
        let malfunction_status = MalfunctionStatus::from((record & BIT_5_IN_BYTE) >> 2);

        ModeSInterrogatorStatus::builder()
            .with_on_off_status(on_off_status)
            .with_transmit_state(transmit_state)
            .with_damage_status(damage_status)
            .with_malfunction_status(malfunction_status)
            .build()
    }
}

impl From<&ModeSInterrogatorStatus> for u8 {
    fn from(value: &ModeSInterrogatorStatus) -> Self {
        let on_off_status: u8 = u8::from(&value.on_off_status) << 7;
        let transmit_state: u8 = u8::from(value.transmit_state) << 4;
        let damage_status: u8 = u8::from(&value.damage_status) << 3;
        let malfunction_status: u8 = u8::from(&value.malfunction_status) << 2;

        on_off_status | transmit_state | damage_status | malfunction_status
    }
}

/// B.2.40 Mode S Levels Present record
#[derive(Clone, Default, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ModeSLevelsPresent {
    pub level_1: IffPresence,
    pub level_2_els: IffPresence,
    pub level_2_ehs: IffPresence,
    pub level_3: IffPresence,
    pub level_4: IffPresence,
}

impl ModeSLevelsPresent {
    #[must_use]
    pub fn new() -> Self {
        ModeSLevelsPresent::default()
    }

    #[must_use]
    pub fn builder() -> ModeSLevelsPresentBuilder {
        ModeSLevelsPresentBuilder::new()
    }
}

impl From<u8> for ModeSLevelsPresent {
    #[allow(clippy::similar_names)]
    fn from(record: u8) -> Self {
        let level_1 = IffPresence::from((record & BIT_1_IN_BYTE) >> 6);
        let level_2_els = IffPresence::from((record & BIT_2_IN_BYTE) >> 5);
        let level_2_ehs = IffPresence::from((record & BIT_3_IN_BYTE) >> 4);
        let level_3 = IffPresence::from((record & BIT_4_IN_BYTE) >> 3);
        let level_4 = IffPresence::from((record & BIT_5_IN_BYTE) >> 2);

        ModeSLevelsPresent::builder()
            .with_level_1(level_1)
            .with_level_2_ehs(level_2_ehs)
            .with_level_2_els(level_2_els)
            .with_level_3(level_3)
            .with_level_4(level_4)
            .build()
    }
}

impl From<&ModeSLevelsPresent> for u8 {
    #[allow(clippy::similar_names)]
    fn from(value: &ModeSLevelsPresent) -> Self {
        let level_1: u8 = u8::from(&value.level_1) << 6;
        let level_2_els: u8 = u8::from(&value.level_2_els) << 5;
        let level_2_ehs: u8 = u8::from(&value.level_2_ehs) << 4;
        let level_3: u8 = u8::from(&value.level_3) << 3;
        let level_4: u8 = u8::from(&value.level_4) << 2;

        level_1 | level_2_els | level_2_ehs | level_3 | level_4
    }
}

/// Custom defined enum to model the presence of an element in an IFF system
#[derive(Clone, Default, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum IffPresence {
    #[default]
    NotPresent, // 0
    Present, // 1
}

/// B.2.41 Mode S Transponder Basic Data record
#[derive(Clone, Default, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ModeSTransponderBasicData {
    pub status: ModeSTransponderStatus,
    pub levels_present: ModeSLevelsPresent,
    pub aircraft_present_domain: AircraftPresentDomain,
    pub aircraft_identification: String, // B.2.35 - String of length 8, in ASCII.
    pub aircraft_address: u32,
    pub aircraft_identification_type: AircraftIdentificationType,
    pub dap_source: DapSource,   // B.2.6
    pub altitude: ModeSAltitude, // B.2.36
    pub capability_report: CapabilityReport,
}

impl ModeSTransponderBasicData {
    #[must_use]
    pub fn new() -> Self {
        ModeSTransponderBasicData::default()
    }

    #[must_use]
    pub fn builder() -> ModeSTransponderBasicDataBuilder {
        ModeSTransponderBasicDataBuilder::new()
    }
}

/// B.2.42 Mode S Transponder Status record
#[derive(Clone, Default, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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

impl ModeSTransponderStatus {
    #[must_use]
    pub fn new() -> Self {
        ModeSTransponderStatus::default()
    }

    #[must_use]
    pub fn builder() -> ModeSTransponderStatusBuilder {
        ModeSTransponderStatusBuilder::new()
    }
}

impl From<u16> for ModeSTransponderStatus {
    fn from(record: u16) -> Self {
        const BIT_0: u16 = 0x8000;
        const BITS_1_3: u16 = 0x7000;
        const BIT_4: u16 = 0x800;
        const BIT_5: u16 = 0x400;
        const BIT_6: u16 = 0x200;
        const BIT_7: u16 = 0x100;
        const BIT_8: u16 = 0x80;
        const BIT_9: u16 = 0x40;
        const BIT_13: u16 = 0x04;
        const BIT_14: u16 = 0x02;
        const BIT_15: u16 = 0x01;

        let squitter_status = SquitterStatus::from(((record & BIT_0) >> 15) as u8);
        let squitter_type = ModeSSquitterType::from(((record & BITS_1_3) >> 12) as u8);
        let squitter_record_source =
            ModeSSquitterRecordSource::from(((record & BIT_4) >> 11) as u8);
        let airborne_pos_ri = IffPresence::from(((record & BIT_5) >> 10) as u8);
        let airborne_vel_ri = IffPresence::from(((record & BIT_6) >> 9) as u8);
        let surface_pos_ri = IffPresence::from(((record & BIT_7) >> 8) as u8);
        let ident_ri = IffPresence::from(((record & BIT_8) >> 7) as u8);
        let event_driven_ri = IffPresence::from(((record & BIT_9) >> 6) as u8);
        let on_off_status = OnOffStatus::from(((record & BIT_13) >> 2) as u8);
        let damage_status = DamageStatus::from(((record & BIT_14) >> 1) as u8);
        let malfunction_status = MalfunctionStatus::from((record & BIT_15) as u8);

        ModeSTransponderStatus::builder()
            .with_squitter_status(squitter_status)
            .with_squitter_type(squitter_type)
            .with_squitter_record_source(squitter_record_source)
            .with_airborne_position_report_indicator(airborne_pos_ri)
            .with_airborne_velocity_report_indicator(airborne_vel_ri)
            .with_surface_position_report_indicator(surface_pos_ri)
            .with_identification_report_indicator(ident_ri)
            .with_event_driven_report_indicator(event_driven_ri)
            .with_on_off_status(on_off_status)
            .with_damage_status(damage_status)
            .with_malfunction_status(malfunction_status)
            .build()
    }
}

impl From<&ModeSTransponderStatus> for u16 {
    fn from(value: &ModeSTransponderStatus) -> Self {
        let squitter_status: u8 = u8::from(&value.squitter_status) << 7;
        let squitter_type: u8 = u8::from(value.squitter_type) << 4;
        let squitter_record_source: u8 = u8::from(value.squitter_record_source) << 3;
        let airborne_pos_ri: u8 = u8::from(&value.airborne_position_report_indicator) << 2;
        let airborne_vel_ri: u8 = u8::from(&value.airborne_velocity_report_indicator) << 1;
        let surface_pos_ri: u8 = u8::from(&value.surface_position_report_indicator);
        let byte_1 = u16::from(
            squitter_status
                | squitter_type
                | squitter_record_source
                | airborne_pos_ri
                | airborne_vel_ri
                | surface_pos_ri,
        );

        let ident_ri: u8 = u8::from(&value.identification_report_indicator) << 7;
        let event_driven_ri: u8 = u8::from(&value.event_driven_report_indicator) << 6;
        let on_off_status: u8 = u8::from(&value.on_off_status) << 2;
        let damage_status: u8 = u8::from(&value.damage_status) << 1;
        let malfunction_status: u8 = u8::from(&value.malfunction_status);
        let byte_2 = u16::from(
            ident_ri | event_driven_ri | on_off_status | damage_status | malfunction_status,
        );

        (byte_1 << 8) | byte_2
    }
}

/// Custom defined enum to model the `SquitterStatus`
#[derive(Clone, Default, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SquitterStatus {
    #[default]
    Off, // 0
    On, // 1
}

/// B.2.52 System Status record
#[derive(Copy, Clone, Default, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SystemStatus {
    pub system_on_off_status: OnOffStatus,
    pub parameter_1_capable: ParameterCapable,
    pub parameter_2_capable: ParameterCapable,
    pub parameter_3_capable: ParameterCapable,
    pub parameter_4_capable: ParameterCapable,
    pub parameter_5_capable: ParameterCapable,
    pub parameter_6_capable: ParameterCapable,
    pub operational_status: OperationalStatus,
}

impl SystemStatus {
    #[must_use]
    pub fn new() -> Self {
        SystemStatus::default()
    }

    #[must_use]
    pub fn builder() -> SystemStatusBuilder {
        SystemStatusBuilder::new()
    }
}

impl From<u8> for SystemStatus {
    fn from(record: u8) -> Self {
        let system_on_off_status = OnOffStatus::from((record & BIT_0_IN_BYTE) >> 7);
        let parameter_1_capable = ParameterCapable::from((record & BIT_1_IN_BYTE) >> 6);
        let parameter_2_capable = ParameterCapable::from((record & BIT_2_IN_BYTE) >> 5);
        let parameter_3_capable = ParameterCapable::from((record & BIT_3_IN_BYTE) >> 4);
        let parameter_4_capable = ParameterCapable::from((record & BIT_4_IN_BYTE) >> 3);
        let parameter_5_capable = ParameterCapable::from((record & BIT_5_IN_BYTE) >> 2);
        let parameter_6_capable = ParameterCapable::from((record & BIT_6_IN_BYTE) >> 1);
        let operational_status = OperationalStatus::from(record & BIT_7_IN_BYTE);

        SystemStatus::builder()
            .with_system_on_off_status(system_on_off_status)
            .with_parameter_1_capable(parameter_1_capable)
            .with_parameter_2_capable(parameter_2_capable)
            .with_parameter_3_capable(parameter_3_capable)
            .with_parameter_4_capable(parameter_4_capable)
            .with_parameter_5_capable(parameter_5_capable)
            .with_parameter_6_capable(parameter_6_capable)
            .with_operational_status(operational_status)
            .build()
    }
}

impl From<&SystemStatus> for u8 {
    fn from(value: &SystemStatus) -> Self {
        let system_on_off_status = u8::from(&value.system_on_off_status) << 7;
        let parameter_1 = u8::from(&value.parameter_1_capable) << 6;
        let parameter_2 = u8::from(&value.parameter_2_capable) << 5;
        let parameter_3 = u8::from(&value.parameter_3_capable) << 4;
        let parameter_4 = u8::from(&value.parameter_4_capable) << 3;
        let parameter_5 = u8::from(&value.parameter_5_capable) << 2;
        let parameter_6 = u8::from(&value.parameter_6_capable) << 1;
        let operational_status = u8::from(&value.operational_status);

        system_on_off_status
            | parameter_1
            | parameter_2
            | parameter_3
            | parameter_4
            | parameter_5
            | parameter_6
            | operational_status
    }
}
