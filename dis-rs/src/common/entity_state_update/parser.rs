use crate::common::entity_state::parser::entity_appearance;
use crate::common::entity_state_update::model::EntityStateUpdate;
use crate::common::model::{EntityType, PduBody};
use crate::common::parser::{entity_id, location, orientation, variable_parameter, vec3_f32};
use crate::enumerations::EntityKind;
use nom::multi::count;
use nom::number::complete::be_u8;
use nom::{IResult, Parser};

pub(crate) fn entity_state_update_body(input: &[u8]) -> IResult<&[u8], PduBody> {
    let (input, entity_id_val) = entity_id(input)?;
    let (input, _padding) = be_u8(input)?;
    let (input, variable_parameters_no) = be_u8(input)?;
    let (input, entity_linear_velocity) = vec3_f32(input)?;
    let (input, entity_location) = location(input)?;
    let (input, entity_orientation) = orientation(input)?;
    let (input, entity_appearance) =
        entity_appearance(EntityType::default().with_kind(EntityKind::Other))(input)?;
    let (input, variable_parameters) = if variable_parameters_no > 0 {
        count(variable_parameter, variable_parameters_no as usize).parse(input)?
    } else {
        (input, vec![])
    };

    let body = EntityStateUpdate::builder()
        .with_entity_id(entity_id_val)
        .with_velocity(entity_linear_velocity)
        .with_location(entity_location)
        .with_orientation(entity_orientation)
        .with_appearance(entity_appearance)
        .with_variable_parameters(variable_parameters)
        .build();

    Ok((input, body.into_pdu_body()))
}
