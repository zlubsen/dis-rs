use bytes::BytesMut;
use crate::dis::errors::DisError;
use crate::dis::v6::model::PduHeader;
use super::builder::EntityStateBuilder;

// TODO check primitive types in TryFrom impls for enums (u8, u16, bits, ...)

// TODO sensible errors for EntityState
pub enum EntityStateValidationError {
    SomeFieldNotOkError,
}

pub struct EntityState {
    pub header : PduHeader, // struct
    pub entity_id : EntityId, // struct
    pub force_id : ForceId, // enum
    pub articulated_parts_no : u8, // FIXME can be obtained from length of articulation_parameter field
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

pub struct EntityId {
    pub(crate) simulation_address : SimulationAddress,
    entity_id : u16
}

pub struct SimulationAddress {
    site_id : u16,
    application_id : u16,
}

pub enum ForceId {
    Other = 0,
    Friendly = 1,
    Opposing = 2,
    Neutral = 3,
}

impl TryFrom<u8> for ForceId {
    type Error = DisError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ForceId::Other),
            1 => Ok(ForceId::Friendly),
            2 => Ok(ForceId::Opposing),
            3 => Ok(ForceId::Neutral),
            n => Err(DisError::InvalidEnumValue(1, n as usize)),
        }
    }
}

pub struct EntityType {
    kind : EntityKind,
    domain : u8,
    country : Country, // TODO u16 instead of big enum? Put codes and names in config file?
    category : u8,
    subcategory : u8,
    specific : u8,
    extra : u8,
}

pub enum EntityKind {
    Other = 0,
    Platform = 1,
    Munition = 2,
    LifeForm = 3,
    Environmental = 4,
    CulturalFeature = 5,
    Supply = 6,
    Radio = 7,
    Expendable = 8,
    SensorEmitter = 9,
}

impl TryFrom<u8> for EntityKind {
    type Error = DisError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(EntityKind::Other),
            1 => Ok(EntityKind::Platform),
            2 => Ok(EntityKind::Munition),
            3 => Ok(EntityKind::LifeForm),
            4 => Ok(EntityKind::Environmental),
            5 => Ok(EntityKind::CulturalFeature),
            6 => Ok(EntityKind::Supply),
            7 => Ok(EntityKind::Radio),
            8 => Ok(EntityKind::Expendable),
            9 => Ok(EntityKind::SensorEmitter),
            n => Err(DisError::InvalidEnumValue(9, n as usize)),
        }
    }
}

// regex: (?<value>[0-9]*)[\t]+(?<field>[\w (),'-.]+)$
// replace: \t${field} = ${value}, | $2 = $1
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
    Svalbard_Norway = 203,
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

impl TryFrom<u8> for Country {
    type Error = DisError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Country::Other),
            1 => Ok(Country::Afghanistan),
            2 => Ok(Country::Albania),
            3 => Ok(Country::Algeria),
            4 => Ok(Country::AmericanSamoa),
            5 => Ok(Country::Andorra),
            6 => Ok(Country::Angola),
            7 => Ok(Country::Anguilla),
            8 => Ok(Country::Antarctica),
            9 => Ok(Country::AntiguaBarbuda),
            10 => Ok(Country::Argentina),
            11 => Ok(Country::Aruba),
            12 => Ok(Country::AshmoreCartierIslands),
            13 => Ok(Country::Australia),
            14 => Ok(Country::Austria),
            15 => Ok(Country::Bahamas),
            16 => Ok(Country::Bahrain),
            17 => Ok(Country::BakerIsland),
            18 => Ok(Country::Bangladesh),
            19 => Ok(Country::Barbados),
            20 => Ok(Country::BassasDaIndia),
            21 => Ok(Country::Belgium),
            22 => Ok(Country::Belize),
            23 => Ok(Country::Benin),
            24 => Ok(Country::Bermuda),
            25 => Ok(Country::Bhutan),
            26 => Ok(Country::Bolivia),
            27 => Ok(Country::Botswana),
            28 => Ok(Country::BouvetIsland),
            29 => Ok(Country::Brazil),
            30 => Ok(Country::BritishIndianOceanTerritory),
            31 => Ok(Country::BritishVirginIslands),
            32 => Ok(Country::Brunei),
            33 => Ok(Country::Bulgaria),
            34 => Ok(Country::Burkina),
            35 => Ok(Country::Burma),
            36 => Ok(Country::Burundi),
            37 => Ok(Country::Cambodia),
            38 => Ok(Country::Cameroon),
            39 => Ok(Country::Canada),
            40 => Ok(Country::CapeVerde),
            41 => Ok(Country::CaymanIslands),
            42 => Ok(Country::CentralAfricanRepublic),
            43 => Ok(Country::Chad),
            44 => Ok(Country::Chile),
            45 => Ok(Country::China),
            46 => Ok(Country::ChristmasIsland),
            47 => Ok(Country::CocosIslands),
            48 => Ok(Country::Colombia),
            49 => Ok(Country::Comoros),
            50 => Ok(Country::Congo),
            51 => Ok(Country::CookIslands),
            52 => Ok(Country::CoralSeaIslands),
            53 => Ok(Country::CostaRica),
            54 => Ok(Country::Cuba),
            55 => Ok(Country::Cyprus),
            56 => Ok(Country::Czechoslovakia),
            57 => Ok(Country::Denmark),
            58 => Ok(Country::Djibouti),
            59 => Ok(Country::Dominica),
            60 => Ok(Country::DominicanRepublic),
            61 => Ok(Country::Ecuador),
            62 => Ok(Country::Egypt),
            63 => Ok(Country::ElSalvador),
            64 => Ok(Country::EquatorialGuinea),
            65 => Ok(Country::Ethiopia),
            66 => Ok(Country::EuropaIsland),
            67 => Ok(Country::FalklandIslands),
            68 => Ok(Country::FaroeIslands),
            69 => Ok(Country::Fiji),
            70 => Ok(Country::Finland),
            71 => Ok(Country::France),
            72 => Ok(Country::FrenchGuiana),
            73 => Ok(Country::FrenchPolynesia),
            74 => Ok(Country::FrenchSouthernAntarcticIslands),
            75 => Ok(Country::Gabon),
            76 => Ok(Country::GambiaThe),
            77 => Ok(Country::GazaStrip),
            78 => Ok(Country::Germany),
            79 => Ok(Country::Ghana),
            80 => Ok(Country::Gibraltar),
            81 => Ok(Country::GloriosoIslands),
            82 => Ok(Country::Greece),
            83 => Ok(Country::Greenland),
            84 => Ok(Country::Grenada),
            85 => Ok(Country::Guadaloupe),
            86 => Ok(Country::Guam),
            87 => Ok(Country::Guatemala),
            88 => Ok(Country::Guernsey),
            89 => Ok(Country::Guinea),
            90 => Ok(Country::GuineaBissau),
            91 => Ok(Country::Guyana),
            92 => Ok(Country::Haiti),
            93 => Ok(Country::HeardIslandMcDonaldIslands),
            94 => Ok(Country::Honduras),
            95 => Ok(Country::HongKong),
            96 => Ok(Country::HowlandIsland),
            97 => Ok(Country::Hungary),
            98 => Ok(Country::Iceland),
            99 => Ok(Country::India),
            100 => Ok(Country::Indonesia),
            101 => Ok(Country::Iran),
            102 => Ok(Country::Iraq),
            104 => Ok(Country::Ireland),
            105 => Ok(Country::Israel),
            106 => Ok(Country::Italy),
            107 => Ok(Country::CoteDIvoire),
            108 => Ok(Country::Jamaica),
            109 => Ok(Country::JanMayen),
            110 => Ok(Country::Japan),
            111 => Ok(Country::JarvisIsland),
            112 => Ok(Country::Jersey),
            113 => Ok(Country::JohnstonAtoll),
            114 => Ok(Country::Jordan),
            115 => Ok(Country::JuanDeNovaIsland),
            116 => Ok(Country::Kenya),
            117 => Ok(Country::KingmanReef),
            118 => Ok(Country::Kiribati),
            119 => Ok(Country::KoreaNorth),
            120 => Ok(Country::KoreaSouth),
            121 => Ok(Country::Kuwait),
            122 => Ok(Country::Laos),
            123 => Ok(Country::Lebanon),
            124 => Ok(Country::Lesotho),
            125 => Ok(Country::Liberia),
            126 => Ok(Country::Libya),
            127 => Ok(Country::Liechtenstein),
            128 => Ok(Country::Luxembourg),
            129 => Ok(Country::Madagascar),
            130 => Ok(Country::Macau),
            131 => Ok(Country::Malawi),
            132 => Ok(Country::Malaysia),
            133 => Ok(Country::Maldives),
            134 => Ok(Country::Mali),
            135 => Ok(Country::Malta),
            136 => Ok(Country::ManIsle),
            137 => Ok(Country::MarshallIslands),
            138 => Ok(Country::Martinique),
            139 => Ok(Country::Mauritania),
            140 => Ok(Country::Mauritius),
            141 => Ok(Country::Mayotte),
            142 => Ok(Country::Mexico),
            143 => Ok(Country::Micronesia),
            144 => Ok(Country::Monaco),
            145 => Ok(Country::Mongolia),
            146 => Ok(Country::Montserrat),
            147 => Ok(Country::Morocco),
            148 => Ok(Country::Mozambique),
            149 => Ok(Country::Namibia),
            150 => Ok(Country::Nauru),
            151 => Ok(Country::NavassaIsland),
            152 => Ok(Country::Nepal),
            153 => Ok(Country::Netherlands),
            154 => Ok(Country::NetherlandsAntilles),
            155 => Ok(Country::NewCaledonia),
            156 => Ok(Country::NewZealand),
            157 => Ok(Country::Nicaragua),
            158 => Ok(Country::Niger),
            159 => Ok(Country::Nigeria),
            160 => Ok(Country::Niue),
            161 => Ok(Country::NorfolkIsland),
            162 => Ok(Country::NorthernMarianaIslands),
            163 => Ok(Country::Norway),
            164 => Ok(Country::Oman),
            165 => Ok(Country::Pakistan),
            166 => Ok(Country::PalmyraAtoll),
            168 => Ok(Country::Panama),
            169 => Ok(Country::PapuaNewGuinea),
            170 => Ok(Country::ParacelIslands),
            171 => Ok(Country::Paraguay),
            172 => Ok(Country::Peru),
            173 => Ok(Country::Philippines),
            174 => Ok(Country::PitcairnIslands),
            175 => Ok(Country::Poland),
            176 => Ok(Country::Portugal),
            177 => Ok(Country::PuertoRico),
            178 => Ok(Country::Qatar),
            179 => Ok(Country::Reunion),
            180 => Ok(Country::Romania),
            181 => Ok(Country::Rwanda),
            182 => Ok(Country::StKittsAndNevis),
            183 => Ok(Country::StHelena),
            184 => Ok(Country::StLucia),
            185 => Ok(Country::StPierreAndMiquelon),
            186 => Ok(Country::StVincentAndTheGrenadines),
            187 => Ok(Country::SanMarino),
            188 => Ok(Country::SaoTomeAndPrincipe),
            189 => Ok(Country::SaudiArabia),
            190 => Ok(Country::Senegal),
            191 => Ok(Country::Seychelles),
            192 => Ok(Country::SierraLeone),
            193 => Ok(Country::Singapore),
            194 => Ok(Country::SolomonIslands),
            195 => Ok(Country::Somalia),
            196 => Ok(Country::SouthGeorgiaSouthSandwichIslands),
            197 => Ok(Country::SouthAfrica),
            198 => Ok(Country::Spain),
            199 => Ok(Country::SpratlyIslands),
            200 => Ok(Country::SriLanka),
            201 => Ok(Country::Sudan),
            202 => Ok(Country::Suriname),
            203 => Ok(Country::Svalbard_Norway),
            204 => Ok(Country::Swaziland),
            205 => Ok(Country::Sweden),
            206 => Ok(Country::Switzerland),
            207 => Ok(Country::Syria),
            208 => Ok(Country::Taiwan),
            209 => Ok(Country::Tanzania),
            210 => Ok(Country::Thailand),
            211 => Ok(Country::Togo),
            212 => Ok(Country::Tokelau),
            213 => Ok(Country::Tonga),
            214 => Ok(Country::TrinidadAndTobago),
            215 => Ok(Country::TromelinIsland),
            216 => Ok(Country::PacificIslands),
            217 => Ok(Country::Tunisia),
            218 => Ok(Country::Turkey),
            219 => Ok(Country::TurksCaicosIslands),
            220 => Ok(Country::Tuvalu),
            221 => Ok(Country::Uganda),
            222 => Ok(Country::CommonwealthOfIndependentStates),
            223 => Ok(Country::UnitedArabEmirates),
            224 => Ok(Country::UnitedKingdom),
            225 => Ok(Country::UnitedStates),
            226 => Ok(Country::Uruguay),
            227 => Ok(Country::Vanuatu),
            228 => Ok(Country::VaticanCity),
            229 => Ok(Country::Venezuela),
            230 => Ok(Country::Vietnam),
            231 => Ok(Country::VirginIslands),
            232 => Ok(Country::WakeIsland),
            233 => Ok(Country::WallisFutuna),
            234 => Ok(Country::WesternSahara),
            235 => Ok(Country::WestBank),
            236 => Ok(Country::WesternSamoa),
            237 => Ok(Country::Yemen),
            241 => Ok(Country::Zaire),
            242 => Ok(Country::Zambia),
            243 => Ok(Country::Zimbabwe),
            244 => Ok(Country::Armenia),
            245 => Ok(Country::Azerbaijan),
            246 => Ok(Country::Belarus),
            247 => Ok(Country::BosniaHercegovina),
            248 => Ok(Country::ClippertonIsland),
            249 => Ok(Country::Croatia),
            250 => Ok(Country::Estonia),
            251 => Ok(Country::Georgia),
            252 => Ok(Country::Kazakhstan),
            253 => Ok(Country::Kyrgyzstan),
            254 => Ok(Country::Latvia),
            255 => Ok(Country::Lithuania),
            256 => Ok(Country::Macedonia),
            257 => Ok(Country::MidwayIslands),
            258 => Ok(Country::Moldova),
            259 => Ok(Country::Montenegro),
            260 => Ok(Country::Russia),
            261 => Ok(Country::SerbiaMontenegro),
            262 => Ok(Country::Slovenia),
            263 => Ok(Country::Tajikistan),
            264 => Ok(Country::Turkmenistan),
            265 => Ok(Country::Ukraine),
            266 => Ok(Country::Uzbekistan),
            n => Err(DisError::InvalidEnumValue(266, n as usize)),
        }
    }
}

pub struct VectorF32 {
    first_vector_component : f32,
    second_vector_component : f32,
    third_vector_component : f32,
}

pub struct Location {
    x_coordinate : f64,
    y_coordinate : f64,
    z_coordinate : f64,
}

// TODO alias to vectorf32?
pub struct Orientation {
    psi : f32,
    theta : f32,
    phi : f32,
}

pub struct Appearance {
    general_appearance : GeneralAppearance,
    specific_appearance : SpecificAppearance,
}

pub struct GeneralAppearance {
    entity_paint_scheme : EntityPaintScheme, // enum
    entity_mobility_kill : EntityMobilityKill, // enum
    entity_fire_power : EntityFirePower, // enum
    entity_damage : EntityDamage, // enum
    entity_smoke : EntitySmoke, // enum
    entity_trailing_effect : EntityTrailingEffect, // enum
    entity_hatch_state : EntityHatchState, // enum
    entity_lights : EntityLights, // enum
    entity_flaming_effect : EntityFlamingEffect, // enum
}

pub enum EntityPaintScheme {
    UniformColor = 0,
    Camouflage = 1,
}

impl TryFrom<u8> for EntityPaintScheme {
    type Error = DisError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(EntityPaintScheme::UniformColor),
            1 => Ok(EntityPaintScheme::Camouflage),
            n => Err(DisError::InvalidEnumValue(1, n as usize)),
        }
    }
}

pub enum EntityMobilityKill {
    NoMobilityKill = 0,
    MobilityKill = 1,
}

impl TryFrom<u8> for EntityMobilityKill {
    type Error = DisError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(EntityMobilityKill::NoMobilityKill),
            1 => Ok(EntityMobilityKill::MobilityKill),
            n => Err(DisError::InvalidEnumValue(1, n as usize)),
        }
    }
}

pub enum EntityFirePower {
    NoFirePowerKill = 0,
    FirePowerKill = 1,
}

impl TryFrom<u8> for EntityFirePower {
    type Error = DisError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(EntityFirePower::NoFirePowerKill),
            1 => Ok(EntityFirePower::FirePowerKill),
            n => Err(DisError::InvalidEnumValue(1, n as usize)),
        }
    }
}

pub enum EntityDamage {
    NoDamage = 0,
    SlightDamage = 1,
    ModerateDamage = 2,
    Destroyed = 3,
}

impl TryFrom<u8> for EntityDamage {
    type Error = DisError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(EntityDamage::NoDamage),
            1 => Ok(EntityDamage::SlightDamage),
            2 => Ok(EntityDamage::ModerateDamage),
            3 => Ok(EntityDamage::Destroyed),
            n => Err(DisError::InvalidEnumValue(3, n as usize)),
        }
    }
}

pub enum EntitySmoke {
    NotSmoking = 0,
    SmokePlumeRising = 1,
    EmittingEngineSmoke = 2,
    EmittingEngineSmokeAndSmokePlumeRising = 3,
}

impl TryFrom<u8> for EntitySmoke {
    type Error = DisError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(EntitySmoke::NotSmoking),
            1 => Ok(EntitySmoke::SmokePlumeRising),
            2 => Ok(EntitySmoke::EmittingEngineSmoke),
            3 => Ok(EntitySmoke::EmittingEngineSmokeAndSmokePlumeRising),
            n => Err(DisError::InvalidEnumValue(3, n as usize)),
        }
    }
}

pub enum EntityTrailingEffect {
    None = 0,
    Small = 1,
    Medium = 2,
    Large = 3,
}

impl TryFrom<u8> for EntityTrailingEffect {
    type Error = DisError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(EntityTrailingEffect::None),
            1 => Ok(EntityTrailingEffect::Small),
            2 => Ok(EntityTrailingEffect::Medium),
            3 => Ok(EntityTrailingEffect::Large),
            n => Err(DisError::InvalidEnumValue(3, n as usize)),
        }
    }
}

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

impl TryFrom<u8> for EntityHatchState {
    type Error = DisError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(EntityHatchState::NotApplicable),
            1 => Ok(EntityHatchState::Closed),
            2 => Ok(EntityHatchState::Popped),
            3 => Ok(EntityHatchState::PoppedAndPersonVisible),
            4 => Ok(EntityHatchState::Open),
            5 => Ok(EntityHatchState::OpenAndPersonVisible),
            6 => Ok(EntityHatchState::Unused1),
            7 => Ok(EntityHatchState::Unused2),
            n => Err(DisError::InvalidEnumValue(7, n as usize)),
        }
    }
}

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

impl TryFrom<u8> for EntityLights {
    type Error = DisError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(EntityLights::None),
            1 => Ok(EntityLights::RunningLightsOn),
            2 => Ok(EntityLights::NavigationLightsOn),
            3 => Ok(EntityLights::FromationLightsOn),
            4 => Ok(EntityLights::Unused1),
            5 => Ok(EntityLights::Unused2),
            6 => Ok(EntityLights::Unused3),
            7 => Ok(EntityLights::Unused4),
            n => Err(DisError::InvalidEnumValue(7, n as usize)),
        }
    }
}

pub enum EntityFlamingEffect {
    None = 0,
    FlamesPresent = 1,
}

impl TryFrom<u8> for EntityFlamingEffect {
    type Error = DisError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(EntityFlamingEffect::None),
            1 => Ok(EntityFlamingEffect::FlamesPresent),
            n => Err(DisError::InvalidEnumValue(1, n as usize)),
        }
    }
}

// TODO replace u16 with specific types for the variants; should be a struct?
pub enum SpecificAppearance {
    LandPlatform(u16),
    AirPlatform(u16),
    SurfacePlatform(u16),
    SubsurfacePlatform(u16),
    SpacePlatform(u16),
    GuidedMunition(u16),
    LifeForm(u16),
    Environmental(u16),
}

pub struct DrParameters {
    algorithm : DrAlgorithm,
    other_parameters : DrOtherParameters,
    linear_acceleration : VectorF32,
    angular_velocity : VectorF32,
}

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

impl TryFrom<u8> for DrAlgorithm {
    type Error = DisError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(DrAlgorithm::Other),
            1 => Ok(DrAlgorithm::Static),
            2 => Ok(DrAlgorithm::DrmFPW),
            3 => Ok(DrAlgorithm::DrmRPW),
            4 => Ok(DrAlgorithm::DrmRVW),
            5 => Ok(DrAlgorithm::DrmFVW),
            6 => Ok(DrAlgorithm::DrmFPB),
            7 => Ok(DrAlgorithm::DrmRPB),
            8 => Ok(DrAlgorithm::DrmRVB),
            9 => Ok(DrAlgorithm::DrmFVB),
            n => Err(DisError::InvalidEnumValue(9, n as usize)),
        }
    }
}

pub struct DrOtherParameters {
    // 120-bits padding
}

pub struct EntityMarking {
    marking_character_set : EntityMarkingCharacterSet,
    marking_string : [u8; 11], // 11 byte String
}

pub enum EntityMarkingCharacterSet {
    Unused = 0,
    ASCII = 1,
    ArmyMarking = 2,
    DigitChevron = 3,
}

impl TryFrom<u8> for EntityMarkingCharacterSet {
    type Error = DisError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(EntityMarkingCharacterSet::Unused),
            1 => Ok(EntityMarkingCharacterSet::ASCII),
            2 => Ok(EntityMarkingCharacterSet::ArmyMarking),
            3 => Ok(EntityMarkingCharacterSet::DigitChevron),
            n => Err(DisError::InvalidEnumValue(3, n as usize)),
        }
    }
}

pub struct EntityCapabilities {
    ammunition_supply : bool,
    fuel_supply : bool,
    recovery : bool,
    repair : bool,
    // 28-bits padding
}

pub struct ArticulationParameter {
    parameter_type_designator : ApTypeDesignator,
    parameter_change_indicator : u8,
    articulation_attachment_ic : u16,
    parameter_type_variant : ParameterTypeVariant,
    articulation_parameter_value : f64,
}

pub enum ApTypeDesignator {
    Articulated = 0,
    Attached = 1,
}

impl TryFrom<u8> for ApTypeDesignator {
    type Error = DisError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ApTypeDesignator::Articulated),
            1 => Ok(ApTypeDesignator::Attached),
            n => Err(DisError::InvalidEnumValue(1, n as usize)),
        }
    }
}

pub struct ParameterTypeVariant {
    attached_parts : u32,
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
    articulated_parts : ArticulatedParts,
}

pub struct ArticulatedParts {
    low_bits : ApLowBits,
    high_bits : ApHighBits,
}

pub enum ApLowBits {
    Position = 1,
    ZRate = 10,
    Azimuth = 11,
    AzimuthRate = 12,
    Elevation = 13,
    ElevationRate = 14,
    Rotation = 15,
    RotationRate = 16,
    PositionRate = 2,
    Extension = 3,
    ExtensionRate = 4,
    X = 5,
    XRate = 6,
    Y = 7,
    YRate = 8,
    Z = 9,
}

impl TryFrom<u16> for ApLowBits {
    type Error = DisError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(ApLowBits::Position),
            2 => Ok(ApLowBits::PositionRate),
            3 => Ok(ApLowBits::Extension),
            4 => Ok(ApLowBits::ExtensionRate),
            5 => Ok(ApLowBits::X),
            6 => Ok(ApLowBits::XRate),
            7 => Ok(ApLowBits::Y),
            8 => Ok(ApLowBits::YRate),
            9 => Ok(ApLowBits::Z),
            10 => Ok(ApLowBits::ZRate),
            11 => Ok(ApLowBits::Azimuth),
            12 => Ok(ApLowBits::AzimuthRate),
            13 => Ok(ApLowBits::Elevation),
            14 => Ok(ApLowBits::ElevationRate),
            15 => Ok(ApLowBits::Rotation),
            16 => Ok(ApLowBits::RotationRate),
            n => Err(DisError::InvalidEnumValue(16, n as usize)),
        }
    }
}

pub enum ApHighBits {
    Placeholder = 0,
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
}

impl EntityState {
    pub fn builder() -> EntityStateBuilder {
        EntityStateBuilder::new()
    }

    // pub fn serialize() ->
}

// impl TryFrom<&BytesMut> for EntityState {
//     type Error = DisError;
//
//     fn try_from(buf: &BytesMut) -> Result<Self, Self::Error> {
//         EntityState::try_from(&buf[..])
//     }
// }
//
// impl TryFrom<&[u8]> for EntityState {
//     type Error = DisError;
//
//     fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
//         todo!()
//     }
// }