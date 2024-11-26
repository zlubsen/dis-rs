use nom::complete::take;
use nom::IResult;
use nom::multi::count;
use dis_rs::DisError;
use dis_rs::enumerations::{AircraftIdentificationType, AircraftPresentDomain, CapabilityReport, DataCategory, IffApplicableModes, IffSystemMode, IffSystemName, IffSystemType, NavigationSource, VariableRecordType};
use dis_rs::iff::model::{ChangeOptionsRecord, DapSource, EnhancedMode1Code, IffDataRecord, InformationLayers, LayersPresenceApplicability, Mode5InterrogatorStatus, Mode5MessageFormats, Mode5TransponderBasicData, Mode5TransponderStatus, Mode5TransponderSupplementalData, ModeSAltitude, ModeSInterrogatorBasicData, ModeSInterrogatorStatus, ModeSLevelsPresent, ModeSTransponderBasicData, ModeSTransponderStatus, SystemId, SystemSpecificData, SystemStatus};
use crate::{BodyProperties, CdisBody};
use crate::constants::{EIGHT_BITS, FIVE_BITS, FOUR_BITS, ONE_BIT, SIXTEEN_BITS, SIX_BITS, TEN_BITS, THIRTY_TWO_BITS, THREE_BITS, TWELVE_BITS};
use crate::records::model::FrequencyFloat;
use crate::iff::model::{CdisFundamentalOperationalData, Iff, IffFundamentalParameterData, IffLayer1FieldsPresent, Mode5BasicData, IffLayer2, IffLayer3, IffLayer4, IffLayer5, Mode5InterrogatorBasicData, ModeSBasicData};
use crate::parsing::{parse_field_when_present, BitInput};
use crate::records::model::UnitsMeters;
use crate::records::parser::{beam_data, entity_coordinate_vector, entity_identification, layer_header};
use crate::types::model::CdisFloat;
use crate::types::parser::uvint16;

pub(crate) fn iff_body(input: BitInput) -> IResult<BitInput, CdisBody> {
    let (input, fields_present) : (BitInput, u16) = take(TWELVE_BITS)(input)?;
    let (input, relative_antenna_location_units) : (BitInput, u8) = take(ONE_BIT)(input)?;
    let relative_antenna_location_units = UnitsMeters::from(relative_antenna_location_units);
    let (input, full_update_flag) : (BitInput, u8) = take(ONE_BIT)(input)?;
    let full_update_flag = full_update_flag != 0;

    let (input, emitting_entity_id) = entity_identification(input)?;
    let (input, event_id) =
        parse_field_when_present(fields_present, IffLayer1FieldsPresent::EVENT_ID_BIT, entity_identification)(input)?;
    let (input, relative_antenna_location) =
        parse_field_when_present(fields_present, IffLayer1FieldsPresent::RELATIVE_ANTENNA_LOCATION_BIT, entity_coordinate_vector)(input)?;
    let (input, system_id) =
        parse_field_when_present(fields_present, IffLayer1FieldsPresent::SYSTEM_ID_DETAILS_BIT, system_id)(input)?;
    let (input, system_designator) : (BitInput, u8) = take(EIGHT_BITS)(input)?;
    let (input, system_specific_data) : (BitInput, Option<u8>) =
        parse_field_when_present(fields_present, IffLayer1FieldsPresent::SYSTEM_SPECIFIC_DATA_BIT, take(EIGHT_BITS))(input)?;

    let (input, fundamental_operational_data) = fundamental_operational_data(fields_present)(input)?;

    let (input, layer_2) = if fundamental_operational_data.information_layers.layer_2 == LayersPresenceApplicability::PresentApplicable {
        let (input, layer_2) = iff_layer_2(input)?;
        (input, Some(layer_2))
    } else { (input, None) };
    let (input, layer_3) = if fundamental_operational_data.information_layers.layer_3 == LayersPresenceApplicability::PresentApplicable {
        let system_type = system_id.clone().unwrap_or_default().system_type;
        let (input, layer_3) = iff_layer_3(&system_type)(input)?;
        (input, Some(layer_3))
    } else { (input, None) };
    let (input, layer_4) = if fundamental_operational_data.information_layers.layer_4 == LayersPresenceApplicability::PresentApplicable {
        let system_type = system_id.clone().unwrap_or_default().system_type;
        let (input, layer_4) = iff_layer_4(&system_type)(input)?;
        (input, Some(layer_4))
    } else { (input, None) };
    let (input, layer_5) = if fundamental_operational_data.information_layers.layer_5 == LayersPresenceApplicability::PresentApplicable {
        let (input, layer_5) = iff_layer_5(input)?;
        (input, Some(layer_5))
    } else { (input, None) };

    Ok((input, Iff {
        relative_antenna_location_units,
        full_update_flag,
        emitting_entity_id,
        event_id,
        relative_antenna_location,
        system_id,
        system_designator,
        system_specific_data,
        fundamental_operational_data,
        layer_2,
        layer_3,
        layer_4,
        layer_5,
    }.into_cdis_body()))
}

fn system_id(input: BitInput) -> IResult<BitInput, SystemId> {
    let (input, system_type) : (BitInput, u16) = take(FOUR_BITS)(input)?;
    let system_type = IffSystemType::from(system_type);
    let (input, system_name) : (BitInput, u16) = take(FIVE_BITS)(input)?;
    let system_name = IffSystemName::from(system_name);
    let (input, system_mode) : (BitInput, u8) = take(THREE_BITS)(input)?;
    let system_mode = IffSystemMode::from(system_mode);
    let (input, change_options) : (BitInput, u8) = take(EIGHT_BITS)(input)?;
    let change_options = ChangeOptionsRecord::from(change_options);

    Ok((input, SystemId::builder()
        .with_system_type(system_type)
        .with_system_name(system_name)
        .with_system_mode(system_mode)
        .with_change_options(change_options)
        .build()))
}

fn fundamental_operational_data(fields_present: u16) -> impl Fn(BitInput) -> IResult<BitInput, CdisFundamentalOperationalData> {
    move |input: BitInput| {
        let (input, system_status) : (BitInput, u8) = take(EIGHT_BITS)(input)?;
        let system_status= SystemStatus::from(system_status);

        let (input, data_field_1) : (BitInput, Option<u8>) =
            parse_field_when_present(fields_present, IffLayer1FieldsPresent::DATA_FIELD_1_BIT, take(EIGHT_BITS))(input)?;

        let (input, information_layers) : (BitInput, u8) = take(EIGHT_BITS)(input)?;
        let information_layers = InformationLayers::from(information_layers);

        let (input, data_field_2) : (BitInput, Option<u8>) =
            parse_field_when_present(fields_present, IffLayer1FieldsPresent::DATA_FIELD_2_BIT, take(EIGHT_BITS))(input)?;

        let (input, parameter_1) : (BitInput, Option<u16>) =
            parse_field_when_present(fields_present, IffLayer1FieldsPresent::PARAMETER_1_BIT, take(SIXTEEN_BITS))(input)?;
        let (input, parameter_2) : (BitInput, Option<u16>) =
            parse_field_when_present(fields_present, IffLayer1FieldsPresent::PARAMETER_2_BIT, take(SIXTEEN_BITS))(input)?;
        let (input, parameter_3) : (BitInput, Option<u16>) =
            parse_field_when_present(fields_present, IffLayer1FieldsPresent::PARAMETER_3_BIT, take(SIXTEEN_BITS))(input)?;
        let (input, parameter_4) : (BitInput, Option<u16>) =
            parse_field_when_present(fields_present, IffLayer1FieldsPresent::PARAMETER_4_BIT, take(SIXTEEN_BITS))(input)?;
        let (input, parameter_5) : (BitInput, Option<u16>) =
            parse_field_when_present(fields_present, IffLayer1FieldsPresent::PARAMETER_5_BIT, take(SIXTEEN_BITS))(input)?;
        let (input, parameter_6) : (BitInput, Option<u16>) =
            parse_field_when_present(fields_present, IffLayer1FieldsPresent::PARAMETER_6_BIT, take(SIXTEEN_BITS))(input)?;

        Ok((input, CdisFundamentalOperationalData {
            system_status,
            data_field_1,
            information_layers,
            data_field_2,
            parameter_1,
            parameter_2,
            parameter_3,
            parameter_4,
            parameter_5,
            parameter_6,
        }))
    }
}

fn iff_layer_2(input: BitInput) -> IResult<BitInput, IffLayer2> {
    // Units field is unused, MUST be '1' but no check performed.
    // Presence of Fundamental Param Data is assumed present as per the spec.
    // Field required to be consistent with all Layers; Table 68
    let (input, _units) : (BitInput, u8) = take(ONE_BIT)(input)?;

    let (input, layer_header) = layer_header(input)?;
    let (input, beam_data) = beam_data(input)?;
    let (input, operational_parameter_1) : (BitInput, u8) = take(EIGHT_BITS)(input)?;
    let (input, operational_parameter_2) : (BitInput, u8) = take(EIGHT_BITS)(input)?;
    let (input, nr_of_data_records) : (BitInput, usize) = take(EIGHT_BITS)(input)?;

    let (input, iff_fundamental_parameters) = count(fundamental_parameter_data_record, nr_of_data_records)(input)?;

    Ok((input, IffLayer2 {
        layer_header,
        beam_data,
        operational_parameter_1,
        operational_parameter_2,
        iff_fundamental_parameters,
    }))
}

fn fundamental_parameter_data_record(input: BitInput) -> IResult<BitInput, IffFundamentalParameterData> {
    let (input, erp) : (BitInput, u8) = take(EIGHT_BITS)(input)?;
    let (input, frequency) = FrequencyFloat::parse(input)?;
    let (input, pgrf) : (BitInput, u16) = take(TEN_BITS)(input)?;
    let (input, pulse_width) : (BitInput, u16) = take(TEN_BITS)(input)?;
    let (input, burst_length) : (BitInput, u16) = take(TEN_BITS)(input)?;

    let (input, applicable_modes) : (BitInput, u8) = take(THREE_BITS)(input)?;
    let applicable_modes = IffApplicableModes::from(applicable_modes);
    let (input, specific_data_1) : (BitInput, u8) = take(EIGHT_BITS)(input)?;
    let (input, specific_data_2) : (BitInput, u8) = take(EIGHT_BITS)(input)?;
    let (input, specific_data_3) : (BitInput, u8) = take(EIGHT_BITS)(input)?;
    let system_specific_data = SystemSpecificData::builder()
        .with_part_1(specific_data_1)
        .with_part_2(specific_data_2)
        .with_part_3(specific_data_3)
        .build();

    Ok((input, IffFundamentalParameterData {
        erp,
        frequency,
        pgrf,
        pulse_width,
        burst_length,
        applicable_modes,
        system_specific_data,
    }))
}

fn iff_layer_3(iff_system_type: &IffSystemType) -> impl Fn(BitInput) -> IResult<BitInput, IffLayer3> + '_ {
    move |input: BitInput| {
        let (input, data_records_present): (BitInput, u8) = take(ONE_BIT)(input)?;
        let data_records_present = data_records_present != 0;
        let (input, layer_header) = layer_header(input)?;
        let (input, reporting_simulation_site) = uvint16(input)?;
        let (input, reporting_simulation_application) = uvint16(input)?;
        let (input, mode_5_basic_data) = mode_5_basic_data(iff_system_type)(input)?;

        let (input, iff_data_records) = if data_records_present {
            let (input, nr_of_data_records) : (BitInput, usize) = take(FIVE_BITS)(input)?;
            count(iff_data_record, nr_of_data_records)(input)?
        } else { (input, vec![]) };

        Ok((input, IffLayer3 {
            layer_header,
            reporting_simulation_site,
            reporting_simulation_application,
            mode_5_basic_data: mode_5_basic_data.unwrap_or(Mode5BasicData::Transponder(Mode5TransponderBasicData::default())),
            iff_data_records,
        }))
    }
}

fn mode_5_basic_data(iff_system_type: &IffSystemType) -> impl Fn(BitInput) -> IResult<BitInput, Result<Mode5BasicData, DisError>> + '_ {
    move |input: BitInput| {
        let (input, mode_5_basic_data) = match iff_system_type {
            IffSystemType::MarkXXIIATCRBSTransponder |
            IffSystemType::SovietTransponder |
            IffSystemType::RRBTransponder |
            IffSystemType::MarkXIIATransponder |
            IffSystemType::Mode5Transponder |
            IffSystemType::ModeSTransponder => {
                let (input, basic_data) = mode_5_transponder_basic_data(input)?;
                (input, Ok(Mode5BasicData::Transponder(basic_data)))
            }
            IffSystemType::MarkXXIIATCRBSInterrogator |
            IffSystemType::SovietInterrogator |
            IffSystemType::MarkXIIAInterrogator |
            IffSystemType::Mode5Interrogator |
            IffSystemType::ModeSInterrogator => {
                let (input, basic_data) = mode_5_interrogator_basic_data(input)?;
                (input, Ok(Mode5BasicData::Interrogator(basic_data)))
            }
            IffSystemType::MarkXIIACombinedInterrogatorTransponder_CIT_ |
            IffSystemType::MarkXIICombinedInterrogatorTransponder_CIT_ |
            IffSystemType::TCASACASTransceiver => { (input, Err(DisError::IffUndeterminedSystemType)) }
            IffSystemType::NotUsed_InvalidValue_ => { (input, Err(DisError::IffIncorrectSystemType)) }
            IffSystemType::Unspecified(_) => { (input, Err(DisError::IffIncorrectSystemType)) }
        };

        Ok((input, mode_5_basic_data))
    }
}

fn mode_5_transponder_basic_data(input: BitInput) -> IResult<BitInput, Mode5TransponderBasicData> {
    let (input, mode_5_status) : (BitInput, u16) = take(SIXTEEN_BITS)(input)?;
    let mode_5_status = Mode5TransponderStatus::from(mode_5_status);
    let (input, pin) : (BitInput, u16) = take(SIXTEEN_BITS)(input)?;
    let (input, message_formats_present) : (BitInput, u32) = take(THIRTY_TWO_BITS)(input)?;
    let message_formats_present = Mode5MessageFormats::from(message_formats_present);
    let (input, enhanced_mode_1) : (BitInput, u16) = take(SIXTEEN_BITS)(input)?;
    let enhanced_mode_1 = EnhancedMode1Code::from(enhanced_mode_1);
    let (input, national_origin) : (BitInput, u16) = take(SIXTEEN_BITS)(input)?;
    let (input, supplemental) : (BitInput, u8) = take(EIGHT_BITS)(input)?;
    let supplemental = Mode5TransponderSupplementalData::from(supplemental);
    let (input, navigation_source) : (BitInput, u8) = take(THREE_BITS)(input)?;
    let navigation_source = NavigationSource::from(navigation_source);
    let (input, figure_of_merit) : (BitInput, u8) = take(FIVE_BITS)(input)?;

    Ok((input, Mode5TransponderBasicData::builder()
        .with_status(mode_5_status)
        .with_pin(pin)
        .with_mode_5_message_formats_present(message_formats_present)
        .with_enhanced_mode_1(enhanced_mode_1)
        .with_national_origin(national_origin)
        .with_supplemental_data(supplemental)
        .with_navigation_source(navigation_source)
        .with_figure_of_merit(figure_of_merit)
        .build()))
}

fn mode_5_interrogator_basic_data(input: BitInput) -> IResult<BitInput, Mode5InterrogatorBasicData> {
    let (input, interrogator_status) : (BitInput, u8) = take(EIGHT_BITS)(input)?;
    let interrogator_status = Mode5InterrogatorStatus::from(interrogator_status);
    let (input, message_formats_present) : (BitInput, u32) = take(THIRTY_TWO_BITS)(input)?;
    let message_formats_present = Mode5MessageFormats::from(message_formats_present);
    let (input, interrogated_entity_id) = entity_identification(input)?;

    Ok((input, Mode5InterrogatorBasicData {
        interrogator_status,
        message_formats_present,
        interrogated_entity_id,
    }))
}

fn iff_data_record(input: BitInput) -> IResult<BitInput, IffDataRecord> {
    const THREE_OCTETS: usize = 3;
    let (input, record_type) : (BitInput, u32) = take(SIXTEEN_BITS)(input)?;
    let record_type = VariableRecordType::from(record_type);
    let (input, record_length) : (BitInput, usize) = take(EIGHT_BITS)(input)?;
    let (input, specific_field) : (BitInput, Vec<u8>) = count(take(EIGHT_BITS), record_length.saturating_sub(THREE_OCTETS))(input)?;

    Ok((input, IffDataRecord::builder()
        .with_record_type(record_type)
        .with_record_specific_field(specific_field)
        .build()))
}

fn iff_layer_4(iff_system_type: &IffSystemType) -> impl Fn(BitInput) -> IResult<BitInput, IffLayer4> + '_ {
    move |input: BitInput| {
        let (input, data_records_present): (BitInput, u8) = take(ONE_BIT)(input)?;
        let data_records_present = data_records_present != 0;
        let (input, layer_header) = layer_header(input)?;
        let (input, reporting_simulation_site) = uvint16(input)?;
        let (input, reporting_simulation_application) = uvint16(input)?;
        let (input, mode_s_basic_data) = mode_s_basic_data(iff_system_type)(input)?;

        let (input, iff_data_records) = if data_records_present {
            let (input, nr_of_data_records) : (BitInput, usize) = take(FIVE_BITS)(input)?;
            count(iff_data_record, nr_of_data_records)(input)?
        } else { (input, vec![]) };

        Ok((input, IffLayer4 {
            layer_header,
            reporting_simulation_site,
            reporting_simulation_application,
            mode_s_basic_data: mode_s_basic_data.unwrap_or(ModeSBasicData::Transponder(ModeSTransponderBasicData::default())),
            iff_data_records,
        }))
    }
}

fn mode_s_basic_data(iff_system_type: &IffSystemType) -> impl Fn(BitInput) -> IResult<BitInput, Result<ModeSBasicData, DisError>> + '_ {
    move |input: BitInput| {
        let (input, mode_s_basic_data) = match iff_system_type {
            IffSystemType::MarkXXIIATCRBSTransponder |
            IffSystemType::SovietTransponder |
            IffSystemType::RRBTransponder |
            IffSystemType::MarkXIIATransponder |
            IffSystemType::Mode5Transponder |
            IffSystemType::ModeSTransponder => {
                let (input, basic_data) = mode_s_transponder_basic_data(input)?;
                (input, Ok(ModeSBasicData::Transponder(basic_data)))
            }
            IffSystemType::MarkXXIIATCRBSInterrogator |
            IffSystemType::SovietInterrogator |
            IffSystemType::MarkXIIAInterrogator |
            IffSystemType::Mode5Interrogator |
            IffSystemType::ModeSInterrogator => {
                let (input, basic_data) = mode_s_interrogator_basic_data(input)?;
                (input, Ok(ModeSBasicData::Interrogator(basic_data)))
            }
            IffSystemType::MarkXIIACombinedInterrogatorTransponder_CIT_ |
            IffSystemType::MarkXIICombinedInterrogatorTransponder_CIT_ |
            IffSystemType::TCASACASTransceiver => { (input, Err(DisError::IffUndeterminedSystemType)) }
            IffSystemType::NotUsed_InvalidValue_ => { (input, Err(DisError::IffIncorrectSystemType)) }
            IffSystemType::Unspecified(_) => { (input, Err(DisError::IffIncorrectSystemType)) }
        };

        Ok((input, mode_s_basic_data))
    }
}

fn mode_s_transponder_basic_data(input: BitInput) -> IResult<BitInput, ModeSTransponderBasicData> {
    let (input, mode_s_status) : (BitInput, u16) = take(SIXTEEN_BITS)(input)?;
    let mode_s_status = ModeSTransponderStatus::from(mode_s_status);

    let (input, mode_s_levels_present) : (BitInput, u8) = take(EIGHT_BITS)(input)?;
    let mode_s_levels_present = ModeSLevelsPresent::from(mode_s_levels_present);

    let (input, aircraft_present_domain) : (BitInput, u8) = take(THREE_BITS)(input)?;
    let aircraft_present_domain = AircraftPresentDomain::from(aircraft_present_domain);

    let (input, nr_of_chars_in_id) : (BitInput, usize) = take(FOUR_BITS)(input)?;

    let (input, aircraft_id) : (BitInput, Vec<u8>)= count(take(EIGHT_BITS), nr_of_chars_in_id)(input)?;
    let mut aircraft_id = String::from_utf8_lossy(&aircraft_id).into_owned();
    aircraft_id.truncate(aircraft_id.trim_end().trim_end_matches(|c : char | !c.is_alphanumeric()).len());

    let (input, aircraft_address) : (BitInput, u32) = take(THIRTY_TWO_BITS)(input)?;

    let (input, aircraft_id_type) : (BitInput, u8) = take(THREE_BITS)(input)?;
    let aircraft_id_type = AircraftIdentificationType::from(aircraft_id_type);

    let (input, dap_source) : (BitInput, u8) = take(EIGHT_BITS)(input)?;
    let dap_source = DapSource::from(dap_source);

    let (input, altitude) : (BitInput, u16) = take(SIXTEEN_BITS)(input)?;
    let altitude = ModeSAltitude::from(altitude);

    let (input, capability_report) : (BitInput, u8) = take(EIGHT_BITS)(input)?;
    let capability_report = CapabilityReport::from(capability_report);

    Ok((input, ModeSTransponderBasicData::builder()
        .with_status(mode_s_status)
        .with_levels_present(mode_s_levels_present)
        .with_aircraft_present_domain(aircraft_present_domain)
        .with_aircraft_identification(aircraft_id)
        .with_aircraft_address(aircraft_address)
        .with_aircraft_identification_type(aircraft_id_type)
        .with_dap_source(dap_source)
        .with_altitude(altitude)
        .with_capability_report(capability_report)
        .build()
    ))
}

fn mode_s_interrogator_basic_data(input: BitInput) -> IResult<BitInput, ModeSInterrogatorBasicData> {
    let (input, interrogator_status) : (BitInput, u8) = take(EIGHT_BITS)(input)?;
    let interrogator_status = ModeSInterrogatorStatus::from(interrogator_status);

    let (input, levels_present) : (BitInput, u8) = take(EIGHT_BITS)(input)?;
    let levels_present = ModeSLevelsPresent::from(levels_present);

    Ok((input, ModeSInterrogatorBasicData::builder()
        .with_mode_s_interrogator_status(interrogator_status)
        .with_mode_s_levels_present(levels_present)
        .build()))
}

fn iff_layer_5(input: BitInput) -> IResult<BitInput, IffLayer5> {
    let (input, data_records_present): (BitInput, u8) = take(ONE_BIT)(input)?;
    let data_records_present = data_records_present != 0;
    let (input, layer_header) = layer_header(input)?;
    let (input, reporting_simulation_site) = uvint16(input)?;
    let (input, reporting_simulation_application) = uvint16(input)?;
    let (input, applicable_layers) : (BitInput, u8) = take(SIX_BITS)(input)?;
    let applicable_layers = InformationLayers::from(applicable_layers);
    let (input, data_category) : (BitInput, u8) = take(THREE_BITS)(input)?;
    let data_category = DataCategory::from(data_category);

    let (input, iff_data_records) = if data_records_present {
        let (input, nr_of_data_records) : (BitInput, usize) = take(FIVE_BITS)(input)?;
        count(iff_data_record, nr_of_data_records)(input)?
    } else { (input, vec![]) };

    Ok((input, IffLayer5 {
        layer_header,
        reporting_simulation_site,
        reporting_simulation_application,
        applicable_layers,
        data_category,
        iff_data_records,
    }))
}