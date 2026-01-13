use crate::common::fire::model::{Fire, FireDescriptor};
use crate::common::model::{PduBody, PduHeader};
use crate::common::parser;
use crate::enumerations::FireTypeIndicator;
use crate::BodyRaw;
use nom::number::complete::{be_f32, be_u32};
use nom::IResult;

pub(crate) fn fire_body(header: &PduHeader) -> impl Fn(&[u8]) -> IResult<&[u8], PduBody> + '_ {
    move |input: &[u8]| {
        // The FireTypeIndicator determines how to parse the DescriptorRecord.
        // Defaulting to `FireTypeIndicator::Munition` handles compatibility for v6,
        // where there is no PduStatus record with FireTypeIndicator field.
        // V6 only defines the DescriptorRecord::Munition variant.
        let fti = header
            .pdu_status
            .unwrap_or_default()
            .fire_type_indicator
            .unwrap_or(FireTypeIndicator::Munition);
        let (input, firing_entity_id) = parser::entity_id(input)?;
        let (input, target_entity_id) = parser::entity_id(input)?;
        let (input, munition_id) = parser::entity_id(input)?;
        let (input, event_id) = parser::event_id(input)?;
        let (input, fire_mission_index) = be_u32(input)?;
        let (input, location_in_world) = parser::location(input)?;
        let (input, descriptor) = fire_descriptor(fti)(input)?;
        let (input, velocity) = parser::vec3_f32(input)?;
        let (input, range) = be_f32(input)?;

        let body = Fire {
            firing_entity_id,
            target_entity_id,
            entity_id: munition_id,
            event_id,
            fire_mission_index,
            location_in_world,
            descriptor,
            velocity,
            range,
        };

        Ok((input, body.into_pdu_body()))
    }
}

pub(crate) fn fire_descriptor(
    fire_type_indicator: FireTypeIndicator,
) -> impl Fn(&[u8]) -> IResult<&[u8], FireDescriptor> {
    move |input: &[u8]| match fire_type_indicator {
        FireTypeIndicator::Munition => {
            let (input, munition) = parser::munition_descriptor(input)?;
            Ok((input, FireDescriptor::Munition(munition)))
        }
        // FIXME: FireTypeIndicator::Unspecified(_) should be an error; for now parse as Expendable, which has no data
        FireTypeIndicator::Expendable | FireTypeIndicator::Unspecified(_) => {
            let (input, expendable) = parser::expendable_descriptor(input)?;
            Ok((input, FireDescriptor::Expendable(expendable)))
        }
    }
}
