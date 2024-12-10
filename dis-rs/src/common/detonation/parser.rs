use crate::common::detonation::model::Detonation;
use crate::common::model::{PduBody, PduHeader};
use crate::common::parser::variable_parameter;
use crate::common::parser::{descriptor_record_dti, entity_id, event_id, location, vec3_f32};
use crate::enumerations::{DetonationResult, DetonationTypeIndicator};
use nom::multi::count;
use nom::number::complete::{be_u16, be_u8};
use nom::IResult;

pub(crate) fn detonation_body(
    header: &PduHeader,
) -> impl Fn(&[u8]) -> IResult<&[u8], PduBody> + '_ {
    move |input: &[u8]| {
        let dti = header
            .pdu_status
            .unwrap_or_default()
            .detonation_type_indicator
            .unwrap_or(DetonationTypeIndicator::Munition);
        let (input, source_entity_id) = entity_id(input)?;
        let (input, target_entity_id) = entity_id(input)?;
        let (input, exploding_entity_id) = entity_id(input)?;
        let (input, event_it) = event_id(input)?;
        let (input, velocity) = vec3_f32(input)?;
        let (input, world_location) = location(input)?;
        let (input, descriptor) = descriptor_record_dti(dti)(input)?;
        let (input, entity_location) = vec3_f32(input)?;
        let (input, detonation_result) = be_u8(input)?;
        let (input, variable_parameters_no) = be_u8(input)?;
        let (input, _padding) = be_u16(input)?;
        let (input, articulation_parameters) = if variable_parameters_no > 0 {
            count(variable_parameter, variable_parameters_no as usize)(input)?
        } else {
            (input, vec![])
        };

        let body = Detonation::builder()
            .with_source_entity_id(source_entity_id)
            .with_target_entity_id(target_entity_id)
            .with_exploding_entity_id(exploding_entity_id)
            .with_event_id(event_it)
            .with_velocity(velocity)
            .with_world_location(world_location)
            .with_descriptor(descriptor)
            .with_entity_location(entity_location)
            .with_detonation_result(DetonationResult::from(detonation_result))
            .with_variable_parameters(articulation_parameters)
            .build();

        Ok((input, body.into_pdu_body()))
    }
}
