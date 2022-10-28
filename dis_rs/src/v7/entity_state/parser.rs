use nom::IResult;
use nom::number::complete::be_u32;
use crate::enumerations::{EntityKind, EntityCapabilities, PlatformDomain};
use crate::enumerations::{LandPlatformCapabilities, AirPlatformCapabilities, SurfacePlatformCapabilities, SubsurfacePlatformCapabilities, SpacePlatformCapabilities, MunitionCapabilities, LifeFormsCapabilities, EnvironmentalCapabilities, CulturalFeatureCapabilities, SupplyCapabilities, RadioCapabilities, ExpendableCapabilities, SensorEmitterCapabilities};
use crate::{EntityType};

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