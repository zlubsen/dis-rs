use crate::enumerations::{AirPlatformCapabilities, CulturalFeatureCapabilities, EntityCapabilities, EntityKind, EnvironmentalCapabilities, ExpendableCapabilities, LandPlatformCapabilities, LifeFormsCapabilities, MunitionCapabilities, PlatformDomain, RadioCapabilities, SensorEmitterCapabilities, SpacePlatformCapabilities, SubsurfacePlatformCapabilities, SupplyCapabilities, SurfacePlatformCapabilities};
use crate::model::EntityType;

pub(crate) mod parser;

/// Helper function to convert the V7 on-wire format bytes (as u32) to an `EntityCapabilities` struct
/// based on the `EntityType` of the entity.
pub fn entity_capabilities_from_bytes(capabilities: u32, entity_type: &EntityType) -> EntityCapabilities {
    match (entity_type.kind, entity_type.domain) {
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
    }
}