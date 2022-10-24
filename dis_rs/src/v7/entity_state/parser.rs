use nom::{bits, IResult};
use nom::error::Error;
use nom::sequence::tuple;
use nom::complete::take as take_bits;
use nom::bytes::complete::take as take_bytes;
use nom::multi::count;
use nom::number::complete::{be_u16, be_u32, be_u8};
use crate::enumerations::*;
use crate::{EntityState, EntityType, PduBody};
use crate::common::parser;

pub fn entity_state_body(input: &[u8]) -> IResult<&[u8], PduBody> {
    todo!()
    // let (input, entity_id_val) = parser::entity_id(input)?;
    // let (input, force_id_val) = parser::force_id(input)?;
    // let (input, articulated_parts_no) = be_u8(input)?;
    // let (input, entity_type_val) = parser::entity_type(input)?;
    // let (input, alternative_entity_type) = parser::entity_type(input)?;
    // let (input, entity_linear_velocity) = parser::vec3_f32(input)?;
    // let (input, entity_location) = parser::location(input)?;
    // let (input, entity_orientation) = parser::orientation(input)?;
    // let (input, entity_appearance) = entity_appearance(entity_type_val)(input)?;
    // let (input, dead_reckoning_parameters) = dr_parameters(input)?;
    // let (input, entity_marking) = parser::entity_marking(input)?;
    // let (input, entity_capabilities) = entity_capabilities(input)?;
    // let (input, articulation_parameter) = if articulated_parts_no > 0 {
    //     let (input, params) = count(articulation_record, articulated_parts_no as usize)(input)?;
    //     (input, Some(params))
    // } else { (input, None) };
    //
    // let builder = EntityState::builder()
    //     // .header(header)
    //     .entity_id(entity_id_val)
    //     .force_id(force_id_val)
    //     .entity_type(entity_type_val)
    //     .alt_entity_type(alternative_entity_type)
    //     .linear_velocity(entity_linear_velocity)
    //     .location(entity_location)
    //     .orientation(entity_orientation)
    //     .appearance(entity_appearance)
    //     .dead_reckoning(dead_reckoning_parameters)
    //     .marking(entity_marking)
    //     .capabilities(entity_capabilities);
    // let builder = if let Some(params) = articulation_parameter {
    //     builder.add_articulation_parameters_vec(params)
    // } else { builder };
    // let body = builder.build();
    //
    // Ok((input, body.unwrap()))
}

// pub fn entity_appearance(entity_type: EntityType) -> impl Fn(&[u8]) -> IResult<&[u8], Appearance> {
//     move | input: &[u8] | {
//         todo!()
//     }
// }

pub fn entity_capabilities(entity_type: EntityType) -> impl Fn(&[u8]) -> IResult<&[u8], EntityCapabilities> {
    move | input: &[u8] | {
        let (input, capabilities) = be_u32(input)?;
        let capabilities = match (entity_type.kind, entity_type.domain) {
            (EntityKind::Other, _) => EntityCapabilities::Unspecified(0u32),
            (EntityKind::Platform, PlatformDomain::Land) => EntityCapabilities::LandPlatformEntityCapabilities(LandPlatformCapabilities::from(capabilities)),
            (EntityKind::Platform, PlatformDomain::Air) => EntityCapabilities::AirPlatformEntityCapabilities(AirPlatformCapabilities::from(capabilities)),
            (EntityKind::Platform, PlatformDomain::Surface) => EntityCapabilities::SurfacePlatformEntityCapabilities(SurfacePlatformCapabilities::from(capabilities)),
            (EntityKind::Platform, PlatformDomain::Subsurface) => EntityCapabilities::SubsurfacePlatformEntityCapabilities(SubsurfacePlatformCapabilities::from(capabilities)),
            (EntityKind::Platform, PlatformDomain::Space) => EntityCapabilities::SpacePlatformEntityCapabilities(SpacePlatformCapabilities::from(capabilities)),
            (EntityKind::Munition, _) => EntityCapabilities::MunitionEntityCapabilities(MunitionCapabilities::from(capabilities)),
            (EntityKind::Lifeform, _) => EntityCapabilities::LifeFormsEntityCapabilities(LifeFormsCapabilities::from(capabilities)),
            (EntityKind::Environmental, _) => EntityCapabilities::EnvironmentalEntityCapabilities(EnvironmentalCapabilities::from(capabilities)),
            (EntityKind::Culturalfeature, _) => EntityCapabilities::CulturalFeatureEntityCapabilities(CulturalFeatureCapabilities::from(capabilities)),
            (EntityKind::Supply, _) => EntityCapabilities::SupplyEntityCapabilities(SupplyCapabilities::from(capabilities)),
            (EntityKind::Radio, _) => EntityCapabilities::RadioEntityCapabilities(RadioCapabilities::from(capabilities)),
            (EntityKind::Expendable, _) => EntityCapabilities::ExpendableEntityCapabilities(ExpendableCapabilities::from(capabilities)),
            (EntityKind::SensorEmitter, _) => EntityCapabilities::SensorEmitterEntityCapabilities(SensorEmitterCapabilities::from(capabilities)),
            (_, _) => EntityCapabilities::Unspecified(capabilities)
        };

        Ok((input, capabilities))
    }
}