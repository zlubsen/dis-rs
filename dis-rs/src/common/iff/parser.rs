use crate::common::iff::model::{
    ChangeOptionsRecord, DamageStatus, DapSource, DapValue, EnabledStatus, EnhancedMode1Code,
    FundamentalOperationalData, Iff, IffDataRecord, IffDataSpecification,
    IffFundamentalParameterData, IffLayer2, IffLayer3, IffLayer4, IffLayer5, IffPresence,
    InformationLayers, LatLonAltSource, LayerHeader, LayersPresenceApplicability,
    MalfunctionStatus, Mode5BasicData, Mode5InterrogatorBasicData, Mode5InterrogatorStatus,
    Mode5MessageFormats, Mode5TransponderBasicData, Mode5TransponderStatus,
    Mode5TransponderSupplementalData, ModeSAltitude, ModeSBasicData, ModeSInterrogatorBasicData,
    ModeSInterrogatorStatus, ModeSLevelsPresent, ModeSTransponderBasicData, ModeSTransponderStatus,
    OnOffStatus, OperationalStatus, ParameterCapable, SquitterStatus, SystemId, SystemSpecificData,
    SystemStatus, BASE_IFF_DATA_RECORD_LENGTH_OCTETS,
};
use crate::common::model::PduBody;
use crate::common::parser::{beam_data, entity_id, event_id, simulation_address, vec3_f32};
use crate::common::DisError;
use crate::constants::EIGHT_OCTETS;
use crate::enumerations::{
    AircraftIdentificationType, AircraftPresentDomain, CapabilityReport, DataCategory,
    IffApplicableModes, IffSystemMode, IffSystemName, IffSystemType, NavigationSource,
    VariableRecordType,
};
use nom::bytes::complete::take;
use nom::multi::count;
use nom::number::complete::{be_f32, be_u16, be_u32, be_u8};
use nom::IResult;

pub(crate) fn iff_body(input: &[u8]) -> IResult<&[u8], PduBody> {
    let (input, entity_id) = entity_id(input)?;
    let (input, event_id) = event_id(input)?;
    let (input, antenna_location) = vec3_f32(input)?;
    let (input, system_id) = system_id(input)?;
    let (input, system_designator) = be_u8(input)?;
    let (input, system_specific_data) = be_u8(input)?;
    let (input, fundamental_data) = fundamental_operational_data(input)?;

    let builder = Iff::builder();

    let (input, builder) = if fundamental_data.information_layers.layer_2
        == LayersPresenceApplicability::PresentApplicable
    {
        let (input, layer_2) = iff_layer_2(input)?;
        (input, builder.with_layer_2(layer_2))
    } else {
        (input, builder)
    };
    let (input, builder) = if fundamental_data.information_layers.layer_3
        == LayersPresenceApplicability::PresentApplicable
    {
        let (input, layer_3) = iff_layer_3(&system_id.system_type)(input)?;
        (input, builder.with_layer_3(layer_3))
    } else {
        (input, builder)
    };
    let (input, builder) = if fundamental_data.information_layers.layer_4
        == LayersPresenceApplicability::PresentApplicable
    {
        let (input, layer_4) = iff_layer_4(&system_id.system_type)(input)?;
        (input, builder.with_layer_4(layer_4))
    } else {
        (input, builder)
    };
    let (input, builder) = if fundamental_data.information_layers.layer_5
        == LayersPresenceApplicability::PresentApplicable
    {
        let (input, layer_5) = iff_layer_5(input)?;
        (input, builder.with_layer_5(layer_5))
    } else {
        (input, builder)
    };

    let builder = builder
        .with_emitting_entity_id(entity_id)
        .with_event_id(event_id)
        .with_relative_antenna_location(antenna_location)
        .with_system_id(system_id)
        .with_system_designator(system_designator)
        .with_system_specific_data(system_specific_data)
        .with_fundamental_operational_data(fundamental_data);

    Ok((input, builder.build().into_pdu_body()))
}

fn iff_layer_2(input: &[u8]) -> IResult<&[u8], IffLayer2> {
    let (input, layer_header) = layer_header(input)?;
    let (input, beam_data) = beam_data(input)?;
    let (input, operational_parameter_1) = be_u8(input)?;
    let (input, operational_parameter_2) = be_u8(input)?;
    let (input, num_params) = be_u16(input)?;
    let (input, fundamental_parameters) =
        count(iff_fundamental_parameter_data, num_params.into())(input)?;

    Ok((
        input,
        IffLayer2::builder()
            .with_header(layer_header)
            .with_beam_data(beam_data)
            .with_operational_parameter_1(operational_parameter_1)
            .with_operational_parameter_2(operational_parameter_2)
            .with_iff_fundamental_parameters(fundamental_parameters)
            .build(),
    ))
}

fn iff_layer_3(system_type: &IffSystemType) -> impl Fn(&[u8]) -> IResult<&[u8], IffLayer3> + '_ {
    move |input: &[u8]| {
        let (input, layer_header) = layer_header(input)?;
        let (input, reporting_simulation) = simulation_address(input)?;
        let (input, basic_data) = mode_5_basic_data(system_type)(input)?;
        let (input, _padding) = be_u16(input)?;
        let (input, data_specification) = iff_data_specification(input)?;

        Ok((
            input,
            IffLayer3::builder()
                .with_header(layer_header)
                .with_reporting_simulation(reporting_simulation)
                // TODO when we cannot match the system type, we insert the default Basic Data (transponder)
                .with_mode_5_basic_data(basic_data.unwrap_or(Mode5BasicData::new_transponder(
                    Mode5TransponderBasicData::default(),
                )))
                .with_iff_data_specification(data_specification)
                .build(),
        ))
    }
}

fn iff_layer_4(system_type: &IffSystemType) -> impl Fn(&[u8]) -> IResult<&[u8], IffLayer4> + '_ {
    move |input: &[u8]| {
        let (input, layer_header) = layer_header(input)?;
        let (input, reporting_simulation) = simulation_address(input)?;
        let (input, basic_data) = mode_s_basic_data(system_type)(input)?;
        let (input, _padding) = be_u16(input)?;
        let (input, data_specification) = iff_data_specification(input)?;

        Ok((
            input,
            IffLayer4::builder()
                .with_header(layer_header)
                .with_reporting_simulation(reporting_simulation)
                // TODO when we cannot match the system type, we insert the default Basic Data (transponder)
                .with_mode_s_basic_data(basic_data.unwrap_or(ModeSBasicData::Transponder(
                    ModeSTransponderBasicData::default(),
                )))
                .with_iff_data_specification(data_specification)
                .build(),
        ))
    }
}

fn iff_layer_5(input: &[u8]) -> IResult<&[u8], IffLayer5> {
    let (input, layer_header) = layer_header(input)?;
    let (input, reporting_simulation) = simulation_address(input)?;
    let (input, _padding) = be_u16(input)?;
    let (input, applicable_layers) = information_layers(input)?;
    let (input, data_category) = be_u8(input)?;
    let data_category = DataCategory::from(data_category);
    let (input, _padding) = be_u16(input)?;
    let (input, data_specification) = iff_data_specification(input)?;

    Ok((
        input,
        IffLayer5::builder()
            .with_header(layer_header)
            .with_reporting_simulation(reporting_simulation)
            .with_applicable_layers(applicable_layers)
            .with_data_category(data_category)
            .with_iff_data_specification(data_specification)
            .build(),
    ))
}

fn change_options_record(input: &[u8]) -> IResult<&[u8], ChangeOptionsRecord> {
    let (input, record) = be_u8(input)?;

    Ok((input, ChangeOptionsRecord::from(record)))
}

fn fundamental_operational_data(input: &[u8]) -> IResult<&[u8], FundamentalOperationalData> {
    let (input, system_status) = system_status(input)?;
    let (input, data_field_1) = be_u8(input)?;
    let (input, information_layers) = information_layers(input)?;
    let (input, data_field_2) = be_u8(input)?;
    let (input, parameter_1) = be_u16(input)?;
    let (input, parameter_2) = be_u16(input)?;
    let (input, parameter_3) = be_u16(input)?;
    let (input, parameter_4) = be_u16(input)?;
    let (input, parameter_5) = be_u16(input)?;
    let (input, parameter_6) = be_u16(input)?;

    Ok((
        input,
        FundamentalOperationalData::builder()
            .with_system_status(system_status)
            .with_data_field_1(data_field_1)
            .with_information_layers(information_layers)
            .with_data_field_2(data_field_2)
            .with_parameter_1(parameter_1)
            .with_parameter_2(parameter_2)
            .with_parameter_3(parameter_3)
            .with_parameter_4(parameter_4)
            .with_parameter_5(parameter_5)
            .with_parameter_6(parameter_6)
            .build(),
    ))
}

fn iff_data_record(input: &[u8]) -> IResult<&[u8], IffDataRecord> {
    let (input, record_type) = be_u32(input)?;
    let record_type = VariableRecordType::from(record_type);
    let (input, record_length) = be_u16(input)?;
    let (input, field) =
        take(record_length.saturating_sub(BASE_IFF_DATA_RECORD_LENGTH_OCTETS))(input)?;

    Ok((
        input,
        IffDataRecord::builder()
            .with_record_type(record_type)
            .with_record_specific_field(field.to_vec())
            .build(),
    ))
}

fn iff_data_specification(input: &[u8]) -> IResult<&[u8], IffDataSpecification> {
    let (input, num_records) = be_u16(input)?;
    let (input, records) = count(iff_data_record, num_records.into())(input)?;

    Ok((
        input,
        IffDataSpecification::builder()
            .with_iff_data_records(records)
            .build(),
    ))
}

fn information_layers(input: &[u8]) -> IResult<&[u8], InformationLayers> {
    let (input, record) = be_u8(input)?;

    Ok((input, InformationLayers::from(record)))
}

fn iff_fundamental_parameter_data(input: &[u8]) -> IResult<&[u8], IffFundamentalParameterData> {
    let (input, erp) = be_f32(input)?;
    let (input, frequency) = be_f32(input)?;
    let (input, pgrf) = be_f32(input)?;
    let (input, pulse_width) = be_f32(input)?;
    let (input, burst_length) = be_f32(input)?;
    let (input, applicable_modes) = be_u8(input)?;
    let applicable_modes = IffApplicableModes::from(applicable_modes);
    let (input, system_specific_data) = system_specific_data(input)?;

    Ok((
        input,
        IffFundamentalParameterData::builder()
            .with_erp(erp)
            .with_frequency(frequency)
            .with_pgrf(pgrf)
            .with_pulse_width(pulse_width)
            .with_burst_length(burst_length)
            .with_applicable_modes(applicable_modes)
            .with_system_specific_data(system_specific_data)
            .build(),
    ))
}

fn layer_header(input: &[u8]) -> IResult<&[u8], LayerHeader> {
    let (input, layer_number) = be_u8(input)?;
    let (input, layer_specific_information) = be_u8(input)?;
    let (input, length) = be_u16(input)?;

    Ok((
        input,
        LayerHeader::builder()
            .with_layer_number(layer_number)
            .with_layer_specific_information(layer_specific_information)
            .with_length(length)
            .build(),
    ))
}

fn system_specific_data(input: &[u8]) -> IResult<&[u8], SystemSpecificData> {
    let (input, part_1) = be_u8(input)?;
    let (input, part_2) = be_u8(input)?;
    let (input, part_3) = be_u8(input)?;

    Ok((
        input,
        SystemSpecificData::builder()
            .with_part_1(part_1)
            .with_part_2(part_2)
            .with_part_3(part_3)
            .build(),
    ))
}

fn system_id(input: &[u8]) -> IResult<&[u8], SystemId> {
    let (input, system_type) = be_u16(input)?;
    let system_type = IffSystemType::from(system_type);
    let (input, system_name) = be_u16(input)?;
    let system_name = IffSystemName::from(system_name);
    let (input, system_mode) = be_u8(input)?;
    let system_mode = IffSystemMode::from(system_mode);
    let (input, change_options_record) = change_options_record(input)?;

    Ok((
        input,
        SystemId::builder()
            .with_system_type(system_type)
            .with_system_name(system_name)
            .with_system_mode(system_mode)
            .with_change_options(change_options_record)
            .build(),
    ))
}

fn dap_source(input: &[u8]) -> IResult<&[u8], DapSource> {
    let (input, record) = be_u8(input)?;

    Ok((input, DapSource::from(record)))
}

impl From<u8> for DapValue {
    fn from(value: u8) -> Self {
        match value {
            0 => DapValue::ComputeLocally,
            _ => DapValue::DataRecordAvailable,
        }
    }
}

fn enhanced_mode_1_code(input: &[u8]) -> IResult<&[u8], EnhancedMode1Code> {
    let (input, record) = be_u16(input)?;

    Ok((input, EnhancedMode1Code::from(record)))
}

fn system_status(input: &[u8]) -> IResult<&[u8], SystemStatus> {
    let (input, record) = be_u8(input)?;

    Ok((input, SystemStatus::from(record)))
}

// TODO This bit of error handling the correct system type to parse is not that nice.
fn mode_5_basic_data(
    system_type: &IffSystemType,
) -> impl Fn(&[u8]) -> IResult<&[u8], Result<Mode5BasicData, DisError>> + '_ {
    move |input: &[u8]| match system_type {
        IffSystemType::MarkXXIIATCRBSTransponder
        | IffSystemType::SovietTransponder
        | IffSystemType::RRBTransponder
        | IffSystemType::MarkXIIATransponder
        | IffSystemType::Mode5Transponder
        | IffSystemType::ModeSTransponder => {
            let (input, basic_data) = mode_5_transponder_basic_data(input)?;
            Ok((input, Ok(Mode5BasicData::Transponder(basic_data))))
        }
        IffSystemType::MarkXXIIATCRBSInterrogator
        | IffSystemType::SovietInterrogator
        | IffSystemType::MarkXIIAInterrogator
        | IffSystemType::Mode5Interrogator
        | IffSystemType::ModeSInterrogator => {
            let (input, basic_data) = mode_5_interrogator_basic_data(input)?;
            Ok((input, Ok(Mode5BasicData::Interrogator(basic_data))))
        }
        IffSystemType::MarkXIIACombinedInterrogatorTransponder_CIT_
        | IffSystemType::MarkXIICombinedInterrogatorTransponder_CIT_
        | IffSystemType::TCASACASTransceiver => {
            Ok((input, Err(DisError::IffUndeterminedSystemType)))
        }
        IffSystemType::NotUsed_InvalidValue_ => Ok((input, Err(DisError::IffIncorrectSystemType))),
        IffSystemType::Unspecified(_) => Ok((input, Err(DisError::IffIncorrectSystemType))),
    }
}

fn mode_5_interrogator_basic_data(input: &[u8]) -> IResult<&[u8], Mode5InterrogatorBasicData> {
    let (input, status) = mode_5_interrogator_status(input)?;
    let (input, _padding) = be_u8(input)?;
    let (input, _padding) = be_u16(input)?;
    let (input, message_formats) = mode_5_message_formats(input)?;
    let (input, entity_id) = entity_id(input)?;
    let (input, _padding) = be_u16(input)?;

    Ok((
        input,
        Mode5InterrogatorBasicData::builder()
            .with_status(status)
            .with_mode_5_message_formats_present(message_formats)
            .with_interrogated_entity_id(entity_id)
            .build(),
    ))
}

fn mode_5_interrogator_status(input: &[u8]) -> IResult<&[u8], Mode5InterrogatorStatus> {
    let (input, record) = be_u8(input)?;

    Ok((input, Mode5InterrogatorStatus::from(record)))
}

fn mode_5_message_formats(input: &[u8]) -> IResult<&[u8], Mode5MessageFormats> {
    let (input, record) = be_u32(input)?;

    Ok((input, Mode5MessageFormats::from(record)))
}

fn mode_5_transponder_basic_data(input: &[u8]) -> IResult<&[u8], Mode5TransponderBasicData> {
    let (input, status) = mode_5_transponder_status(input)?;
    let (input, pin) = be_u16(input)?;
    let (input, message_formats_present) = mode_5_message_formats(input)?;
    let (input, enhanced_mode_1) = enhanced_mode_1_code(input)?;
    let (input, national_origin) = be_u16(input)?;
    let (input, supplemental_data) = mode_5_transponder_supplemental_data(input)?;
    let (input, navigation_source) = be_u8(input)?;
    let navigation_source = NavigationSource::from(navigation_source);
    let (input, figure_of_merit) = be_u8(input)?;
    let (input, _padding) = be_u8(input)?;

    Ok((
        input,
        Mode5TransponderBasicData::builder()
            .with_status(status)
            .with_pin(pin)
            .with_mode_5_message_formats_present(message_formats_present)
            .with_enhanced_mode_1(enhanced_mode_1)
            .with_national_origin(national_origin)
            .with_supplemental_data(supplemental_data)
            .with_navigation_source(navigation_source)
            .with_figure_of_merit(figure_of_merit)
            .build(),
    ))
}

fn mode_5_transponder_supplemental_data(
    input: &[u8],
) -> IResult<&[u8], Mode5TransponderSupplementalData> {
    let (input, record) = be_u8(input)?;

    Ok((input, Mode5TransponderSupplementalData::from(record)))
}

fn mode_5_transponder_status(input: &[u8]) -> IResult<&[u8], Mode5TransponderStatus> {
    let (input, record) = be_u16(input)?;

    Ok((input, Mode5TransponderStatus::from(record)))
}

fn mode_s_altitude(input: &[u8]) -> IResult<&[u8], ModeSAltitude> {
    let (input, record) = be_u16(input)?;

    Ok((input, ModeSAltitude::from(record)))
}

// TODO This bit of error handling the correct system type to parse is not that nice.
fn mode_s_basic_data(
    system_type: &IffSystemType,
) -> impl Fn(&[u8]) -> IResult<&[u8], Result<ModeSBasicData, DisError>> + '_ {
    move |input: &[u8]| match system_type {
        IffSystemType::MarkXXIIATCRBSTransponder
        | IffSystemType::SovietTransponder
        | IffSystemType::RRBTransponder
        | IffSystemType::MarkXIIATransponder
        | IffSystemType::Mode5Transponder
        | IffSystemType::ModeSTransponder => {
            let (input, basic_data) = mode_s_transponder_basic_data(input)?;
            Ok((input, Ok(ModeSBasicData::Transponder(basic_data))))
        }
        IffSystemType::MarkXXIIATCRBSInterrogator
        | IffSystemType::SovietInterrogator
        | IffSystemType::MarkXIIAInterrogator
        | IffSystemType::Mode5Interrogator
        | IffSystemType::ModeSInterrogator => {
            let (input, basic_data) = mode_s_interrogator_basic_data(input)?;
            Ok((input, Ok(ModeSBasicData::Interrogator(basic_data))))
        }
        IffSystemType::MarkXIIACombinedInterrogatorTransponder_CIT_
        | IffSystemType::MarkXIICombinedInterrogatorTransponder_CIT_
        | IffSystemType::TCASACASTransceiver => {
            Ok((input, Err(DisError::IffUndeterminedSystemType)))
        }
        IffSystemType::NotUsed_InvalidValue_ => Ok((input, Err(DisError::IffIncorrectSystemType))),
        IffSystemType::Unspecified(_) => Ok((input, Err(DisError::IffIncorrectSystemType))),
    }
}

fn mode_s_interrogator_basic_data(input: &[u8]) -> IResult<&[u8], ModeSInterrogatorBasicData> {
    const PAD_168_BITS_IN_OCTETS: usize = 21;

    let (input, status) = mode_s_interrogator_status(input)?;
    let (input, _padding_1_octet) = be_u8(input)?;
    let (input, levels_present) = mode_s_levels_present(input)?;
    let (input, _padding_21_octets) = take(PAD_168_BITS_IN_OCTETS)(input)?;

    Ok((
        input,
        ModeSInterrogatorBasicData::builder()
            .with_mode_s_interrogator_status(status)
            .with_mode_s_levels_present(levels_present)
            .build(),
    ))
}

fn mode_s_interrogator_status(input: &[u8]) -> IResult<&[u8], ModeSInterrogatorStatus> {
    let (input, record) = be_u8(input)?;

    Ok((input, ModeSInterrogatorStatus::from(record)))
}

fn mode_s_levels_present(input: &[u8]) -> IResult<&[u8], ModeSLevelsPresent> {
    let (input, record) = be_u8(input)?;

    Ok((input, ModeSLevelsPresent::from(record)))
}

fn mode_s_transponder_basic_data(input: &[u8]) -> IResult<&[u8], ModeSTransponderBasicData> {
    let (input, status) = mode_s_transponder_status(input)?;
    let (input, levels_present) = mode_s_levels_present(input)?;
    let (input, aircraft_present_domain) = be_u8(input)?;
    let aircraft_present_domain = AircraftPresentDomain::from(aircraft_present_domain);

    let mut buf: [u8; EIGHT_OCTETS] = [0; EIGHT_OCTETS];
    let (input, _) = nom::multi::fill(be_u8, &mut buf)(input)?;

    let mut aircraft_id = String::from_utf8_lossy(&buf[..]).into_owned();
    aircraft_id.truncate(
        aircraft_id
            .trim_end()
            .trim_end_matches(|c: char| !c.is_alphanumeric())
            .len(),
    );

    let (input, aircraft_address) = be_u32(input)?;
    let (input, aircraft_identification_type) = be_u8(input)?;
    let aircraft_identification_type =
        AircraftIdentificationType::from(aircraft_identification_type);
    let (input, dap_source) = dap_source(input)?;
    let (input, altitude) = mode_s_altitude(input)?;
    let (input, capability_report) = be_u8(input)?;
    let capability_report = CapabilityReport::from(capability_report);

    Ok((
        input,
        ModeSTransponderBasicData::builder()
            .with_status(status)
            .with_levels_present(levels_present)
            .with_aircraft_present_domain(aircraft_present_domain)
            .with_aircraft_identification(aircraft_id)
            .with_aircraft_address(aircraft_address)
            .with_aircraft_identification_type(aircraft_identification_type)
            .with_dap_source(dap_source)
            .with_altitude(altitude)
            .with_capability_report(capability_report)
            .build(),
    ))
}

fn mode_s_transponder_status(input: &[u8]) -> IResult<&[u8], ModeSTransponderStatus> {
    let (input, record) = be_u16(input)?;

    Ok((input, ModeSTransponderStatus::from(record)))
}

impl From<u8> for OnOffStatus {
    fn from(value: u8) -> Self {
        match value {
            0 => OnOffStatus::Off,
            _ => OnOffStatus::On,
        }
    }
}

impl From<u8> for DamageStatus {
    fn from(value: u8) -> Self {
        match value {
            0 => DamageStatus::NoDamage,
            _ => DamageStatus::Damaged,
        }
    }
}

impl From<u8> for MalfunctionStatus {
    fn from(value: u8) -> Self {
        match value {
            0 => MalfunctionStatus::NoMalfunction,
            _ => MalfunctionStatus::Malfunction,
        }
    }
}

impl From<u8> for EnabledStatus {
    fn from(value: u8) -> Self {
        match value {
            0 => EnabledStatus::NotEnabled,
            _ => EnabledStatus::Enabled,
        }
    }
}

impl From<u8> for LatLonAltSource {
    fn from(value: u8) -> Self {
        match value {
            0 => LatLonAltSource::ComputeLocally,
            _ => LatLonAltSource::TransponderLocationDataRecordPresent,
        }
    }
}

impl From<u8> for IffPresence {
    fn from(value: u8) -> Self {
        match value {
            0 => IffPresence::NotPresent,
            _ => IffPresence::Present,
        }
    }
}

impl From<u8> for SquitterStatus {
    fn from(value: u8) -> Self {
        match value {
            0 => SquitterStatus::Off,
            _ => SquitterStatus::On,
        }
    }
}

impl From<u8> for ParameterCapable {
    fn from(value: u8) -> Self {
        match value {
            0 => ParameterCapable::Capable,
            _ => ParameterCapable::NotCapable,
        }
    }
}

impl From<u8> for OperationalStatus {
    fn from(value: u8) -> Self {
        match value {
            0 => OperationalStatus::Operational,
            _ => OperationalStatus::SystemFailed,
        }
    }
}

impl From<u8> for LayersPresenceApplicability {
    fn from(value: u8) -> Self {
        match value {
            0 => LayersPresenceApplicability::NotPresentApplicable,
            _ => LayersPresenceApplicability::PresentApplicable,
        }
    }
}
