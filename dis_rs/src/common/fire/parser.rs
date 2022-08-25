use nom::IResult;
use nom::number::complete::{be_f32, be_u16, be_u32};
use crate::common::fire::model::Fire;
use crate::common::model::BurstDescriptor;
use crate::common::parser::{entity_id, entity_type, event_id, location, vec3_f32};
use crate::{MunitionDescriptorFuse, MunitionDescriptorWarhead, PduBody};

pub fn fire_body() -> impl Fn(&[u8]) -> IResult<&[u8], PduBody> {
    move |input: &[u8]| {
        let (input, firing_entity_id) = entity_id(input)?;
        let (input, target_entity_id) = entity_id(input)?;
        let (input, munition_id) = entity_id(input)?;
        let (input, event_id) = event_id(input)?;
        let (input, fire_mission_index) = be_u32(input)?;
        let (input, location_in_world) = location(input)?;
        let (input, burst_descriptor) = burst_descriptor(input)?;//BurstDescriptor,
        let (input, velocity) = vec3_f32(input)?;
        let (input, range) = be_f32(input)?;

        let body = Fire {
            firing_entity_id,
            target_entity_id,
            munition_id,
            event_id,
            fire_mission_index,
            location_in_world,
            burst_descriptor,
            velocity,
            range,
        };

        Ok((input, PduBody::Fire(body)))
    }
}

pub fn burst_descriptor(input: &[u8]) -> IResult<&[u8], BurstDescriptor> {
    let (input, munition) = entity_type(input)?;
    let (input, warhead) = warhead(input)?;
    let (input, fuse) = fuse(input)?;
    let (input, quantity) = be_u16(input)?;
    let (input, rate) = be_u16(input)?;

    Ok((input, BurstDescriptor {
        munition,
        warhead,
        fuse,
        quantity,
        rate,
    }))
}

fn warhead(input: &[u8]) -> IResult<&[u8], MunitionDescriptorWarhead> {
    let (input, warhead) = be_u16(input)?;
    let warhead = MunitionDescriptorWarhead::from(warhead);
    Ok((input, warhead))
}

fn fuse(input: &[u8]) -> IResult<&[u8], MunitionDescriptorFuse> {
    let (input, fuse) = be_u16(input)?;
    let fuse = MunitionDescriptorFuse::from(fuse);
    Ok((input, fuse))
}