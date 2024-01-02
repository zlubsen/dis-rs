use crate::common::iff::model::{ChangeOptionsRecord, DamageStatus, DapSource, DapValue, EnabledStatus, EnhancedMode1Code, FundamentalOperationalData, Iff, IffDataRecord, IffDataSpecification, IffFundamentalParameterData, IffLayer2, IffLayer3, IffLayer4, IffLayer5, IffPresence, InformationLayers, LatLonAltSource, LayerHeader, LayersPresenceApplicability, MalfunctionStatus, Mode5BasicData, Mode5InterrogatorBasicData, Mode5InterrogatorStatus, Mode5MessageFormats, Mode5TransponderBasicData, Mode5TransponderStatus, Mode5TransponderSupplementalData, ModeSAltitude, ModeSBasicData, ModeSInterrogatorBasicData, ModeSInterrogatorStatus, ModeSLevelsPresent, ModeSTransponderBasicData, ModeSTransponderStatus, OnOffStatus, OperationalStatus, ParameterCapable, SquitterStatus, SystemId, SystemSpecificData, SystemStatus};
use crate::common::model::EntityId;
use crate::{AircraftIdentificationType, AircraftPresentDomain, AntennaSelection, BeamData, CapabilityReport, DataCategory, EventId, IffApplicableModes, IffSystemMode, IffSystemName, IffSystemType, Level2SquitterStatus, Mode5IffMission, Mode5LevelSelection, Mode5LocationErrors, Mode5MessageFormatsStatus, Mode5PlatformType, Mode5Reply, Mode5SAltitudeResolution, ModeSSquitterRecordSource, ModeSSquitterType, ModeSTransmitState, NavigationSource, SimulationAddress, VariableRecordType, VectorF32};

pub struct IffBuilder(Iff);

impl IffBuilder {
    pub fn new() -> Self {
        IffBuilder(Iff::default())
    }

    pub fn with_emitting_entity_id(mut self, v: EntityId) -> Self {
        self.0.emitting_entity_id = v;
        self
    }

    pub fn with_event_id(mut self, v: EventId) -> Self {
        self.0.event_id = v;
        self
    }

    pub fn with_relative_antenna_location(mut self, v: VectorF32) -> Self {
        self.0.relative_antenna_location = v;
        self
    }

    pub fn with_system_id(mut self, v: SystemId) -> Self {
        self.0.system_id = v;
        self
    }

    pub fn with_system_designator(mut self, v: u8) -> Self {
        self.0.system_designator = v;
        self
    }

    pub fn with_system_specific_data(mut self, v: u8) -> Self {
        self.0.system_specific_data = v;
        self
    }

    pub fn with_fundamental_operational_data(mut self, v: FundamentalOperationalData) -> Self {
        self.0.fundamental_operational_data = v;
        self
    }

    pub fn with_layer_2(mut self, v: IffLayer2) -> Self {
        self.0.layer_2 = Some(v);
        self
    }

    pub fn with_layer_3(mut self, v: IffLayer3) -> Self {
        self.0.layer_3 = Some(v);
        self
    }

    pub fn with_layer_4(mut self, v: IffLayer4) -> Self {
        self.0.layer_4 = Some(v);
        self
    }

    pub fn with_layer_5(mut self, v: IffLayer5) -> Self {
        self.0.layer_5 = Some(v);
        self
    }

    pub fn build(self) -> Iff {
        self.0
    }
}

pub struct IffLayer2Builder(IffLayer2);

impl IffLayer2Builder {
    pub fn new() -> Self {
        IffLayer2Builder(IffLayer2::default())
    }

    pub fn with_header(mut self, v: LayerHeader) -> Self {
        self.0.layer_header = v;
        self
    }

    pub fn with_beam_data(mut self, v: BeamData) -> Self {
        self.0.beam_data = v;
        self
    }

    pub fn with_operational_parameter_1(mut self, v: u8) -> Self {
        self.0.operational_parameter_1 = v;
        self
    }

    pub fn with_operational_parameter_2(mut self, v: u8) -> Self {
        self.0.operational_parameter_2 = v;
        self
    }

    pub fn with_iff_fundamental_parameter(mut self, v: IffFundamentalParameterData) -> Self {
        self.0.iff_fundamental_parameters.push(v);
        self
    }

    pub fn with_iff_fundamental_parameters(mut self, v: Vec<IffFundamentalParameterData>) -> Self {
        self.0.iff_fundamental_parameters = v;
        self
    }

    pub fn build(self) -> IffLayer2 {
        self.0
    }
}

pub struct IffLayer3Builder(IffLayer3);

impl IffLayer3Builder {
    pub fn new() -> Self {
        IffLayer3Builder(IffLayer3::default())
    }

    pub fn with_header(mut self, v: LayerHeader) -> Self {
        self.0.layer_header = v;
        self
    }

    pub fn with_reporting_simulation(mut self, v: SimulationAddress) -> Self {
        self.0.reporting_simulation = v;
        self
    }

    pub fn with_mode_5_basic_data(mut self, v: Mode5BasicData) -> Self {
        self.0.mode_5_basic_data = v;
        self
    }

    pub fn with_iff_data_specification(mut self, v: IffDataSpecification) -> Self {
        self.0.data_records = v;
        self
    }

    pub fn build(self) -> IffLayer3 {
        self.0
    }
}

pub struct IffLayer4Builder(IffLayer4);

impl IffLayer4Builder {
    pub fn new() -> Self {
        IffLayer4Builder(IffLayer4::default())
    }

    pub fn with_header(mut self, v: LayerHeader) -> Self {
        self.0.layer_header = v;
        self
    }

    pub fn with_reporting_simulation(mut self, v: SimulationAddress) -> Self {
        self.0.reporting_simulation = v;
        self
    }

    pub fn with_mode_s_basic_data(mut self, v: ModeSBasicData) -> Self {
        self.0.mode_s_basic_data = v;
        self
    }

    pub fn with_iff_data_specification(mut self, v: IffDataSpecification) -> Self {
        self.0.data_records = v;
        self
    }

    pub fn build(self) -> IffLayer4 {
        self.0
    }
}

pub struct IffLayer5Builder(IffLayer5);

impl IffLayer5Builder {
    pub fn new() -> Self {
        IffLayer5Builder(IffLayer5::default())
    }

    pub fn with_header(mut self, v: LayerHeader) -> Self {
        self.0.layer_header = v;
        self
    }

    pub fn with_reporting_simulation(mut self, v: SimulationAddress) -> Self {
        self.0.reporting_simulation = v;
        self
    }

    pub fn with_applicable_layers(mut self, v: InformationLayers) -> Self {
        self.0.applicable_layers = v;
        self
    }

    pub fn with_data_category(mut self, v: DataCategory) -> Self {
        self.0.data_category = v;
        self
    }

    pub fn with_iff_data_specification(mut self, v: IffDataSpecification) -> Self {
        self.0.data_records = v;
        self
    }

    pub fn build(self) -> IffLayer5 {
        self.0
    }
}

pub struct ChangeOptionsRecordBuilder(ChangeOptionsRecord);

impl ChangeOptionsRecordBuilder {
    pub fn new() -> Self {
        Self(ChangeOptionsRecord::default())
    }

    pub fn set_change_indicator(mut self) -> Self {
        self.0.change_indicator = true;
        self
    }

    pub fn set_system_specific_field_1(mut self) -> Self {
        self.0.system_specific_field_1 = true;
        self
    }

    pub fn set_system_specific_field_2(mut self) -> Self {
        self.0.system_specific_field_2 = true;
        self
    }

    pub fn set_heartbeat_indicator(mut self) -> Self {
        self.0.heartbeat_indicator = true;
        self
    }

    pub fn set_transponder_interrogator_indicator(mut self) -> Self {
        self.0.transponder_interrogator_indicator = true;
        self
    }

    pub fn set_simulation_mode(mut self) -> Self {
        self.0.simulation_mode = true;
        self
    }

    pub fn set_interactive_capable(mut self) -> Self {
        self.0.interactive_capable = true;
        self
    }

    pub fn set_test_mode(mut self) -> Self {
        self.0.test_mode = true;
        self
    }

    pub fn build(self) -> ChangeOptionsRecord {
        self.0
    }
}

pub struct FundamentalOperationalDataBuilder(FundamentalOperationalData);

impl FundamentalOperationalDataBuilder {
    pub fn new() -> Self {
        Self(FundamentalOperationalData::default())
    }

    pub fn with_system_status(mut self, v: SystemStatus) -> Self {
        self.0.system_status = v;
        self
    }

    pub fn with_data_field_1(mut self, v: u8) -> Self {
        self.0.data_field_1 = v;
        self
    }

    pub fn with_information_layers(mut self, v: InformationLayers) -> Self {
        self.0.information_layers = v;
        self
    }

    pub fn with_data_field_2(mut self, v: u8) -> Self {
        self.0.data_field_2 = v;
        self
    }

    pub fn with_parameter_1(mut self, v: u16) -> Self {
        self.0.parameter_1 = v;
        self
    }

    pub fn with_parameter_2(mut self, v: u16) -> Self {
        self.0.parameter_2 = v;
        self
    }

    pub fn with_parameter_3(mut self, v: u16) -> Self {
        self.0.parameter_3 = v;
        self
    }

    pub fn with_parameter_4(mut self, v: u16) -> Self {
        self.0.parameter_4 = v;
        self
    }

    pub fn with_parameter_5(mut self, v: u16) -> Self {
        self.0.parameter_5 = v;
        self
    }

    pub fn with_parameter_6(mut self, v: u16) -> Self {
        self.0.parameter_6 = v;
        self
    }

    pub fn build(self) -> FundamentalOperationalData {
        self.0
    }
}

pub struct IffDataRecordBuilder(IffDataRecord);

impl IffDataRecordBuilder {
    pub fn new() -> Self {
        Self(IffDataRecord::default())
    }

    pub fn with_record_type(mut self, v: VariableRecordType) -> Self {
        self.0.record_type = v;
        self
    }

    pub fn with_record_specific_field(mut self, v: Vec<u8>) -> Self {
        self.0.record_specific_fields = v;
        self
    }

    pub fn build(self) -> IffDataRecord {
        self.0
    }
}

pub struct IffDataSpecificationBuilder(IffDataSpecification);

impl IffDataSpecificationBuilder {
    pub fn new() -> Self {
        Self(IffDataSpecification::default())
    }

    pub fn with_iff_data_record(mut self, v: IffDataRecord) -> Self {
        self.0.iff_data_records.push(v);
        self
    }

    pub fn with_iff_data_records(mut self, v: Vec<IffDataRecord>) -> Self {
        self.0.iff_data_records = v;
        self
    }

    pub fn build(self) -> IffDataSpecification {
        self.0
    }
}

pub struct InformationLayersBuilder(InformationLayers);

impl InformationLayersBuilder {
    pub fn new() -> Self {
        Self(InformationLayers::default())
    }

    pub fn with_layer_1(mut self, v: LayersPresenceApplicability) -> Self {
        self.0.layer_1 = v;
        self
    }

    pub fn with_layer_2(mut self, v: LayersPresenceApplicability) -> Self {
        self.0.layer_2 = v;
        self
    }

    pub fn with_layer_3(mut self, v: LayersPresenceApplicability) -> Self {
        self.0.layer_3 = v;
        self
    }

    pub fn with_layer_4(mut self, v: LayersPresenceApplicability) -> Self {
        self.0.layer_4 = v;
        self
    }

    pub fn with_layer_5(mut self, v: LayersPresenceApplicability) -> Self {
        self.0.layer_5 = v;
        self
    }

    pub fn with_layer_6(mut self, v: LayersPresenceApplicability) -> Self {
        self.0.layer_6 = v;
        self
    }

    pub fn with_layer_7(mut self, v: LayersPresenceApplicability) -> Self {
        self.0.layer_7 = v;
        self
    }

    pub fn build(self) -> InformationLayers {
        self.0
    }
}

pub struct IffFundamentalParameterDataBuilder(IffFundamentalParameterData);

impl IffFundamentalParameterDataBuilder {
    pub fn new() -> Self {
        Self(IffFundamentalParameterData::default())
    }

    pub fn with_erp(mut self, v: f32) -> Self {
        self.0.erp = v;
        self
    }

    pub fn with_frequency(mut self, v: f32) -> Self {
        self.0.frequency = v;
        self
    }

    pub fn with_pgrf(mut self, v: f32) -> Self {
        self.0.pgrf = v;
        self
    }

    pub fn with_pulse_width(mut self, v: f32) -> Self {
        self.0.pulse_width = v;
        self
    }

    pub fn with_burst_length(mut self, v: f32) -> Self {
        self.0.burst_length = v;
        self
    }

    pub fn with_applicable_modes(mut self, v: IffApplicableModes) -> Self {
        self.0.applicable_modes = v;
        self
    }

    pub fn with_system_specific_data(mut self, v: SystemSpecificData) -> Self {
        self.0.system_specific_data = v;
        self
    }

    pub fn build(self) -> IffFundamentalParameterData {
        self.0
    }
}

pub struct LayerHeaderBuilder(LayerHeader);

impl LayerHeaderBuilder {
    pub fn new() -> Self {
        Self(LayerHeader::default())
    }

    pub fn with_layer_number(mut self, v: u8) -> Self {
        self.0.layer_number = v;
        self
    }

    pub fn with_layer_specific_information(mut self, v: u8) -> Self {
        self.0.layer_specific_information = v;
        self
    }

    pub fn with_length(mut self, v: u16) -> Self {
        self.0.length = v;
        self
    }

    pub fn build(self) -> LayerHeader {
        self.0
    }
}

pub struct SystemSpecificDataBuilder(SystemSpecificData);

impl SystemSpecificDataBuilder {
    pub fn new() -> Self {
        Self(SystemSpecificData::default())
    }

    pub fn with_part_1(mut self, v: u8) -> Self {
        self.0.part_1 = v;
        self
    }

    pub fn with_part_2(mut self, v: u8) -> Self {
        self.0.part_2 = v;
        self
    }

    pub fn with_part_3(mut self, v: u8) -> Self {
        self.0.part_3 = v;
        self
    }

    pub fn build(self) -> SystemSpecificData {
        self.0
    }
}

pub struct SystemIdBuilder(SystemId);

impl SystemIdBuilder {
    pub fn new() -> Self {
        Self(SystemId::default())
    }

    pub fn with_system_type(mut self, v: IffSystemType) -> Self {
        self.0.system_type = v;
        self
    }

    pub fn with_system_name(mut self, v: IffSystemName) -> Self {
        self.0.system_name = v;
        self
    }

    pub fn with_system_mode(mut self, v: IffSystemMode) -> Self {
        self.0.system_mode = v;
        self
    }

    pub fn with_change_options(mut self, v: ChangeOptionsRecord) -> Self {
        self.0.change_options = v;
        self
    }

    pub fn build(self) -> SystemId {
        self.0
    }
}

pub struct DapSourceBuilder(DapSource);

impl DapSourceBuilder {
    pub fn new() -> Self {
        Self(DapSource::default())
    }

    pub fn with_indicated_air_speed(mut self, v: DapValue) -> Self {
        self.0.indicated_air_speed = v;
        self
    }

    pub fn with_mach_number(mut self, v: DapValue) -> Self {
        self.0.mach_number = v;
        self
    }

    pub fn with_ground_speed(mut self, v: DapValue) -> Self {
        self.0.ground_speed = v;
        self
    }

    pub fn with_magnetic_heading(mut self, v: DapValue) -> Self {
        self.0.magnetic_heading = v;
        self
    }

    pub fn with_track_angle_rate(mut self, v: DapValue) -> Self {
        self.0.track_angle_rate = v;
        self
    }

    pub fn with_true_track_angle(mut self, v: DapValue) -> Self {
        self.0.true_track_angle = v;
        self
    }

    pub fn with_true_airspeed(mut self, v: DapValue) -> Self {
        self.0.true_airspeed = v;
        self
    }

    pub fn with_vertical_rate(mut self, v: DapValue) -> Self {
        self.0.vertical_rate = v;
        self
    }

    pub fn build(self) -> DapSource {
        self.0
    }
}

pub struct EnhancedMode1CodeBuilder(EnhancedMode1Code);

impl EnhancedMode1CodeBuilder{
    pub fn new() -> Self {
        Self(EnhancedMode1Code::default())
    }

    pub fn with_code_element_1_d(mut self, v: u16) -> Self {
        self.0.code_element_1_d = v;
        self
    }

    pub fn with_code_element_2_c(mut self, v: u16) -> Self {
        self.0.code_element_2_c = v;
        self
    }

    pub fn with_code_element_3_b(mut self, v: u16) -> Self {
        self.0.code_element_3_b = v;
        self
    }

    pub fn with_code_element_4_a(mut self, v: u16) -> Self {
        self.0.code_element_4_a = v;
        self
    }

    pub fn with_on_off_status(mut self, v: OnOffStatus) -> Self {
        self.0.on_off_status = v;
        self
    }

    pub fn with_damage_status(mut self, v: DamageStatus) -> Self {
        self.0.damage_status = v;
        self
    }

    pub fn with_malfunction_status(mut self, v: MalfunctionStatus) -> Self {
        self.0.malfunction_status = v;
        self
    }

    pub fn build(self) -> EnhancedMode1Code {
        self.0
    }
}

pub struct Mode5InterrogatorBasicDataBuilder(Mode5InterrogatorBasicData);

impl Mode5InterrogatorBasicDataBuilder {
    pub fn new() -> Self {
        Self(Mode5InterrogatorBasicData::default())
    }

    pub fn with_status(mut self, v: Mode5InterrogatorStatus) -> Self {
        self.0.status = v;
        self
    }

    pub fn with_mode_5_message_formats_present(mut self, v: Mode5MessageFormats) -> Self {
        self.0.mode_5_message_formats_present = v;
        self
    }

    pub fn with_interrogated_entity_id(mut self, v: EntityId) -> Self {
        self.0.interrogated_entity_id = v;
        self
    }

    pub fn build(self) -> Mode5InterrogatorBasicData {
        self.0
    }
}

pub struct Mode5InterrogatorStatusBuilder(Mode5InterrogatorStatus);

impl Mode5InterrogatorStatusBuilder {
    pub fn new() -> Self {
        Self(Mode5InterrogatorStatus::default())
    }

    pub fn with_iff_mission(mut self, v: Mode5IffMission) -> Self {
        self.0.iff_mission = v;
        self
    }

    pub fn with_mode_5_message_formats_status(mut self, v: Mode5MessageFormatsStatus) -> Self {
        self.0.mode_5_message_formats_status = v;
        self
    }

    pub fn with_on_off_status(mut self, v: OnOffStatus) -> Self {
        self.0.on_off_status = v;
        self
    }

    pub fn with_damage_status(mut self, v: DamageStatus) -> Self {
        self.0.damage_status = v;
        self
    }

    pub fn with_malfunction_status(mut self, v: MalfunctionStatus) -> Self {
        self.0.malfunction_status = v;
        self
    }

    pub fn build(self) -> Mode5InterrogatorStatus {
        self.0
    }
}

pub struct Mode5MessageFormatsBuilder(Mode5MessageFormats);

impl Mode5MessageFormatsBuilder {
    pub fn new() -> Self {
        Self(Mode5MessageFormats::default())
    }

    pub fn with_message_format_0(mut self, v: IffPresence) -> Self {
        self.0.message_format_0 = v;
        self
    }

    pub fn with_message_format_1(mut self, v: IffPresence) -> Self {
        self.0.message_format_1 = v;
        self
    }

    pub fn with_message_format_2(mut self, v: IffPresence) -> Self {
        self.0.message_format_2 = v;
        self
    }

    pub fn with_message_format_3(mut self, v: IffPresence) -> Self {
        self.0.message_format_3 = v;
        self
    }

    pub fn with_message_format_4(mut self, v: IffPresence) -> Self {
        self.0.message_format_4 = v;
        self
    }

    pub fn with_message_format_5(mut self, v: IffPresence) -> Self {
        self.0.message_format_5 = v;
        self
    }

    pub fn with_message_format_6(mut self, v: IffPresence) -> Self {
        self.0.message_format_6 = v;
        self
    }

    pub fn with_message_format_7(mut self, v: IffPresence) -> Self {
        self.0.message_format_7 = v;
        self
    }

    pub fn with_message_format_8(mut self, v: IffPresence) -> Self {
        self.0.message_format_8 = v;
        self
    }

    pub fn with_message_format_9(mut self, v: IffPresence) -> Self {
        self.0.message_format_9 = v;
        self
    }

    pub fn with_message_format_10(mut self, v: IffPresence) -> Self {
        self.0.message_format_10 = v;
        self
    }

    pub fn with_message_format_11(mut self, v: IffPresence) -> Self {
        self.0.message_format_11 = v;
        self
    }

    pub fn with_message_format_12(mut self, v: IffPresence) -> Self {
        self.0.message_format_12 = v;
        self
    }

    pub fn with_message_format_13(mut self, v: IffPresence) -> Self {
        self.0.message_format_13 = v;
        self
    }

    pub fn with_message_format_14(mut self, v: IffPresence) -> Self {
        self.0.message_format_14 = v;
        self
    }

    pub fn with_message_format_15(mut self, v: IffPresence) -> Self {
        self.0.message_format_15 = v;
        self
    }

    pub fn with_message_format_16(mut self, v: IffPresence) -> Self {
        self.0.message_format_16 = v;
        self
    }

    pub fn with_message_format_17(mut self, v: IffPresence) -> Self {
        self.0.message_format_17 = v;
        self
    }

    pub fn with_message_format_18(mut self, v: IffPresence) -> Self {
        self.0.message_format_18 = v;
        self
    }

    pub fn with_message_format_19(mut self, v: IffPresence) -> Self {
        self.0.message_format_19 = v;
        self
    }

    pub fn with_message_format_20(mut self, v: IffPresence) -> Self {
        self.0.message_format_20 = v;
        self
    }

    pub fn with_message_format_21(mut self, v: IffPresence) -> Self {
        self.0.message_format_21 = v;
        self
    }

    pub fn with_message_format_22(mut self, v: IffPresence) -> Self {
        self.0.message_format_22 = v;
        self
    }

    pub fn with_message_format_23(mut self, v: IffPresence) -> Self {
        self.0.message_format_23 = v;
        self
    }

    pub fn with_message_format_24(mut self, v: IffPresence) -> Self {
        self.0.message_format_24 = v;
        self
    }

    pub fn with_message_format_25(mut self, v: IffPresence) -> Self {
        self.0.message_format_25 = v;
        self
    }

    pub fn with_message_format_26(mut self, v: IffPresence) -> Self {
        self.0.message_format_26 = v;
        self
    }

    pub fn with_message_format_27(mut self, v: IffPresence) -> Self {
        self.0.message_format_27 = v;
        self
    }

    pub fn with_message_format_28(mut self, v: IffPresence) -> Self {
        self.0.message_format_28 = v;
        self
    }

    pub fn with_message_format_29(mut self, v: IffPresence) -> Self {
        self.0.message_format_29 = v;
        self
    }

    pub fn with_message_format_30(mut self, v: IffPresence) -> Self {
        self.0.message_format_30 = v;
        self
    }

    pub fn with_message_format_31(mut self, v: IffPresence) -> Self {
        self.0.message_format_31 = v;
        self
    }

    pub fn build(self) -> Mode5MessageFormats {
        self.0
    }
}

pub struct Mode5TransponderBasicDataBuilder(Mode5TransponderBasicData);

impl Mode5TransponderBasicDataBuilder {
    pub fn new() -> Self {
        Self(Mode5TransponderBasicData::default())
    }

    pub fn with_status(mut self, v: Mode5TransponderStatus) -> Self {
        self.0.status = v;
        self
    }

    pub fn with_pin(mut self, v: u16) -> Self {
        self.0.pin = v;
        self
    }

    pub fn with_mode_5_message_formats_present(mut self, v: Mode5MessageFormats) -> Self {
        self.0.mode_5_message_formats_present = v;
        self
    }

    pub fn with_enhanced_mode_1(mut self, v: EnhancedMode1Code) -> Self {
        self.0.enhanced_mode_1 = v;
        self
    }

    pub fn with_national_origin(mut self, v: u16) -> Self {
        self.0.national_origin = v;
        self
    }

    pub fn with_supplemental_data(mut self, v: Mode5TransponderSupplementalData) -> Self {
        self.0.supplemental_data = v;
        self
    }

    pub fn with_navigation_source(mut self, v: NavigationSource) -> Self {
        self.0.navigation_source = v;
        self
    }

    pub fn with_figure_of_merit(mut self, v: u8) -> Self {
        self.0.figure_of_merit = v;
        self
    }

    pub fn build(self) -> Mode5TransponderBasicData {
        self.0
    }
}

pub struct Mode5TransponderSupplementalDataBuilder(Mode5TransponderSupplementalData);

impl Mode5TransponderSupplementalDataBuilder {
    pub fn new() -> Self {
        Self(Mode5TransponderSupplementalData::default())
    }

    pub fn with_squitter_on_off_status(mut self, v: SquitterStatus) -> Self {
        self.0.squitter_on_off_status = v;
        self
    }

    pub fn with_level_2_squitter_status(mut self, v: Level2SquitterStatus) -> Self {
        self.0.level_2_squitter_status = v;
        self
    }

    pub fn with_iff_mission(mut self, v: Mode5IffMission) -> Self {
        self.0.iff_mission = v;
        self
    }

    pub fn build(self) -> Mode5TransponderSupplementalData {
        self.0
    }
}

pub struct Mode5TransponderStatusBuilder(Mode5TransponderStatus);

impl Mode5TransponderStatusBuilder {
    pub fn new() -> Self {
        Self(Mode5TransponderStatus::default())
    }

    pub fn with_mode_5_reply(mut self, v: Mode5Reply) -> Self {
        self.0.mode_5_reply = v;
        self
    }

    pub fn with_line_test(mut self, v: EnabledStatus) -> Self {
        self.0.line_test = v;
        self
    }

    pub fn with_antenna_selection(mut self, v: AntennaSelection) -> Self {
        self.0.antenna_selection = v;
        self
    }

    pub fn with_crypto_control(mut self, v: IffPresence) -> Self {
        self.0.crypto_control = v;
        self
    }

    pub fn with_lat_lon_alt_source(mut self, v: LatLonAltSource) -> Self {
        self.0.lat_lon_alt_source = v;
        self
    }

    pub fn with_location_errors(mut self, v: Mode5LocationErrors) -> Self {
        self.0.location_errors = v;
        self
    }

    pub fn with_platform_type(mut self, v: Mode5PlatformType) -> Self {
        self.0.platform_type = v;
        self
    }

    pub fn with_mode_5_level_selection(mut self, v: Mode5LevelSelection) -> Self {
        self.0.mode_5_level_selection = v;
        self
    }

    pub fn with_on_off_status(mut self, v: OnOffStatus) -> Self {
        self.0.on_off_status = v;
        self
    }

    pub fn with_damage_status(mut self, v: DamageStatus) -> Self {
        self.0.damage_status = v;
        self
    }

    pub fn with_malfunction_status(mut self, v: MalfunctionStatus) -> Self {
        self.0.malfunction_status = v;
        self
    }

    pub fn build(self) -> Mode5TransponderStatus {
        self.0
    }
}

pub struct ModeSAltitudeBuilder(ModeSAltitude);

impl ModeSAltitudeBuilder {
    pub fn new() -> Self {
        Self(ModeSAltitude::default())
    }

    pub fn with_altitude(mut self, v: u16) -> Self {
        self.0.altitude = v;
        self
    }

    pub fn with_resolution(mut self, v: Mode5SAltitudeResolution) -> Self {
        self.0.resolution = v;
        self
    }

    pub fn build(self) -> ModeSAltitude {
        self.0
    }
}

pub struct ModeSInterrogatorBasicDataBuilder(ModeSInterrogatorBasicData);

impl ModeSInterrogatorBasicDataBuilder {
    pub fn new() -> Self {
        Self(ModeSInterrogatorBasicData::default())
    }

    pub fn with_mode_s_interrogator_status(mut self, v: ModeSInterrogatorStatus) -> Self {
        self.0.mode_s_interrogator_status = v;
        self
    }

    pub fn with_mode_s_levels_present(mut self, v: ModeSLevelsPresent) -> Self {
        self.0.mode_s_levels_present = v;
        self
    }

    pub fn build(self) -> ModeSInterrogatorBasicData {
        self.0
    }
}

pub struct ModeSInterrogatorStatusBuilder(ModeSInterrogatorStatus);

impl ModeSInterrogatorStatusBuilder {
    pub fn new() -> Self {
        Self(ModeSInterrogatorStatus::default())
    }

    pub fn with_on_off_status(mut self, v: OnOffStatus) -> Self {
        self.0.on_off_status = v;
        self
    }

    pub fn with_transmit_state(mut self, v: ModeSTransmitState) -> Self {
        self.0.transmit_state = v;
        self
    }

    pub fn with_damage_status(mut self, v: DamageStatus) -> Self {
        self.0.damage_status = v;
        self
    }

    pub fn with_malfunction_status(mut self, v: MalfunctionStatus) -> Self {
        self.0.malfunction_status = v;
        self
    }

    pub fn build(self) -> ModeSInterrogatorStatus {
        self.0
    }
}

pub struct ModeSLevelsPresentBuilder(ModeSLevelsPresent);

impl ModeSLevelsPresentBuilder {
    pub fn new() -> Self {
        Self(ModeSLevelsPresent::default())
    }

    pub fn with_level_1(mut self, v: IffPresence) -> Self {
        self.0.level_1 = v;
        self
    }

    pub fn with_level_2_els(mut self, v: IffPresence) -> Self {
        self.0.level_2_els = v;
        self
    }

    pub fn with_level_2_ehs(mut self, v: IffPresence) -> Self {
        self.0.level_2_ehs = v;
        self
    }

    pub fn with_level_3(mut self, v: IffPresence) -> Self {
        self.0.level_3 = v;
        self
    }

    pub fn with_level_4(mut self, v: IffPresence) -> Self {
        self.0.level_4 = v;
        self
    }

    pub fn build(self) -> ModeSLevelsPresent {
        self.0
    }
}

pub struct ModeSTransponderBasicDataBuilder(ModeSTransponderBasicData);

impl ModeSTransponderBasicDataBuilder {
    pub fn new() -> Self {
        Self(ModeSTransponderBasicData::default())
    }

    pub fn with_status(mut self, v: ModeSTransponderStatus) -> Self {
        self.0.status = v;
        self
    }

    pub fn with_levels_present(mut self, v: ModeSLevelsPresent) -> Self {
        self.0.levels_present = v;
        self
    }

    pub fn with_aircraft_present_domain(mut self, v: AircraftPresentDomain) -> Self {
        self.0.aircraft_present_domain = v;
        self
    }

    pub fn with_aircraft_identification(mut self, v: String) -> Self {
        self.0.aircraft_identification = v;
        self
    }

    pub fn with_aircraft_address(mut self, v: u32) -> Self {
        self.0.aircraft_address = v;
        self
    }

    pub fn with_aircraft_identification_type(mut self, v: AircraftIdentificationType) -> Self {
        self.0.aircraft_identification_type = v;
        self
    }

    pub fn with_dap_source(mut self, v: DapSource) -> Self {
        self.0.dap_source = v;
        self
    }

    pub fn with_altitude(mut self, v: ModeSAltitude) -> Self {
        self.0.altitude = v;
        self
    }

    pub fn with_capability_report(mut self, v: CapabilityReport) -> Self {
        self.0.capability_report = v;
        self
    }

    pub fn build(self) -> ModeSTransponderBasicData {
        self.0
    }
}

pub struct ModeSTransponderStatusBuilder(ModeSTransponderStatus);

impl ModeSTransponderStatusBuilder {
    pub fn new() -> Self {
        Self(ModeSTransponderStatus::default())
    }

    pub fn with_squitter_status(mut self, v: SquitterStatus) -> Self {
        self.0.squitter_status = v;
        self
    }

    pub fn with_squitter_type(mut self, v: ModeSSquitterType) -> Self {
        self.0.squitter_type = v;
        self
    }

    pub fn with_squitter_record_source(mut self, v: ModeSSquitterRecordSource) -> Self {
        self.0.squitter_record_source = v;
        self
    }

    pub fn with_airborne_position_report_indicator(mut self, v: IffPresence) -> Self {
        self.0.airborne_position_report_indicator = v;
        self
    }

    pub fn with_airborne_velocity_report_indicator(mut self, v: IffPresence) -> Self {
        self.0.airborne_velocity_report_indicator = v;
        self
    }

    pub fn with_surface_position_report_indicator(mut self, v: IffPresence) -> Self {
        self.0.surface_position_report_indicator = v;
        self
    }

    pub fn with_identification_report_indicator(mut self, v: IffPresence) -> Self {
        self.0.identification_report_indicator = v;
        self
    }

    pub fn with_event_driven_report_indicator(mut self, v: IffPresence) -> Self {
        self.0.event_driven_report_indicator = v;
        self
    }

    pub fn with_on_off_status(mut self, v: OnOffStatus) -> Self {
        self.0.on_off_status = v;
        self
    }

    pub fn with_damage_status(mut self, v: DamageStatus) -> Self {
        self.0.damage_status = v;
        self
    }

    pub fn with_malfunction_status(mut self, v: MalfunctionStatus) -> Self {
        self.0.malfunction_status = v;
        self
    }

    pub fn build(self) -> ModeSTransponderStatus {
        self.0
    }
}

pub struct SystemStatusBuilder(SystemStatus);

impl SystemStatusBuilder {
    pub fn new() -> Self {
        Self(SystemStatus::default())
    }

    pub fn with_system_on_off_status(mut self, v: OnOffStatus) -> Self {
        self.0.system_on_off_status = v;
        self
    }

    pub fn with_parameter_1_capable(mut self, v: ParameterCapable) -> Self {
        self.0.parameter_1_capable = v;
        self
    }

    pub fn with_parameter_2_capable(mut self, v: ParameterCapable) -> Self {
        self.0.parameter_2_capable = v;
        self
    }

    pub fn with_parameter_3_capable(mut self, v: ParameterCapable) -> Self {
        self.0.parameter_3_capable = v;
        self
    }

    pub fn with_parameter_4_capable(mut self, v: ParameterCapable) -> Self {
        self.0.parameter_4_capable = v;
        self
    }

    pub fn with_parameter_5_capable(mut self, v: ParameterCapable) -> Self {
        self.0.parameter_5_capable = v;
        self
    }

    pub fn with_parameter_6_capable(mut self, v: ParameterCapable) -> Self {
        self.0.parameter_6_capable = v;
        self
    }

    pub fn with_operational_status(mut self, v: OperationalStatus) -> Self {
        self.0.operational_status = v;
        self
    }

    pub fn build(self) -> SystemStatus {
        self.0
    }
}