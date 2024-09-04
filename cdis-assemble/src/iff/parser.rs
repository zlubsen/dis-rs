use nom::complete::take;
use nom::IResult;
use dis_rs::enumerations::{IffSystemMode, IffSystemName, IffSystemType};
use dis_rs::iff::model::{ChangeOptionsRecord, InformationLayers, LayersPresenceApplicability, SystemId, SystemStatus};
use crate::{BodyProperties, CdisBody};
use crate::constants::{EIGHT_BITS, FIVE_BITS, FOUR_BITS, ONE_BIT, SIXTEEN_BITS, THREE_BITS, TWELVE_BITS};
use crate::iff::model::{CdisFundamentalOperationalData, Iff, IffLayer1FieldsPresent, IffLayer2, IffLayer3, IffLayer4, IffLayer5};
use crate::parsing::{parse_field_when_present, BitInput};
use crate::records::model::UnitsMeters;
use crate::records::parser::{entity_coordinate_vector, entity_identification};

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
        let (input, layer_3) = iff_layer_3(input)?;
        (input, Some(layer_3))
    } else { (input, None) };
    let (input, layer_4) = if fundamental_operational_data.information_layers.layer_4 == LayersPresenceApplicability::PresentApplicable {
        let (input, layer_4) = iff_layer_4(input)?;
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
    todo!()
}

fn iff_layer_3(input: BitInput) -> IResult<BitInput, IffLayer3> {
    todo!()
}

fn iff_layer_4(input: BitInput) -> IResult<BitInput, IffLayer4> {
    todo!()
}

fn iff_layer_5(input: BitInput) -> IResult<BitInput, IffLayer5> {
    todo!()
}