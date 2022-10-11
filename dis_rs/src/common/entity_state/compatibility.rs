use crate::v6::entity_state::model::EntityCapabilities;
use crate::enumerations::{EntityCapabilities as EntityCapabilitiesV7, LandPlatformCapabilities, AirPlatformCapabilities, SurfacePlatformCapabilities, SubsurfacePlatformCapabilities, SpacePlatformCapabilities, MunitionCapabilities, LifeFormsCapabilities, EnvironmentalCapabilities, CulturalFeatureCapabilities, SupplyCapabilities, RadioCapabilities, ExpendableCapabilities, SensorEmitterCapabilities};

impl From<EntityCapabilitiesV7> for EntityCapabilities {
    fn from(value: EntityCapabilitiesV7) -> Self {
        match value {
            EntityCapabilitiesV7::LandPlatformEntityCapabilities(capabilities) => capabilities.into(),
            EntityCapabilitiesV7::AirPlatformEntityCapabilities(capabilities) => capabilities.into(),
            EntityCapabilitiesV7::SurfacePlatformEntityCapabilities(capabilities) => capabilities.into(),
            EntityCapabilitiesV7::SubsurfacePlatformEntityCapabilities(capabilities) => capabilities.into(),
            EntityCapabilitiesV7::SpacePlatformEntityCapabilities(capabilities) => capabilities.into(),
            EntityCapabilitiesV7::MunitionEntityCapabilities(capabilities) => capabilities.into(),
            EntityCapabilitiesV7::LifeFormsEntityCapabilities(capabilities) => capabilities.into(),
            EntityCapabilitiesV7::EnvironmentalEntityCapabilities(capabilities) => capabilities.into(),
            EntityCapabilitiesV7::CulturalFeatureEntityCapabilities(capabilities) => capabilities.into(),
            EntityCapabilitiesV7::SupplyEntityCapabilities(capabilities) => capabilities.into(),
            EntityCapabilitiesV7::RadioEntityCapabilities(capabilities) => capabilities.into(),
            EntityCapabilitiesV7::ExpendableEntityCapabilities(capabilities) => capabilities.into(),
            EntityCapabilitiesV7::SensorEmitterEntityCapabilities(capabilities) => capabilities.into(),
            EntityCapabilitiesV7::Unspecified(_unspecified_value) => EntityCapabilities::default(),
        }
    }
}

impl From<LandPlatformCapabilities> for EntityCapabilities {
    fn from(value: LandPlatformCapabilities) -> Self {
        Self {
            ammunition_supply: value.ammunition_supply,
            fuel_supply: value.fuel_supply,
            recovery: value.recovery,
            repair: value.repair,
        }
    }
}

impl From<AirPlatformCapabilities> for EntityCapabilities {
    fn from(value: AirPlatformCapabilities) -> Self {
        Self {
            ammunition_supply: value.ammunition_supply,
            fuel_supply: value.fuel_supply,
            recovery: value.recovery,
            repair: value.repair,
        }
    }
}

impl From<SurfacePlatformCapabilities> for EntityCapabilities {
    fn from(value: SurfacePlatformCapabilities) -> Self {
        Self {
            ammunition_supply: value.ammunition_supply,
            fuel_supply: value.fuel_supply,
            recovery: value.recovery,
            repair: value.repair,
        }
    }
}

impl From<SubsurfacePlatformCapabilities> for EntityCapabilities {
    fn from(value: SubsurfacePlatformCapabilities) -> Self {
        Self {
            ammunition_supply: value.ammunition_supply,
            fuel_supply: value.fuel_supply,
            recovery: value.recovery,
            repair: value.repair,
        }
    }
}

impl From<SpacePlatformCapabilities> for EntityCapabilities {
    fn from(value: SpacePlatformCapabilities) -> Self {
        Self {
            ammunition_supply: value.ammunition_supply,
            fuel_supply: value.fuel_supply,
            recovery: value.recovery,
            repair: value.repair,
        }
    }
}

impl From<MunitionCapabilities> for EntityCapabilities {
    fn from(_value: MunitionCapabilities) -> Self {
        Self::default()
    }
}

impl From<LifeFormsCapabilities> for EntityCapabilities {
    fn from(value: LifeFormsCapabilities) -> Self {
        Self {
            ammunition_supply: value.ammunition_supply,
            fuel_supply: value.fuel_supply,
            recovery: value.recovery,
            repair: value.repair,
        }
    }
}

impl From<EnvironmentalCapabilities> for EntityCapabilities {
    fn from(_value: EnvironmentalCapabilities) -> Self {
        Self::default()
    }
}

impl From<CulturalFeatureCapabilities> for EntityCapabilities {
    fn from(_value: CulturalFeatureCapabilities) -> Self {
        Self::default()
    }
}

impl From<SupplyCapabilities> for EntityCapabilities {
    fn from(value: SupplyCapabilities) -> Self {
        Self {
            ammunition_supply: value.ammunition_supply,
            fuel_supply: value.fuel_supply,
            recovery: false,
            repair: false,
        }
    }
}

impl From<RadioCapabilities> for EntityCapabilities {
    fn from(_value: RadioCapabilities) -> Self {
        Self::default()
    }
}

impl From<ExpendableCapabilities> for EntityCapabilities {
    fn from(_value: ExpendableCapabilities) -> Self {
        Self::default()
    }
}

impl From<SensorEmitterCapabilities> for EntityCapabilities {
    fn from(_value: SensorEmitterCapabilities) -> Self {
        Self::default()
    }
}

impl From<EntityCapabilities> for EntityCapabilitiesV7 {
    fn from(value: EntityCapabilities) -> Self {
        Self::LandPlatformEntityCapabilities(LandPlatformCapabilities {
            ammunition_supply: value.ammunition_supply,
            fuel_supply: value.fuel_supply,
            recovery: value.recovery,
            repair: value.repair,
            reserved: false,
            sling_loadable: false,
            ied_presence_indicator: false,
            task_organizable: false,
        })
    }
}