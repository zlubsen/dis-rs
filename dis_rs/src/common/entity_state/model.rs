use dis_rs_macros::PduConversion;
use crate::common::entity_state::builder::{AirPlatformBuilder, EntityStateBuilder, EnvironmentalBuilder, GeneralAppearanceBuilder, GuidedMunitionBuilder, LandPlatformBuilder, LifeFormBuilder, SpacePlatformBuilder, SubsurfacePlatformBuilder, SurfacePlatformBuilder};
use crate::common::Interaction;
use crate::common::model::{EntityId, EntityType, Location, Orientation, VectorF32};
use crate::enumerations::ForceID;

// TODO sensible errors for EntityState
pub enum EntityStateValidationError {
    SomeFieldNotOkError,
}

pub struct EntityState {
    pub entity_id : EntityId, // struct
    pub force_id : ForceID, // enum
    pub entity_type : EntityType, // struct
    pub alternative_entity_type : EntityType, // struct
    pub entity_linear_velocity : VectorF32, // struct
    pub entity_location : Location, // struct
    pub entity_orientation : Orientation, // struct
    pub entity_appearance : Appearance, // struct
    pub dead_reckoning_parameters : DrParameters, // struct
    pub entity_marking : EntityMarking, // struct
    pub entity_capabilities : EntityCapabilities, // struct
    pub articulation_parameter : Option<Vec<ArticulationParameter>>, // optional list of records
}

// #[derive(Copy, Clone, Debug, PartialEq, PduConversion)]
// #[repr(u8)]
// pub enum ForceId {
//     Other = 0,
//     Friendly = 1,
//     Opposing = 2,
//     Neutral = 3,
// }
//
// impl Default for ForceId {
//     fn default() -> Self {
//         ForceId::Other
//     }
// }

// // TODO enumeration refactoring
// #[derive(Copy, Clone, Debug, PartialEq, PduConversion)]
// #[repr(u8)]
// pub enum EntityKind {
//     Other = 0,
//     Platform = 1,
//     Munition = 2,
//     LifeForm = 3,
//     Environmental = 4,
//     CulturalFeature = 5,
//     Supply = 6,
//     Radio = 7,
//     Expendable = 8,
//     SensorEmitter = 9,
// }
//
// impl Default for EntityKind {
//     fn default() -> Self {
//         EntityKind::Other
//     }
// }

// TODO enumeration refactoring
// regex: (?<value>[0-9]*)[\t]+(?<field>[\w (),'-.]+)$
// replace: \t${field} = ${value}, | $2 = $1
#[derive(Copy, Clone, Debug, PartialEq, PduConversion)]
#[repr(u16)]
pub enum Country {
    Other = 0,
    Afghanistan = 1,
    Albania = 2,
    Algeria = 3,
    AmericanSamoa = 4,
    Andorra = 5,
    Angola = 6,
    Anguilla = 7,
    Antarctica = 8,
    AntiguaBarbuda = 9,
    Argentina = 10,
    Aruba = 11,
    AshmoreCartierIslands = 12,
    Australia = 13,
    Austria = 14,
    Bahamas = 15,
    Bahrain = 16,
    BakerIsland = 17,
    Bangladesh = 18,
    Barbados = 19,
    BassasDaIndia = 20,
    Belgium = 21,
    Belize = 22,
    Benin = 23,
    Bermuda = 24,
    Bhutan = 25,
    Bolivia = 26,
    Botswana = 27,
    BouvetIsland = 28,
    Brazil = 29,
    BritishIndianOceanTerritory = 30,
    BritishVirginIslands = 31,
    Brunei = 32,
    Bulgaria = 33,
    Burkina = 34,
    Burma = 35,
    Burundi = 36,
    Cambodia = 37,
    Cameroon = 38,
    Canada = 39,
    CapeVerde = 40,
    CaymanIslands = 41,
    CentralAfricanRepublic = 42,
    Chad = 43,
    Chile = 44,
    China = 45,
    ChristmasIsland = 46,
    CocosIslands = 47,
    Colombia = 48,
    Comoros = 49,
    Congo = 50,
    CookIslands = 51,
    CoralSeaIslands = 52,
    CostaRica = 53,
    Cuba = 54,
    Cyprus = 55,
    Czechoslovakia = 56,
    Denmark = 57,
    Djibouti = 58,
    Dominica = 59,
    DominicanRepublic = 60,
    Ecuador = 61,
    Egypt = 62,
    ElSalvador = 63,
    EquatorialGuinea = 64,
    Ethiopia = 65,
    EuropaIsland = 66,
    FalklandIslands = 67,
    FaroeIslands = 68,
    Fiji = 69,
    Finland = 70,
    France = 71,
    FrenchGuiana = 72,
    FrenchPolynesia = 73,
    FrenchSouthernAntarcticIslands = 74,
    Gabon = 75,
    GambiaThe = 76,
    GazaStrip = 77,
    Germany = 78,
    Ghana = 79,
    Gibraltar = 80,
    GloriosoIslands = 81,
    Greece = 82,
    Greenland = 83,
    Grenada = 84,
    Guadaloupe = 85,
    Guam = 86,
    Guatemala = 87,
    Guernsey = 88,
    Guinea = 89,
    GuineaBissau = 90,
    Guyana = 91,
    Haiti = 92,
    HeardIslandMcDonaldIslands = 93,
    Honduras = 94,
    HongKong = 95,
    HowlandIsland = 96,
    Hungary = 97,
    Iceland = 98,
    India = 99,
    Indonesia = 100,
    Iran = 101,
    Iraq = 102,
    Ireland = 104,
    Israel = 105,
    Italy = 106,
    CoteDIvoire = 107,
    Jamaica = 108,
    JanMayen = 109,
    Japan = 110,
    JarvisIsland = 111,
    Jersey = 112,
    JohnstonAtoll = 113,
    Jordan = 114,
    JuanDeNovaIsland = 115,
    Kenya = 116,
    KingmanReef = 117,
    Kiribati = 118,
    KoreaNorth = 119,
    KoreaSouth = 120,
    Kuwait = 121,
    Laos = 122,
    Lebanon = 123,
    Lesotho = 124,
    Liberia = 125,
    Libya = 126,
    Liechtenstein = 127,
    Luxembourg = 128,
    Madagascar = 129,
    Macau = 130,
    Malawi = 131,
    Malaysia = 132,
    Maldives = 133,
    Mali = 134,
    Malta = 135,
    ManIsle = 136,
    MarshallIslands = 137,
    Martinique = 138,
    Mauritania = 139,
    Mauritius = 140,
    Mayotte = 141,
    Mexico = 142,
    Micronesia = 143,
    Monaco = 144,
    Mongolia = 145,
    Montserrat = 146,
    Morocco = 147,
    Mozambique = 148,
    Namibia = 149,
    Nauru = 150,
    NavassaIsland = 151,
    Nepal = 152,
    Netherlands = 153,
    NetherlandsAntilles = 154,
    NewCaledonia = 155,
    NewZealand = 156,
    Nicaragua = 157,
    Niger = 158,
    Nigeria = 159,
    Niue = 160,
    NorfolkIsland = 161,
    NorthernMarianaIslands = 162,
    Norway = 163,
    Oman = 164,
    Pakistan = 165,
    PalmyraAtoll = 166,
    Panama = 168,
    PapuaNewGuinea = 169,
    ParacelIslands = 170,
    Paraguay = 171,
    Peru = 172,
    Philippines = 173,
    PitcairnIslands = 174,
    Poland = 175,
    Portugal = 176,
    PuertoRico = 177,
    Qatar = 178,
    Reunion = 179,
    Romania = 180,
    Rwanda = 181,
    StKittsAndNevis = 182,
    StHelena = 183,
    StLucia = 184,
    StPierreAndMiquelon = 185,
    StVincentAndTheGrenadines = 186,
    SanMarino = 187,
    SaoTomeAndPrincipe = 188,
    SaudiArabia = 189,
    Senegal = 190,
    Seychelles = 191,
    SierraLeone = 192,
    Singapore = 193,
    SolomonIslands = 194,
    Somalia = 195,
    SouthGeorgiaSouthSandwichIslands = 196,
    SouthAfrica = 197,
    Spain = 198,
    SpratlyIslands = 199,
    SriLanka = 200,
    Sudan = 201,
    Suriname = 202,
    SvalbardNorway = 203,
    Swaziland = 204,
    Sweden = 205,
    Switzerland = 206,
    Syria = 207,
    Taiwan = 208,
    Tanzania = 209,
    Thailand = 210,
    Togo = 211,
    Tokelau = 212,
    Tonga = 213,
    TrinidadAndTobago = 214,
    TromelinIsland = 215,
    PacificIslands = 216,
    Tunisia = 217,
    Turkey = 218,
    TurksCaicosIslands = 219,
    Tuvalu = 220,
    Uganda = 221,
    CommonwealthOfIndependentStates = 222,
    UnitedArabEmirates = 223,
    UnitedKingdom = 224,
    UnitedStates = 225,
    Uruguay = 226,
    Vanuatu = 227,
    VaticanCity = 228,
    Venezuela = 229,
    Vietnam = 230,
    VirginIslands = 231,
    WakeIsland = 232,
    WallisFutuna = 233,
    WesternSahara = 234,
    WestBank = 235,
    WesternSamoa = 236,
    Yemen = 237,
    Zaire = 241,
    Zambia = 242,
    Zimbabwe = 243,
    Armenia = 244,
    Azerbaijan = 245,
    Belarus = 246,
    BosniaHercegovina = 247,
    ClippertonIsland = 248,
    Croatia = 249,
    Estonia = 250,
    Georgia = 251,
    Kazakhstan = 252,
    Kyrgyzstan = 253,
    Latvia = 254,
    Lithuania = 255,
    Macedonia = 256,
    MidwayIslands = 257,
    Moldova = 258,
    Montenegro = 259,
    Russia = 260,
    SerbiaMontenegro = 261,
    Slovenia = 262,
    Tajikistan = 263,
    Turkmenistan = 264,
    Ukraine = 265,
    Uzbekistan = 266,
}

impl Default for Country {
    fn default() -> Self {
        Country::Other
    }
}

pub struct EntityMarking {
    pub marking_character_set : EntityMarkingCharacterSet,
    pub marking_string : String, // 11 byte String
}

// TODO enumeration refactoring
#[derive(Copy, Clone, PartialEq, Debug, PduConversion)]
#[repr(u8)]
pub enum EntityMarkingCharacterSet {
    Unused = 0,
    ASCII = 1,
    ArmyMarking = 2,
    DigitChevron = 3,
}

impl Default for EntityMarkingCharacterSet {
    fn default() -> Self {
        EntityMarkingCharacterSet::ASCII
    }
}

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

pub struct DrParameters {
    pub algorithm : DrAlgorithm,
    pub other_parameters : [u8; 15],
    pub linear_acceleration : VectorF32,
    pub angular_velocity : VectorF32,
}

#[derive(Copy, Clone, Debug, PartialEq, PduConversion)]
#[repr(u8)]
pub enum DrAlgorithm {
    Other = 0,
    Static = 1,
    DrmFPW = 2,
    DrmRPW = 3,
    DrmRVW = 4,
    DrmFVW = 5,
    DrmFPB = 6,
    DrmRPB = 7,
    DrmRVB = 8,
    DrmFVB = 9,
}

impl Default for DrAlgorithm {
    fn default() -> Self {
        DrAlgorithm::Other
    }
}

#[derive(Debug, PartialEq)]
pub struct EntityCapabilities {
    pub ammunition_supply : bool,
    pub fuel_supply : bool,
    pub recovery : bool,
    pub repair : bool,
    // 28-bits padding
}

pub struct ArticulationParameter {
    pub parameter_type_designator : ApTypeDesignator,
    pub parameter_change_indicator : u8,
    pub articulation_attachment_id: u16,
    pub parameter_type_variant : ParameterTypeVariant,
    pub articulation_parameter_value : f32,
}

#[derive(Copy, Clone, Debug, PartialEq, PduConversion)]
#[repr(u8)]
pub enum ApTypeDesignator {
    Articulated = 0,
    Attached = 1,
}

impl Default for ApTypeDesignator {
    fn default() -> Self {
        ApTypeDesignator::Articulated
    }
}

#[derive(Debug, PartialEq)]
pub enum ParameterTypeVariant {
    AttachedParts(u32),
    // 0	Nothing, Empty
    // 1-511	Sequential IDs for model-specific stations
    // 512-639	Fuselage Stations
    // 640-767	Left-wing Stations
    // 768-895	Right-wing Stations
    // 896	M16A42 rifle
    // 897	M249 SAW
    // 898	M60 Machine gun
    // 899	M203 Grenade Launcher
    // 900	M136 AT4
    // 901	M47 Dragon
    // 902	AAWS-M Javelin
    // 903	M18A1 Claymore Mine
    // 904	MK19 Grenade Launcher
    // 905	M2 Machine Gun
    // 906-1023	Other attached parts
    ArticulatedParts(ArticulatedParts),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ArticulatedParts {
    pub type_metric : ApTypeMetric,
    pub type_class : u32,
}

#[derive(Copy, Clone, Debug, PartialEq, PduConversion)]
#[repr(u32)]
pub enum ApTypeMetric {
    Unspecified = 0,
    Position = 1,
    PositionRate = 2,
    Extension = 3,
    ExtensionRate = 4,
    X = 5,
    XRate = 6,
    Y = 7,
    YRate = 8,
    Z = 9,
    ZRate = 10,
    Azimuth = 11,
    AzimuthRate = 12,
    Elevation = 13,
    ElevationRate = 14,
    Rotation = 15,
    RotationRate = 16,
}

impl Default for ApTypeMetric {
    fn default() -> Self {
        ApTypeMetric::Unspecified
    }
}

// pub enum ApHighBits {
//     Placeholder = 0,
// TODO finish enum values
// 1024	rudder
// 1056	left flap
// 1088	right flap
// 1120	left aileron
// 1152	right aileron
// 1184	helicopter - main rotor
// 1216	helicopter - tail rotor
// 1248	other Aircraft Control Surfaces defined as needed
// 2048	periscope
// 2080	generic antenna
// 2112	snorkel
// 2144	other extendable parts defined as needed
// 3072	landing gear
// 3104	tail hook
// 3136	speed brake
// 3168	left weapon bay door
// 3200	right weapon bay doors
// 3232	tank or APC hatch
// 3264	wingsweep
// 3296	Bridge launcher
// 3328	Bridge section 1
// 3360	Bridge section 2
// 3392	Bridge section 3
// 3424	Primary blade 1
// 3456	Primary blade 2
// 3488	Primary boom
// 3520	Primary launcher arm
// 3552	other fixed position parts defined as needed
// 4096	Primary turret number 1
// 4128	Primary turret number 2
// 4160	Primary turret number 3
// 4192	Primary turret number 4
// 4224	Primary turret number 5
// 4256	Primary turret number 6
// 4288	Primary turret number 7
// 4320	Primary turret number 8
// 4352	Primary turret number 9
// 4384	Primary turret number 10
// 4416	Primary gun number 1
// 4448	Primary gun number 2
// 4480	Primary gun number 3
// 4512	Primary gun number 4
// 4544	Primary gun number 5
// 4576	Primary gun number 6
// 4608	Primary gun number 7
// 4640	Primary gun number 8
// 4672	Primary gun number 9
// 4704	Primary gun number 10
// 4736	Primary launcher 1
// 4768	Primary launcher 2
// 4800	Primary launcher 3
// 4832	Primary launcher 4
// 4864	Primary launcher 5
// 4896	Primary launcher 6
// 4928	Primary launcher 7
// 4960	Primary launcher 8
// 4992	Primary launcher 9
// 5024	Primary launcher 10
// 5056	Primary defense systems 1
// 5088	Primary defense systems 2
// 5120	Primary defense systems 3
// 5152	Primary defense systems 4
// 5184	Primary defense systems 5
// 5216	Primary defense systems 6
// 5248	Primary defense systems 7
// 5280	Primary defense systems 8
// 5312	Primary defense systems 9
// 5344	Primary defense systems 10
// 5376	Primary radar 1
// 5408	Primary radar 2
// 5440	Primary radar 3
// 5472	Primary radar 4
// 5504	Primary radar 5
// 5536	Primary radar 6
// 5568	Primary radar 7
// 5600	Primary radar 8
// 5632	Primary radar 9
// 5664	Primary radar 10
// 5696	Secondary turret number 1
// 5728	Secondary turret number 2
// 5760	Secondary turret number 3
// 5792	Secondary turret number 4
// 5824	Secondary turret number 5
// 5856	Secondary turret number 6
// 5888	Secondary turret number 7
// 5920	Secondary turret number 8
// 5952	Secondary turret number 9
// 5984	Secondary turret number 10
// 6016	Secondary gun number 1
// 6048	Secondary gun number 2
// 6080	Secondary gun number 3
// 6112	Secondary gun number 4
// 6144	Secondary gun number 5
// 6176	Secondary gun number 6
// 6208	Secondary gun number 7
// 6240	Secondary gun number 8
// 6272	Secondary gun number 9
// 6304	Secondary gun number 10
// 6336	Secondary launcher 1
// 6368	Secondary launcher 2
// 6400	Secondary launcher 3
// 6432	Secondary launcher 4
// 6464	Secondary launcher 5
// 6496	Secondary launcher 6
// 6528	Secondary launcher 7
// 6560	Secondary launcher 8
// 6592	Secondary launcher 9
// 6624	Secondary launcher 10
// 6656	Secondary defense systems 1
// 6688	Secondary defense systems 2
// 6720	Secondary defense systems 3
// 6752	Secondary defense systems 4
// 6784	Secondary defense systems 5
// 6816	Secondary defense systems 6
// 6848	Secondary defense systems 7
// 6880	Secondary defense systems 8
// 6912	Secondary defense systems 9
// 6944	Secondary defense systems 10
// 6976	Secondary radar 1
// 7008	Secondary radar 2
// 7040	Secondary radar 3
// 7072	Secondary radar 4
// 7104	Secondary radar 5
// 7136	Secondary radar 6
// 7168	Secondary radar 7
// 7200	Secondary radar 8
// 7232	Secondary radar 9
// 7264	Secondary radar 10
// }

// TODO refactor builder
impl EntityState {
    pub fn builder() -> EntityStateBuilder {
        EntityStateBuilder::new()
    }
}

impl Interaction for EntityState {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.entity_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        None
    }
}