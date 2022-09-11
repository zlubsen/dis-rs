use dis_rs_macros::PduConversion;
use crate::v6::entity_state::builder::{AirPlatformBuilder, EnvironmentalBuilder, GeneralAppearanceBuilder, GuidedMunitionBuilder, LandPlatformBuilder, LifeFormBuilder, SpacePlatformBuilder, SubsurfacePlatformBuilder, SurfacePlatformBuilder};
use crate::VectorF32;
use crate::enumerations::{DeadReckoningAlgorithm};

pub struct Appearance {
    pub general_appearance : GeneralAppearance,
    pub specific_appearance : SpecificAppearance,
}

#[derive(Debug, PartialEq)]
pub struct GeneralAppearance {
    pub entity_paint_scheme : EntityPaintScheme, // enum
    pub entity_mobility_kill : EntityMobilityKill, // enum
    pub entity_fire_power : EntityFirePower, // enum
    pub entity_damage : EntityDamage, // enum
    pub entity_smoke : EntitySmoke, // enum
    pub entity_trailing_effect : EntityTrailingEffect, // enum
    pub entity_hatch_state : EntityHatchState, // enum
    pub entity_lights : EntityLights, // enum
    pub entity_flaming_effect : EntityFlamingEffect, // enum
}

impl GeneralAppearance {
    pub fn builder() -> GeneralAppearanceBuilder {
        GeneralAppearanceBuilder::new()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PduConversion)]
#[repr(u16)]
pub enum EntityPaintScheme {
    UniformColor = 0,
    Camouflage = 1,
}

impl Default for EntityPaintScheme {
    fn default() -> Self {
        EntityPaintScheme::UniformColor
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PduConversion)]
#[repr(u16)]
pub enum EntityMobilityKill {
    NoMobilityKill = 0,
    MobilityKill = 1,
}

impl Default for EntityMobilityKill {
    fn default() -> Self {
        EntityMobilityKill::NoMobilityKill
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PduConversion)]
#[repr(u16)]
pub enum EntityFirePower {
    NoFirePowerKill = 0,
    FirePowerKill = 1,
}

impl Default for EntityFirePower {
    fn default() -> Self {
        EntityFirePower::NoFirePowerKill
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PduConversion)]
#[repr(u16)]
pub enum EntityDamage {
    NoDamage = 0,
    SlightDamage = 1,
    ModerateDamage = 2,
    Destroyed = 3,
}

impl Default for EntityDamage {
    fn default() -> Self {
        EntityDamage::NoDamage
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PduConversion)]
#[repr(u16)]
pub enum EntitySmoke {
    NotSmoking = 0,
    SmokePlumeRising = 1,
    EmittingEngineSmoke = 2,
    EmittingEngineSmokeAndSmokePlumeRising = 3,
}

impl Default for EntitySmoke {
    fn default() -> Self {
        EntitySmoke::NotSmoking
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PduConversion)]
#[repr(u16)]
pub enum EntityTrailingEffect {
    None = 0,
    Small = 1,
    Medium = 2,
    Large = 3,
}

impl Default for EntityTrailingEffect {
    fn default() -> Self {
        EntityTrailingEffect::None
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PduConversion)]
#[repr(u16)]
pub enum EntityHatchState {
    NotApplicable = 0,
    Closed = 1,
    Popped = 2,
    PoppedAndPersonVisible = 3,
    Open = 4,
    OpenAndPersonVisible = 5,
    Unused1 = 6,
    Unused2 = 7,
}

impl Default for EntityHatchState {
    fn default() -> Self {
        EntityHatchState::NotApplicable
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PduConversion)]
#[repr(u16)]
pub enum EntityLights {
    None = 0,
    RunningLightsOn = 1,
    NavigationLightsOn = 2,
    FromationLightsOn = 3,
    Unused1 = 4,
    Unused2 = 5,
    Unused3 = 6,
    Unused4 = 7,
}

impl Default for EntityLights {
    fn default() -> Self {
        EntityLights::None
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PduConversion)]
#[repr(u16)]
pub enum EntityFlamingEffect {
    None = 0,
    FlamesPresent = 1,
}

impl Default for EntityFlamingEffect {
    fn default() -> Self {
        EntityFlamingEffect::None
    }
}

pub enum SpecificAppearance {
    LandPlatform(LandPlatformsRecord),
    AirPlatform(AirPlatformsRecord),
    SurfacePlatform(SurfacePlatformRecord),
    SubsurfacePlatform(SubsurfacePlatformsRecord),
    SpacePlatform(SpacePlatformsRecord),
    GuidedMunition(GuidedMunitionsRecord),
    LifeForm(LifeFormsRecord),
    Environmental(EnvironmentalsRecord),
    Other([u8;2]), // when we cannot determine the specific entity kind
}

impl SpecificAppearance {
    pub fn builder_land_platform() -> LandPlatformBuilder {
        LandPlatformBuilder::new()
    }

    pub fn builder_air_platform() -> AirPlatformBuilder {
        AirPlatformBuilder::new()
    }

    pub fn builder_surface_platform() -> SurfacePlatformBuilder {
        SurfacePlatformBuilder::new()
    }

    pub fn builder_subsurface_platform() -> SubsurfacePlatformBuilder {
        SubsurfacePlatformBuilder::new()
    }

    pub fn builder_space_platform() -> SpacePlatformBuilder {
        SpacePlatformBuilder::new()
    }

    pub fn builder_guided_munition() -> GuidedMunitionBuilder {
        GuidedMunitionBuilder::new()
    }

    pub fn builder_life_form() -> LifeFormBuilder {
        LifeFormBuilder::new()
    }

    pub fn builder_environmental() -> EnvironmentalBuilder {
        EnvironmentalBuilder::new()
    }
}

#[derive(Debug, PartialEq)]
pub struct LandPlatformsRecord {
    pub launcher : Launcher,
    pub camouflage_type : Camouflage,
    pub concealed : Concealed,
    pub frozen_status : FrozenStatus,
    pub power_plant_status : PowerPlantStatus,
    pub state : State,
    pub tent : Tent,
    pub ramp : Ramp,
}

#[derive(Debug, PartialEq)]
pub struct AirPlatformsRecord {
    pub afterburner : Afterburner,
    pub frozen_status : FrozenStatus,
    pub power_plant_status : PowerPlantStatus,
    pub state : State,
}

#[derive(Debug, PartialEq)]
pub struct SurfacePlatformRecord {
    pub frozen_status : FrozenStatus,
    pub power_plant_status : PowerPlantStatus,
    pub state : State,
}

#[derive(Debug, PartialEq)]
pub struct SubsurfacePlatformsRecord {
    pub frozen_status : FrozenStatus,
    pub power_plant_status : PowerPlantStatus,
    pub state : State,
}

#[derive(Debug, PartialEq)]
pub struct SpacePlatformsRecord {
    pub frozen_status : FrozenStatus,
    pub power_plant_status : PowerPlantStatus,
    pub state : State,
}

#[derive(Debug, PartialEq)]
pub struct GuidedMunitionsRecord {
    pub launch_flash : LaunchFlash,
    pub frozen_status : FrozenStatus,
    pub state : State,
}

#[derive(Debug, PartialEq)]
pub struct LifeFormsRecord {
    pub life_form_state : LifeFormsState,
    pub frozen_status : FrozenStatus,
    pub activity_state : ActivityState,
    pub weapon_1 : Weapon,
    pub weapon_2 : Weapon,
}

#[derive(Debug, PartialEq)]
pub struct EnvironmentalsRecord {
    pub density : Density,
}

#[derive(Copy, Clone, Debug, PartialEq, PduConversion)]
#[repr(u16)]
pub enum Launcher {
    NotRaised = 0,
    Raised = 1,
}

impl Default for Launcher {
    fn default() -> Self {
        Launcher::NotRaised
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PduConversion)]
#[repr(u16)]
pub enum Camouflage {
    Desert = 0,
    Winter = 1,
    Forest = 2,
}

impl Default for Camouflage {
    fn default() -> Self {
        Camouflage::Desert
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PduConversion)]
#[repr(u16)]
pub enum Concealed {
    NotConcealed = 0,
    Concealed = 1,
}

impl Default for Concealed {
    fn default() -> Self {
        Concealed::NotConcealed
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PduConversion)]
#[repr(u16)]
pub enum FrozenStatus {
    NotFrozen = 0,
    Frozen = 1,
}

impl Default for FrozenStatus {
    fn default() -> Self {
        FrozenStatus::NotFrozen
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PduConversion)]
#[repr(u16)]
pub enum PowerPlantStatus {
    Off = 0,
    On = 1,
}

impl Default for PowerPlantStatus {
    fn default() -> Self {
        PowerPlantStatus::Off
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PduConversion)]
#[repr(u16)]
pub enum State {
    Active = 0,
    Deactivated = 1,
}

impl Default for State {
    fn default() -> Self {
        State::Active
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PduConversion)]
#[repr(u16)]
pub enum Tent {
    NotExtended = 0,
    Extended = 1,
}

impl Default for Tent {
    fn default() -> Self {
        Tent::NotExtended
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PduConversion)]
#[repr(u16)]
pub enum Ramp {
    Up = 0,
    Down = 1,
}

impl Default for Ramp {
    fn default() -> Self {
        Ramp::Up
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PduConversion)]
#[repr(u16)]
pub enum Afterburner {
    NotOn = 0,
    On = 1,
}

impl Default for Afterburner {
    fn default() -> Self {
        Afterburner::NotOn
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PduConversion)]
#[repr(u16)]
pub enum LaunchFlash {
    NotPresent = 0,
    Present = 1,
}

impl Default for LaunchFlash {
    fn default() -> Self {
        LaunchFlash::NotPresent
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PduConversion)]
#[repr(u16)]
pub enum LifeFormsState {
    Null = 0,
    UprightStandingStill = 1,
    UprightWalking = 2,
    UprightRunning = 3,
    Kneeling = 4,
    Prone = 5,
    Crawling = 6,
    Swimming = 7,
    Parachuting = 8,
    Jumping = 9,
}

impl Default for LifeFormsState {
    fn default() -> Self {
        LifeFormsState::UprightStandingStill
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PduConversion)]
#[repr(u16)]
pub enum ActivityState {
    Active = 0,
    Deactivated = 1,
}

impl Default for ActivityState {
    fn default() -> Self {
        ActivityState::Active
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PduConversion)]
#[repr(u16)]
pub enum Weapon {
    NotPresent = 0,
    Stowed = 1,
    Deployed = 2,
    FiringPosition = 3,
}

impl Default for Weapon {
    fn default() -> Self {
        Weapon::NotPresent
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PduConversion)]
#[repr(u16)]
pub enum Density {
    Clear = 0,
    Hazy = 1,
    Dense = 2,
    VeryDense = 3,
    Opaque = 4,
}

impl Default for Density {
    fn default() -> Self {
        Density::Clear
    }
}

#[derive(Debug, PartialEq)]
pub struct EntityCapabilities {
    pub ammunition_supply : bool,
    pub fuel_supply : bool,
    pub recovery : bool,
    pub repair : bool,
}

pub struct DrParameters {
    pub algorithm : DeadReckoningAlgorithm,
    pub other_parameters : [u8; 15],
    pub linear_acceleration : VectorF32,
    pub angular_velocity : VectorF32,
}
