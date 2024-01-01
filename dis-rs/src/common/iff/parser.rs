use nom::IResult;
use nom::number::complete::{be_u16, be_u8};
use crate::common::iff::model::{ChangeOptionsRecord, FundamentalOperationalData, Iff, InformationLayers, LayersPresenceApplicability, ParameterCapable, SystemId, SystemStatus};
use crate::common::parser::{entity_id, event_id, vec3_f32};
use crate::constants::{BIT_0_IN_BYTE, BIT_1_IN_BYTE, BIT_2_IN_BYTE, BIT_3_IN_BYTE, BIT_4_IN_BYTE, BIT_5_IN_BYTE, BIT_6_IN_BYTE, BIT_7_IN_BYTE};
use crate::{IffSystemMode, IffSystemName, IffSystemType, PduBody};

pub fn iff_body(input: &[u8]) -> IResult<&[u8], PduBody> {
    let (input, entity_id) = entity_id(input)?;
    let (input, event_id) = event_id(input)?;
    let (input, antenna_location) = vec3_f32(input)?;
    let (input, system_id) = system_id(input)?;
    let (input, system_designator) = be_u8(input)?;
    let (input, system_specific_data) = be_u8(input)?;
    let (input, fundamental_data) = fundamental_operational_data(input)?;

    Ok((input, Iff::builder()
        .with_emitting_entity_id(entity_id)
        .with_event_id(event_id)
        .with_relative_antenna_location(antenna_location)
        .with_system_id(system_id)
        .with_system_designator(system_designator)
        .with_system_specific_data(system_specific_data)
        .with_fundamental_operational_data(fundamental_data)
        .build()
        .into_pdu_body()
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

    Ok((input, SystemId {
        system_type,
        system_name,
        system_mode,
        change_options,
    }))
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

    todo!("construct the struct");
    Ok((input, FundamentalOperationalData::default()))

}

fn system_status(input: &[u8]) -> IResult<&[u8], SystemStatus> {
    todo!()
}

fn information_layers(input: &[u8]) -> IResult<&[u8], InformationLayers> {
    todo!()
}

impl From<u8> for ParameterCapable {
    fn from(value: u8) -> Self {
        match value {
            0 => ParameterCapable::Capable,
            _ => ParameterCapable::NotCapable,
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