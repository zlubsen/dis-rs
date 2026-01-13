use crate::common::detonation::model::{Detonation, DetonationDescriptor};
use crate::common::model::{PduBody, PduHeader};
use crate::common::parser;
use crate::enumerations::{DetonationResult, DetonationTypeIndicator};
use crate::BodyRaw;
use nom::multi::count;
use nom::number::complete::{be_u16, be_u8};
use nom::IResult;
use nom::Parser;

pub(crate) fn detonation_body(
    header: &PduHeader,
) -> impl Fn(&[u8]) -> IResult<&[u8], PduBody> + '_ {
    move |input: &[u8]| {
        let dti = header
            .pdu_status
            .unwrap_or_default()
            .detonation_type_indicator
            .unwrap_or(DetonationTypeIndicator::Munition);
        let (input, source_entity_id) = parser::entity_id(input)?;
        let (input, target_entity_id) = parser::entity_id(input)?;
        let (input, exploding_entity_id) = parser::entity_id(input)?;
        let (input, event_it) = parser::event_id(input)?;
        let (input, velocity) = parser::vec3_f32(input)?;
        let (input, world_location) = parser::location(input)?;
        let (input, descriptor) = detonation_descriptor(dti)(input)?;
        let (input, entity_location) = parser::vec3_f32(input)?;
        let (input, detonation_result) = be_u8(input)?;
        let (input, variable_parameters_no) = be_u8(input)?;
        let (input, _padding) = be_u16(input)?;
        let (input, articulation_parameters) = if variable_parameters_no > 0 {
            count(parser::variable_parameter, variable_parameters_no as usize).parse(input)?
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

pub(crate) fn detonation_descriptor(
    detonation_type_indicator: DetonationTypeIndicator,
) -> impl Fn(&[u8]) -> IResult<&[u8], DetonationDescriptor> {
    move |input: &[u8]| match detonation_type_indicator {
        DetonationTypeIndicator::Munition => {
            let (input, munition) = parser::munition_descriptor(input)?;
            Ok((input, DetonationDescriptor::Munition(munition)))
        }
        DetonationTypeIndicator::NonmunitionExplosion => {
            let (input, explosion) = parser::explosion_descriptor(input)?;
            Ok((input, DetonationDescriptor::Explosion(explosion)))
        }
        // FIXME: DetonationTypeIndicator::Unspecified(_) should be an error; for now parse as Expendable, which has no data
        DetonationTypeIndicator::Expendable | DetonationTypeIndicator::Unspecified(_) => {
            let (input, expendable) = parser::expendable_descriptor(input)?;
            Ok((input, DetonationDescriptor::Expendable(expendable)))
        }
    }
}
