use std::os::macos::raw::stat;
use nom::bytes::complete::take;
use nom::IResult;
use nom::multi::count;
use nom::number::complete::{be_f32, be_u16, be_u32, be_u8};
use crate::common::iff::model::{BASE_IFF_DATA_RECORD_LENGTH_OCTETS, ChangeOptionsRecord, DamageStatus, DapSource, DapValue, EnabledStatus, EnhancedMode1Code, FundamentalOperationalData, Iff, IffDataRecord, IffDataSpecification, IffFundamentalParameterData, IffLayer2, IffLayer3, IffLayer4, IffLayer5, IffPresence, InformationLayers, LatLonAltSource, LayerHeader, LayersPresenceApplicability, MalfunctionStatus, Mode5InterrogatorBasicData, Mode5InterrogatorStatus, Mode5MessageFormats, Mode5TransponderBasicData, Mode5TransponderStatus, Mode5TransponderSupplementalData, ModeSAltitude, ModeSInterrogatorBasicData, ModeSInterrogatorStatus, ModeSLevelsPresent, ModeSTransponderBasicData, ModeSTransponderStatus, OnOffStatus, OperationalStatus, ParameterCapable, SquitterStatus, SystemId, SystemSpecificData, SystemStatus};
use crate::common::parser::{beam_data, entity_id, event_id, simulation_address, vec3_f32};
use crate::constants::{BIT_0_IN_BYTE, BIT_1_IN_BYTE, BIT_2_IN_BYTE, BIT_3_IN_BYTE, BIT_4_IN_BYTE, BIT_5_IN_BYTE, BIT_6_IN_BYTE, BIT_7_IN_BYTE};
use crate::{AntennaSelection, DataCategory, IffApplicableModes, IffSystemMode, IffSystemName, IffSystemType, Level2SquitterStatus, Mode5IffMission, Mode5LevelSelection, Mode5LocationErrors, Mode5MessageFormatsStatus, Mode5PlatformType, Mode5Reply, Mode5SAltitudeResolution, NavigationSource, PduBody, VariableRecordType};

pub fn iff_body(input: &[u8]) -> IResult<&[u8], PduBody> {
    let (input, entity_id) = entity_id(input)?;
    let (input, event_id) = event_id(input)?;
    let (input, antenna_location) = vec3_f32(input)?;
    let (input, system_id) = system_id(input)?;
    let (input, system_designator) = be_u8(input)?;
    let (input, system_specific_data) = be_u8(input)?;
    let (input, fundamental_data) = fundamental_operational_data(input)?;

    let builder = Iff::builder()
        .with_emitting_entity_id(entity_id)
        .with_event_id(event_id)
        .with_relative_antenna_location(antenna_location)
        .with_system_id(system_id)
        .with_system_designator(system_designator)
        .with_system_specific_data(system_specific_data)
        .with_fundamental_operational_data(fundamental_data);

    let (input, builder) =
        if fundamental_data.information_layers.layer_2 == LayersPresenceApplicability::PresentApplicable {
            let (input, layer_2) = iff_layer_2(input)?;
            builder.with_layer_2(layer_2);
            (input, builder)
        } else { (input, builder) };
    let (input, builder) =
        if fundamental_data.information_layers.layer_3 == LayersPresenceApplicability::PresentApplicable {
            let (input, layer_3) = iff_layer_3(input)?;
            builder.with_layer_3(layer_3);
            (input, builder)
        } else { (input, builder) };
    let (input, builder) =
        if fundamental_data.information_layers.layer_4 == LayersPresenceApplicability::PresentApplicable {
            let (input, layer_4) = iff_layer_4(input)?;
            builder.with_layer_4(layer_4);
            (input, builder)
        } else { (input, builder) };
    let (input, builder) =
        if fundamental_data.information_layers.layer_5 == LayersPresenceApplicability::PresentApplicable {
            let (input, layer_5) = iff_layer_5(input)?;
            builder.with_layer_5(layer_5);
            (input, builder)
        } else { (input, builder) };

    Ok((input, builder
        .build()
        .into_pdu_body()
    ))
}

fn iff_layer_2(input: &[u8]) -> IResult<&[u8], IffLayer2> {
    let (input, layer_header) = layer_header(input)?;
    let (input, beam_data) = beam_data(input)?;
    let (input, operational_parameter_1) = be_u8(input)?;
    let (input, operational_parameter_2) = be_u8(input)?;
    let (input, num_params) = be_u16(input)?;
    let (input, fundamental_parameters) =
        count(iff_fundamental_parameter_data, num_params.into())(input)?;

    Ok((input, IffLayer2::builder()
        .with_header(layer_header)
        .with_beam_data(beam_data)
        .with_operational_parameter_1(operational_parameter_1)
        .with_operational_parameter_2(operational_parameter_2)
        .with_iff_fundamental_parameters(fundamental_parameters)
        .build()
    ))
}

fn iff_layer_3(input: &[u8]) -> IResult<&[u8], IffLayer3> {
    let (input, layer_header) = layer_header(input)?;
    let (input, reporting_simulation) = simulation_address(input)?;
    let (input, basic_data) = mode_5_basic_data(input)?;
    let (input, _padding) = be_u16(input)?;
    let (input, data_specification) = iff_data_specification(input)?;

    Ok((input, IffLayer3::builder()
        .with_header(layer_header)
        .with_reporting_simulation(reporting_simulation)
        .with_mode_5_basic_data(basic_data)
        .with_iff_data_specification(data_specification)
        .build()
    ))
}

fn iff_layer_4(input: &[u8]) -> IResult<&[u8], IffLayer4> {
    let (input, layer_header) = layer_header(input)?;
    let (input, reporting_simulation) = simulation_address(input)?;
    let (input, basic_data) = mode_s_basic_data(input)?;
    let (input, _padding) = be_u16(input)?;
    let (input, data_specification) = iff_data_specification(input)?;

    Ok((input, IffLayer4::builder()
        .with_header(layer_header)
        .with_reporting_simulation(reporting_simulation)
        .with_mode_s_basic_data(basic_data)
        .with_iff_data_specification(data_specification)
        .build()
    ))
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

    Ok((input, IffLayer5::builder()
        .with_header(layer_header)
        .with_reporting_simulation(reporting_simulation)
        .with_applicable_layers(applicable_layers)
        .with_data_category(data_category)
        .with_iff_data_specification(data_specification)
        .build()
    ))
}

fn change_options_record(input: &[u8]) -> IResult<&[u8], ChangeOptionsRecord> {
    let (input, record) = be_u8(input)?;

    let builder = ChangeOptionsRecord::builder();
    let builder = if ((record & BIT_0_IN_BYTE) >> 7) != 0 {
        builder.set_change_indicator()
    } else { builder };

    let builder = if ((record & BIT_1_IN_BYTE) >> 6) != 0 {
        builder.set_system_specific_field_1()
    } else { builder };

    let builder = if ((record & BIT_2_IN_BYTE) >> 5) != 0 {
        builder.set_system_specific_field_2()
    } else { builder };

    let builder = if ((record & BIT_3_IN_BYTE) >> 4) != 0 {
        builder.set_heartbeat_indicator()
    } else { builder };

    let builder = if ((record & BIT_4_IN_BYTE) >> 3) != 0 {
        builder.set_transponder_interrogator_indicator()
    } else { builder };

    let builder = if ((record & BIT_5_IN_BYTE) >> 2) != 0 {
        builder.set_simulation_mode()
    } else { builder };

    let builder = if ((record & BIT_6_IN_BYTE) >> 1) != 0 {
        builder.set_interactive_capable()
    } else { builder };

    let builder = if (record & BIT_7_IN_BYTE) != 0 {
        builder.set_test_mode()
    } else { builder };

    Ok((input, builder.build()))
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

    Ok((input, FundamentalOperationalData::builder()
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
        .build()
    ))
}

fn iff_data_record(input: &[u8]) -> IResult<&[u8], IffDataRecord> {
    let (input, record_type) = be_u32(input)?;
    let record_type = VariableRecordType::from(record_type);
    let (input, record_length) = be_u16(input)?;
    let (input, field) = take(record_length - BASE_IFF_DATA_RECORD_LENGTH_OCTETS)(input)?;

    Ok((input, IffDataRecord::builder()
        .with_record_type(record_type)
        .with_record_specific_field(field.to_vec())
        .build()
    ))
}

fn iff_data_specification(input: &[u8]) -> IResult<&[u8], IffDataSpecification> {
    let (input, num_records) = be_u16(input)?;
    let (input, records) =
        count(iff_data_record, num_records.into())(input)?;

    Ok((input, IffDataSpecification::builder()
        .with_iff_data_records(records)
        .build()
    ))
}

fn information_layers(input: &[u8]) -> IResult<&[u8], InformationLayers> {
    let (input, record) = be_u8(input)?;

    let builder = InformationLayers::builder()
        .with_layer_1(LayersPresenceApplicability::from(
            record & BIT_1_IN_BYTE >> 6))
        .with_layer_2(LayersPresenceApplicability::from(
            record & BIT_2_IN_BYTE >> 5))
        .with_layer_3(LayersPresenceApplicability::from(
            record & BIT_3_IN_BYTE >> 4))
        .with_layer_4(LayersPresenceApplicability::from(
            record & BIT_4_IN_BYTE >> 3))
        .with_layer_5(LayersPresenceApplicability::from(
            record & BIT_5_IN_BYTE >> 2))
        .with_layer_6(LayersPresenceApplicability::from(
            record & BIT_6_IN_BYTE >> 1))
        .with_layer_7(LayersPresenceApplicability::from(
            record & BIT_7_IN_BYTE));

    Ok((input, builder.build()))
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

    Ok((input, IffFundamentalParameterData::builder()
        .with_erp(erp)
        .with_frequency(frequency)
        .with_pgrf(pgrf)
        .with_pulse_width(pulse_width)
        .with_burst_length(burst_length)
        .with_applicable_modes(applicable_modes)
        .with_system_specific_data(system_specific_data)
        .build()
    ))
}

fn layer_header(input: &[u8]) -> IResult<&[u8], LayerHeader> {
    let (input, layer_number) = be_u8(input)?;
    let (input, layer_specific_information) = be_u8(input)?;
    let (input, length) = be_u16(input)?;

    Ok((input, LayerHeader::builder()
        .with_layer_number(layer_number)
        .with_layer_specific_information(layer_specific_information)
        .with_length(length)
        .build()
    ))
}

fn system_specific_data(input: &[u8]) -> IResult<&[u8], SystemSpecificData> {
    let (input, part_1) = be_u8(input)?;
    let (input, part_2) = be_u8(input)?;
    let (input, part_3) = be_u8(input)?;

    Ok((input, SystemSpecificData::builder()
        .with_part_1(part_1)
        .with_part_2(part_2)
        .with_part_3(part_3)
        .build()
    ))
}

fn system_id(input: &[u8]) -> IResult<&[u8], SystemId> {
    let (input, system_type) = be_u16(input)?;
    let system_type = IffSystemType::from(system_type);
    let (input, system_name) = be_u16(input)?;
    let system_name = IffSystemName::from(system_name);
    let (input, system_mode) = be_u8(input)?;
    let system_mode = IffSystemMode::from(system_mode);
    let (input, change_options_record) = be_u8(input)?;
    let change_options = ChangeOptionsRecord::from(change_options_record);

    Ok((input, SystemId::builder()
        .with_system_type(system_type)
        .with_system_name(system_name)
        .with_system_mode(system_mode)
        .with_change_options(change_options)
        .build()
    ))
}

fn dap_source(input: &[u8]) -> IResult<&[u8], DapSource> {
    let (input, record) = be_u8(input)?;

    let indicated_air_speed = DapValue::from((record & BIT_0_IN_BYTE) >> 7);
    let mach_number = DapValue::from((record & BIT_1_IN_BYTE) >> 6);
    let ground_speed = DapValue::from((record & BIT_2_IN_BYTE) >> 5);
    let magnetic_heading = DapValue::from((record & BIT_3_IN_BYTE) >> 4);
    let track_angle_rate = DapValue::from((record & BIT_4_IN_BYTE) >> 3);
    let true_track_angle = DapValue::from((record & BIT_5_IN_BYTE) >> 2);
    let true_airspeed = DapValue::from((record & BIT_6_IN_BYTE) >> 1);
    let vertical_rate = DapValue::from(record & BIT_7_IN_BYTE);

    Ok((input, DapSource::builder()
        .with_indicated_air_speed(indicated_air_speed)
        .with_mach_number(mach_number)
        .with_ground_speed(ground_speed)
        .with_magnetic_heading(magnetic_heading)
        .with_track_angle_rate(track_angle_rate)
        .with_true_track_angle(true_track_angle)
        .with_true_airspeed(true_airspeed)
        .with_vertical_rate(vertical_rate)
        .build()
    ))
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
    const BITS_0_2: u16 = 0xE000;
    const BITS_3_5: u16 = 0x1C00;
    const BITS_6_8: u16 = 0x0380;
    const BITS_9_11: u16 = 0x0070;
    const BITS_12: u16 = 0x0008;
    const BITS_13: u16 = 0x0004;
    const BITS_14: u16 = 0x0002;
    const BITS_15: u16 = 0x0001;

    let (input, record) = be_u16(input)?;

    let code_element_1_d = (record & BITS_0_2) >> 13;
    let code_element_2_c = (record & BITS_3_5) >> 10;
    let code_element_3_b = (record & BITS_6_8) >> 7;
    let code_element_4_a = (record & BITS_9_11) >> 4;
    let on_off_status =
        OnOffStatus::from(((record & BITS_12) >> 2) as u8);
    let damage_status =
        DamageStatus::from(((record & BITS_12) >> 1) as u8);
    let malfunction_status =
        MalfunctionStatus::from((record & BITS_12) as u8);

    Ok((input, EnhancedMode1Code::builder()
        .with_code_element_1_d(code_element_1_d)
        .with_code_element_2_c(code_element_2_c)
        .with_code_element_3_b(code_element_3_b)
        .with_code_element_4_a(code_element_4_a)
        .with_on_off_status(on_off_status)
        .with_damage_status(damage_status)
        .with_malfunction_status(malfunction_status)
        .build()
    ))
}

fn system_status(input: &[u8]) -> IResult<&[u8], SystemStatus> {
    let (input, record) = be_u8(input)?;

    let system_on_off_status = OnOffStatus::from((record & BIT_0_IN_BYTE) >> 7);
    let parameter_1_capable = ParameterCapable::from((record & BIT_1_IN_BYTE) >> 6);
    let parameter_2_capable = ParameterCapable::from((record & BIT_2_IN_BYTE) >> 5);
    let parameter_3_capable = ParameterCapable::from((record & BIT_3_IN_BYTE) >> 4);
    let parameter_4_capable = ParameterCapable::from((record & BIT_4_IN_BYTE) >> 3);
    let parameter_5_capable = ParameterCapable::from((record & BIT_5_IN_BYTE) >> 2);
    let parameter_6_capable = ParameterCapable::from((record & BIT_6_IN_BYTE) >> 1);
    let operational_status = OperationalStatus::from(record & BIT_7_IN_BYTE);

    Ok((input, SystemStatus::builder()
        .with_system_on_off_status(system_on_off_status)
        .with_parameter_1_capable(parameter_1_capable)
        .with_parameter_2_capable(parameter_2_capable)
        .with_parameter_3_capable(parameter_3_capable)
        .with_parameter_4_capable(parameter_4_capable)
        .with_parameter_5_capable(parameter_5_capable)
        .with_parameter_6_capable(parameter_6_capable)
        .with_operational_status(operational_status)
        .build()
    ))
}

fn mode_5_interrogator_basic_data(input: &[u8]) -> IResult<&[u8], Mode5InterrogatorBasicData> {
    let (input, status) = mode_5_interrogator_status(input)?;
    let (input, _padding) = be_u8(input)?;
    let (input, _padding) = be_u16(input)?;
    let (input, message_formats) = mode_5_message_formats(input)?;
    let (input, entity_id) = entity_id(input)?;
    let (input, _padding) = be_u16(input)?;

    Ok((input, Mode5InterrogatorBasicData::builder()
        .with_status(status)
        .with_mode_5_message_formats_present(message_formats)
        .with_interrogated_entity_id(entity_id)
        .build()
    ))
}

fn mode_5_interrogator_status(input: &[u8]) -> IResult<&[u8], Mode5InterrogatorStatus> {
    let (input, record) = be_u8(input)?;

    const BITS_0_2: u8 = 0xE0;
    let iff_mission = Mode5IffMission::from((record & BITS_0_2) >> 5);
    let mode_5_message_formats_status = Mode5MessageFormatsStatus::from((record & BIT_3_IN_BYTE) >> 4);
    let on_off_status =
        OnOffStatus::from((record & BIT_5_IN_BYTE) >> 2);
    let damage_status =
        DamageStatus::from((record & BIT_6_IN_BYTE) >> 1);
    let malfunction_status =
        MalfunctionStatus::from(record & BIT_7_IN_BYTE);

    Ok((input, Mode5InterrogatorStatus::builder()
        .with_iff_mission(iff_mission)
        .with_mode_5_message_formats_status(mode_5_message_formats_status)
        .with_on_off_status(on_off_status)
        .with_damage_status(damage_status)
        .with_malfunction_status(malfunction_status)
        .build()
    ))
}

fn mode_5_message_formats(input: &[u8]) -> IResult<&[u8], Mode5MessageFormats> {
    let (input, record) = be_u32(input)?;

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

    Ok((input, Mode5MessageFormats::builder()
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
    ))
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

    Ok((input, Mode5TransponderBasicData::builder()
        .with_status(status)
        .with_pin(pin)
        .with_mode_5_message_formats_present(message_formats_present)
        .with_enhanced_mode_1(enhanced_mode_1)
        .with_national_origin(national_origin)
        .with_supplemental_data(supplemental_data)
        .with_navigation_source(navigation_source)
        .with_figure_of_merit(figure_of_merit)
        .build()
    ))
}

fn mode_5_transponder_supplemental_data(input: &[u8]) -> IResult<&[u8], Mode5TransponderSupplementalData> {
    let (input, record) = be_u8(input)?;

    const BITS_2_4: u8 = 0x38;
    let squitter_status = SquitterStatus::from((record & BIT_0_IN_BYTE) >> 7);
    let level_2_squitter_status = Level2SquitterStatus::from((record & BIT_1_IN_BYTE) >> 6);
    let iff_mission = Mode5IffMission::from((record & BITS_2_4) >> 3);

    Ok((input, Mode5TransponderSupplementalData::builder()
        .with_squitter_on_off_status(squitter_status)
        .with_level_2_squitter_status(level_2_squitter_status)
        .with_iff_mission(iff_mission)
        .build()
    ))
}

fn mode_5_transponder_status(input: &[u8]) -> IResult<&[u8], Mode5TransponderStatus> {
    let (input, record) = be_u16(input)?;
    const BITS_0_3: u16 = 0xF000;
    const BIT_4: u16 = 0x0800;
    const BITS_5_6: u16 = 0x0600;
    const BIT_7: u16 = 0x0100;
    const BIT_8: u16 = 0x0080;
    const BIT_9: u16 = 0x0040;
    const BIT_10: u16 = 0x0020;
    const BIT_11: u16 = 0x0010;
    const BIT_12: u16 = 0x0008;
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

    Ok((input, Mode5TransponderStatus::builder()
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
    ))
}

fn mode_s_altitude(input: &[u8]) -> IResult<&[u8], ModeSAltitude> {
    let (input, record) = be_u16(input)?;

    const BITS_0_10: u16 = 0xFFE0;
    const BIT_11: u16 = 0x0010;
    let altitude = (record & BITS_0_10) >> 5;
    let resolution =
        Mode5SAltitudeResolution::from(((record & BIT_11) as u8) >> 4);

    Ok((input, ModeSAltitude::builder()
        .with_altitude(altitude)
        .with_resolution(resolution)
        .build()
    ))
}

fn mode_s_interrogator_basic_data(input: &[u8]) -> IResult<&[u8], ModeSInterrogatorBasicData> {
    todo!()
}

fn mode_s_interrogator_status(input: &[u8]) -> IResult<&[u8], ModeSInterrogatorStatus> {
    todo!()
}

fn mode_s_levels_present(input: &[u8]) -> IResult<&[u8], ModeSLevelsPresent> {
    todo!()
}

fn mode_s_transponder_basic_data(input: &[u8]) -> IResult<&[u8], ModeSTransponderBasicData> {
    todo!()
}

fn mode_s_transponder_status(input: &[u8]) -> IResult<&[u8], ModeSTransponderStatus> {
    todo!()
}

impl From<u8> for OnOffStatus {
    fn from(value: u8) -> Self {
        match value {
            0 => OnOffStatus::Off,
            _ => OnOffStatus::On
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

impl From<u8> for ChangeOptionsRecord {
    fn from(value: u8) -> Self {
        ChangeOptionsRecord {
            change_indicator: ((value & BIT_0_IN_BYTE) >> 7) == 1u8,
            system_specific_field_1: ((value & BIT_1_IN_BYTE) >> 6) == 1u8,
            system_specific_field_2: ((value & BIT_2_IN_BYTE) >> 5) == 1u8,
            heartbeat_indicator: ((value & BIT_3_IN_BYTE) >> 4) == 1u8,
            transponder_interrogator_indicator: ((value & BIT_4_IN_BYTE) >> 3) == 1u8,
            simulation_mode: ((value & BIT_5_IN_BYTE) >> 2) == 1u8,
            interactive_capable: ((value & BIT_6_IN_BYTE) >> 1) == 1u8,
            test_mode: (value & BIT_7_IN_BYTE) == 1u8,
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