use nom::IResult;
use nom::multi::count;
use nom::number::complete::be_u8;
use crate::{EntityKind, EntityType, PduBody};
use crate::common::entity_state::parser::{entity_appearance, variable_parameter};
use crate::common::entity_state_update::model::EntityStateUpdate;
use crate::common::parser::{entity_id, location, orientation, vec3_f32};

pub fn entity_state_update_body(input: &[u8]) -> IResult<&[u8], PduBody> {
    let (input, entity_id_val) = entity_id(input)?;
    let (input, _padding) = be_u8(input)?;
    let (input, variable_parameters_no) = be_u8(input)?;
    let (input, entity_linear_velocity) = vec3_f32(input)?;
    let (input, entity_location) = location(input)?;
    let (input, entity_orientation) = orientation(input)?;
    let (input, entity_appearance) = entity_appearance(
        EntityType::default().with_kind(EntityKind::Other))(input)?;
    let (input, variable_parameters) = if variable_parameters_no > 0 {
        count(variable_parameter, variable_parameters_no as usize)(input)?
    } else { (input, vec![]) };

    let body = EntityStateUpdate::new(entity_id_val)
        .with_velocity(entity_linear_velocity)
        .with_location(entity_location)
        .with_orientation(entity_orientation)
        .with_appearance(entity_appearance)
        .with_variable_parameters(variable_parameters);

    Ok((input, body.into_pdu_body()))
}