use crate::dis::v6::entity_state::builder::{AirPlatformBuilder, EnvironmentalBuilder, GeneralAppearanceBuilder, GuidedMunitionBuilder, LandPlatformBuilder, LifeFormBuilder, SpacePlatformBuilder, SubsurfacePlatformBuilder, SurfacePlatformBuilder};
use crate::dis::v6::model::PduHeader;
use super::builder::EntityStateBuilder;

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
    pub simulation_address : SimulationAddress,
    pub entity_id : u16
}

pub struct SimulationAddress {
    pub site_id : u16,
    pub application_id : u16,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ForceId {
    Other = 0,
    Friendly = 1,
    Opposing = 2,
    Neutral = 3,
}

impl From<u8> for ForceId {
    fn from(value: u8) -> Self {
        match value {
            0 => ForceId::Other,
            1 => ForceId::Friendly,
            2 => ForceId::Opposing,
            3 => ForceId::Neutral,
            _unspecified_value => ForceId::Other,
        }
    }
}

impl From<ForceId> for u8 {
    fn from(value: ForceId) -> Self {
        match value {
            ForceId::Other => { 0u8 }
            ForceId::Friendly => { 1u8 }
            ForceId::Opposing => { 2u8 }
            ForceId::Neutral => { 3u8 }
        }
    }
}

// TODO Needed?
impl Default for ForceId {
    fn default() -> Self {
        ForceId::Neutral
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EntityType {
    pub kind : EntityKind,
    pub domain : u8,
    pub country : Country, // TODO u16 instead of big enum? Put codes and names in config file?
    pub category : u8,
    pub subcategory : u8,
    pub specific : u8,
    pub extra : u8,
}

#[derive(Copy, Clone, Debug, PartialEq)]
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

impl From<u8> for EntityKind {
    fn from(value: u8) -> Self {
        match value {
            0 => EntityKind::Other,
            1 => EntityKind::Platform,
            2 => EntityKind::Munition,
            3 => EntityKind::LifeForm,
            4 => EntityKind::Environmental,
            5 => EntityKind::CulturalFeature,
            6 => EntityKind::Supply,
            7 => EntityKind::Radio,
            8 => EntityKind::Expendable,
            9 => EntityKind::SensorEmitter,
            _unspecified_value => EntityKind::Other,
        }
    }
}

impl From<EntityKind> for u8 {
    fn from(value: EntityKind) -> Self {
        match value {
            EntityKind::Other => { 0u8 }
            EntityKind::Platform => { 1u8 }
            EntityKind::Munition => { 2u8 }
            EntityKind::LifeForm => { 3u8 }
            EntityKind::Environmental => { 4u8 }
            EntityKind::CulturalFeature => { 5u8 }
            EntityKind::Supply => { 6u8 }
            EntityKind::Radio => { 7u8 }
            EntityKind::Expendable => { 8u8 }
            EntityKind::SensorEmitter => { 9u8 }
        }
    }
}

// regex: (?<value>[0-9]*)[\t]+(?<field>[\w (),'-.]+)$
// replace: \t${field} = ${value}, | $2 = $1
#[derive(Copy, Clone, Debug, PartialEq)]
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

impl From<u16> for Country {
    fn from(value: u16) -> Self {
        match value {
            0 => Country::Other,
            1 => Country::Afghanistan,
            2 => Country::Albania,
            3 => Country::Algeria,
            4 => Country::AmericanSamoa,
            5 => Country::Andorra,
            6 => Country::Angola,
            7 => Country::Anguilla,
            8 => Country::Antarctica,
            9 => Country::AntiguaBarbuda,
            10 => Country::Argentina,
            11 => Country::Aruba,
            12 => Country::AshmoreCartierIslands,
            13 => Country::Australia,
            14 => Country::Austria,
            15 => Country::Bahamas,
            16 => Country::Bahrain,
            17 => Country::BakerIsland,
            18 => Country::Bangladesh,
            19 => Country::Barbados,
            20 => Country::BassasDaIndia,
            21 => Country::Belgium,
            22 => Country::Belize,
            23 => Country::Benin,
            24 => Country::Bermuda,
            25 => Country::Bhutan,
            26 => Country::Bolivia,
            27 => Country::Botswana,
            28 => Country::BouvetIsland,
            29 => Country::Brazil,
            30 => Country::BritishIndianOceanTerritory,
            31 => Country::BritishVirginIslands,
            32 => Country::Brunei,
            33 => Country::Bulgaria,
            34 => Country::Burkina,
            35 => Country::Burma,
            36 => Country::Burundi,
            37 => Country::Cambodia,
            38 => Country::Cameroon,
            39 => Country::Canada,
            40 => Country::CapeVerde,
            41 => Country::CaymanIslands,
            42 => Country::CentralAfricanRepublic,
            43 => Country::Chad,
            44 => Country::Chile,
            45 => Country::China,
            46 => Country::ChristmasIsland,
            47 => Country::CocosIslands,
            48 => Country::Colombia,
            49 => Country::Comoros,
            50 => Country::Congo,
            51 => Country::CookIslands,
            52 => Country::CoralSeaIslands,
            53 => Country::CostaRica,
            54 => Country::Cuba,
            55 => Country::Cyprus,
            56 => Country::Czechoslovakia,
            57 => Country::Denmark,
            58 => Country::Djibouti,
            59 => Country::Dominica,
            60 => Country::DominicanRepublic,
            61 => Country::Ecuador,
            62 => Country::Egypt,
            63 => Country::ElSalvador,
            64 => Country::EquatorialGuinea,
            65 => Country::Ethiopia,
            66 => Country::EuropaIsland,
            67 => Country::FalklandIslands,
            68 => Country::FaroeIslands,
            69 => Country::Fiji,
            70 => Country::Finland,
            71 => Country::France,
            72 => Country::FrenchGuiana,
            73 => Country::FrenchPolynesia,
            74 => Country::FrenchSouthernAntarcticIslands,
            75 => Country::Gabon,
            76 => Country::GambiaThe,
            77 => Country::GazaStrip,
            78 => Country::Germany,
            79 => Country::Ghana,
            80 => Country::Gibraltar,
            81 => Country::GloriosoIslands,
            82 => Country::Greece,
            83 => Country::Greenland,
            84 => Country::Grenada,
            85 => Country::Guadaloupe,
            86 => Country::Guam,
            87 => Country::Guatemala,
            88 => Country::Guernsey,
            89 => Country::Guinea,
            90 => Country::GuineaBissau,
            91 => Country::Guyana,
            92 => Country::Haiti,
            93 => Country::HeardIslandMcDonaldIslands,
            94 => Country::Honduras,
            95 => Country::HongKong,
            96 => Country::HowlandIsland,
            97 => Country::Hungary,
            98 => Country::Iceland,
            99 => Country::India,
            100 => Country::Indonesia,
            101 => Country::Iran,
            102 => Country::Iraq,
            104 => Country::Ireland,
            105 => Country::Israel,
            106 => Country::Italy,
            107 => Country::CoteDIvoire,
            108 => Country::Jamaica,
            109 => Country::JanMayen,
            110 => Country::Japan,
            111 => Country::JarvisIsland,
            112 => Country::Jersey,
            113 => Country::JohnstonAtoll,
            114 => Country::Jordan,
            115 => Country::JuanDeNovaIsland,
            116 => Country::Kenya,
            117 => Country::KingmanReef,
            118 => Country::Kiribati,
            119 => Country::KoreaNorth,
            120 => Country::KoreaSouth,
            121 => Country::Kuwait,
            122 => Country::Laos,
            123 => Country::Lebanon,
            124 => Country::Lesotho,
            125 => Country::Liberia,
            126 => Country::Libya,
            127 => Country::Liechtenstein,
            128 => Country::Luxembourg,
            129 => Country::Madagascar,
            130 => Country::Macau,
            131 => Country::Malawi,
            132 => Country::Malaysia,
            133 => Country::Maldives,
            134 => Country::Mali,
            135 => Country::Malta,
            136 => Country::ManIsle,
            137 => Country::MarshallIslands,
            138 => Country::Martinique,
            139 => Country::Mauritania,
            140 => Country::Mauritius,
            141 => Country::Mayotte,
            142 => Country::Mexico,
            143 => Country::Micronesia,
            144 => Country::Monaco,
            145 => Country::Mongolia,
            146 => Country::Montserrat,
            147 => Country::Morocco,
            148 => Country::Mozambique,
            149 => Country::Namibia,
            150 => Country::Nauru,
            151 => Country::NavassaIsland,
            152 => Country::Nepal,
            153 => Country::Netherlands,
            154 => Country::NetherlandsAntilles,
            155 => Country::NewCaledonia,
            156 => Country::NewZealand,
            157 => Country::Nicaragua,
            158 => Country::Niger,
            159 => Country::Nigeria,
            160 => Country::Niue,
            161 => Country::NorfolkIsland,
            162 => Country::NorthernMarianaIslands,
            163 => Country::Norway,
            164 => Country::Oman,
            165 => Country::Pakistan,
            166 => Country::PalmyraAtoll,
            168 => Country::Panama,
            169 => Country::PapuaNewGuinea,
            170 => Country::ParacelIslands,
            171 => Country::Paraguay,
            172 => Country::Peru,
            173 => Country::Philippines,
            174 => Country::PitcairnIslands,
            175 => Country::Poland,
            176 => Country::Portugal,
            177 => Country::PuertoRico,
            178 => Country::Qatar,
            179 => Country::Reunion,
            180 => Country::Romania,
            181 => Country::Rwanda,
            182 => Country::StKittsAndNevis,
            183 => Country::StHelena,
            184 => Country::StLucia,
            185 => Country::StPierreAndMiquelon,
            186 => Country::StVincentAndTheGrenadines,
            187 => Country::SanMarino,
            188 => Country::SaoTomeAndPrincipe,
            189 => Country::SaudiArabia,
            190 => Country::Senegal,
            191 => Country::Seychelles,
            192 => Country::SierraLeone,
            193 => Country::Singapore,
            194 => Country::SolomonIslands,
            195 => Country::Somalia,
            196 => Country::SouthGeorgiaSouthSandwichIslands,
            197 => Country::SouthAfrica,
            198 => Country::Spain,
            199 => Country::SpratlyIslands,
            200 => Country::SriLanka,
            201 => Country::Sudan,
            202 => Country::Suriname,
            203 => Country::SvalbardNorway,
            204 => Country::Swaziland,
            205 => Country::Sweden,
            206 => Country::Switzerland,
            207 => Country::Syria,
            208 => Country::Taiwan,
            209 => Country::Tanzania,
            210 => Country::Thailand,
            211 => Country::Togo,
            212 => Country::Tokelau,
            213 => Country::Tonga,
            214 => Country::TrinidadAndTobago,
            215 => Country::TromelinIsland,
            216 => Country::PacificIslands,
            217 => Country::Tunisia,
            218 => Country::Turkey,
            219 => Country::TurksCaicosIslands,
            220 => Country::Tuvalu,
            221 => Country::Uganda,
            222 => Country::CommonwealthOfIndependentStates,
            223 => Country::UnitedArabEmirates,
            224 => Country::UnitedKingdom,
            225 => Country::UnitedStates,
            226 => Country::Uruguay,
            227 => Country::Vanuatu,
            228 => Country::VaticanCity,
            229 => Country::Venezuela,
            230 => Country::Vietnam,
            231 => Country::VirginIslands,
            232 => Country::WakeIsland,
            233 => Country::WallisFutuna,
            234 => Country::WesternSahara,
            235 => Country::WestBank,
            236 => Country::WesternSamoa,
            237 => Country::Yemen,
            241 => Country::Zaire,
            242 => Country::Zambia,
            243 => Country::Zimbabwe,
            244 => Country::Armenia,
            245 => Country::Azerbaijan,
            246 => Country::Belarus,
            247 => Country::BosniaHercegovina,
            248 => Country::ClippertonIsland,
            249 => Country::Croatia,
            250 => Country::Estonia,
            251 => Country::Georgia,
            252 => Country::Kazakhstan,
            253 => Country::Kyrgyzstan,
            254 => Country::Latvia,
            255 => Country::Lithuania,
            256 => Country::Macedonia,
            257 => Country::MidwayIslands,
            258 => Country::Moldova,
            259 => Country::Montenegro,
            260 => Country::Russia,
            261 => Country::SerbiaMontenegro,
            262 => Country::Slovenia,
            263 => Country::Tajikistan,
            264 => Country::Turkmenistan,
            265 => Country::Ukraine,
            266 => Country::Uzbekistan,
            _unspecified_value => Country::Other,
        }
    }
}

impl From<Country> for u16 {
    fn from(value: Country) -> Self {
        match value {
            Country::Other => { 0u16 },
            Country::Afghanistan => { 1u16 },
            Country::Albania => { 2u16 },
            Country::Algeria => { 3u16 },
            Country::AmericanSamoa => { 4u16 },
            Country::Andorra => { 5u16 },
            Country::Angola => { 6u16 },
            Country::Anguilla => { 7u16 },
            Country::Antarctica => { 8u16 },
            Country::AntiguaBarbuda => { 9u16 },
            Country::Argentina => { 10u16 },
            Country::Aruba => { 11u16 },
            Country::AshmoreCartierIslands => { 12u16 },
            Country::Australia => { 13u16 },
            Country::Austria => { 14u16 },
            Country::Bahamas => { 15u16 },
            Country::Bahrain => { 16u16 },
            Country::BakerIsland => { 17u16 },
            Country::Bangladesh => { 18u16 },
            Country::Barbados => { 19u16 },
            Country::BassasDaIndia => { 20u16 },
            Country::Belgium => { 21u16 },
            Country::Belize => { 22u16 },
            Country::Benin => { 23u16 },
            Country::Bermuda => { 24u16 },
            Country::Bhutan => { 25u16 },
            Country::Bolivia => { 26u16 },
            Country::Botswana => { 27u16 },
            Country::BouvetIsland => { 28u16 },
            Country::Brazil => { 29u16 },
            Country::BritishIndianOceanTerritory => { 30u16 },
            Country::BritishVirginIslands => { 31u16 },
            Country::Brunei => { 32u16 },
            Country::Bulgaria => { 33u16 },
            Country::Burkina => { 34u16 },
            Country::Burma => { 35u16 },
            Country::Burundi => { 36u16 },
            Country::Cambodia => { 37u16 },
            Country::Cameroon => { 38u16 },
            Country::Canada => { 39u16 },
            Country::CapeVerde => { 40u16 },
            Country::CaymanIslands => { 41u16 },
            Country::CentralAfricanRepublic => { 42u16 },
            Country::Chad => { 43u16 },
            Country::Chile => { 44u16 },
            Country::China => { 45u16 },
            Country::ChristmasIsland => { 46u16 },
            Country::CocosIslands => { 47u16 },
            Country::Colombia => { 48u16 },
            Country::Comoros => { 49u16 },
            Country::Congo => { 50u16 },
            Country::CookIslands => { 51u16 },
            Country::CoralSeaIslands => { 52u16 },
            Country::CostaRica => { 53u16 },
            Country::Cuba => { 54u16 },
            Country::Cyprus => { 55u16 },
            Country::Czechoslovakia => { 56u16 },
            Country::Denmark => { 57u16 },
            Country::Djibouti => { 58u16 },
            Country::Dominica => { 59u16 },
            Country::DominicanRepublic => { 60u16 },
            Country::Ecuador => { 61u16 },
            Country::Egypt => { 62u16 },
            Country::ElSalvador => { 63u16 },
            Country::EquatorialGuinea => { 64u16 },
            Country::Ethiopia => { 65u16 },
            Country::EuropaIsland => { 66u16 },
            Country::FalklandIslands => { 67u16 },
            Country::FaroeIslands => { 68u16 },
            Country::Fiji => { 69u16 },
            Country::Finland => { 70u16 },
            Country::France => { 71u16 },
            Country::FrenchGuiana => { 72u16 },
            Country::FrenchPolynesia => { 73u16 },
            Country::FrenchSouthernAntarcticIslands => { 74u16 },
            Country::Gabon => { 75u16 },
            Country::GambiaThe => { 76u16 },
            Country::GazaStrip => { 77u16 },
            Country::Germany => { 78u16 },
            Country::Ghana => { 79u16 },
            Country::Gibraltar => { 80u16 },
            Country::GloriosoIslands => { 81u16 },
            Country::Greece => { 82u16 },
            Country::Greenland => { 83u16 },
            Country::Grenada => { 84u16 },
            Country::Guadaloupe => { 85u16 },
            Country::Guam => { 86u16 },
            Country::Guatemala => { 87u16 },
            Country::Guernsey => { 88u16 },
            Country::Guinea => { 89u16 },
            Country::GuineaBissau => { 90u16 },
            Country::Guyana => { 91u16 },
            Country::Haiti => { 92u16 },
            Country::HeardIslandMcDonaldIslands => { 93u16 },
            Country::Honduras => { 94u16 },
            Country::HongKong => { 95u16 },
            Country::HowlandIsland => { 96u16 },
            Country::Hungary => { 97u16 },
            Country::Iceland => { 98u16 },
            Country::India => { 99u16 },
            Country::Indonesia => { 100u16 },
            Country::Iran => { 101u16 },
            Country::Iraq => { 102u16 },
            Country::Ireland => { 104u16 },
            Country::Israel => { 105u16 },
            Country::Italy => { 106u16 },
            Country::CoteDIvoire => { 107u16 },
            Country::Jamaica => { 108u16 },
            Country::JanMayen => { 109u16 },
            Country::Japan => { 110u16 },
            Country::JarvisIsland => { 111u16 },
            Country::Jersey => { 112u16 },
            Country::JohnstonAtoll => { 113u16 },
            Country::Jordan => { 114u16 },
            Country::JuanDeNovaIsland => { 115u16 },
            Country::Kenya => { 116u16 },
            Country::KingmanReef => { 117u16 },
            Country::Kiribati => { 118u16 },
            Country::KoreaNorth => { 119u16 },
            Country::KoreaSouth => { 120u16 },
            Country::Kuwait => { 121u16 },
            Country::Laos => { 122u16 },
            Country::Lebanon => { 123u16 },
            Country::Lesotho => { 124u16 },
            Country::Liberia => { 125u16 },
            Country::Libya => { 126u16 },
            Country::Liechtenstein => { 127u16 },
            Country::Luxembourg => { 128u16 },
            Country::Madagascar => { 129u16 },
            Country::Macau => { 130u16 },
            Country::Malawi => { 131u16 },
            Country::Malaysia => { 132u16 },
            Country::Maldives => { 133u16 },
            Country::Mali => { 134u16 },
            Country::Malta => { 135u16 },
            Country::ManIsle => { 136u16 },
            Country::MarshallIslands => { 137u16 },
            Country::Martinique => { 138u16 },
            Country::Mauritania => { 139u16 },
            Country::Mauritius => { 140u16 },
            Country::Mayotte => { 141u16 },
            Country::Mexico => { 142u16 },
            Country::Micronesia => { 143u16 },
            Country::Monaco => { 144u16 },
            Country::Mongolia => { 145u16 },
            Country::Montserrat => { 146u16 },
            Country::Morocco => { 147u16 },
            Country::Mozambique => { 148u16 },
            Country::Namibia => { 149u16 },
            Country::Nauru => { 150u16 },
            Country::NavassaIsland => { 151u16 },
            Country::Nepal => { 152u16 },
            Country::Netherlands => { 153u16 },
            Country::NetherlandsAntilles => { 154u16 },
            Country::NewCaledonia => { 155u16 },
            Country::NewZealand => { 156u16 },
            Country::Nicaragua => { 157u16 },
            Country::Niger => { 158u16 },
            Country::Nigeria => { 159u16 },
            Country::Niue => { 160u16 },
            Country::NorfolkIsland => { 161u16 },
            Country::NorthernMarianaIslands => { 162u16 },
            Country::Norway => { 163u16 },
            Country::Oman => { 164u16 },
            Country::Pakistan => { 165u16 },
            Country::PalmyraAtoll => { 166u16 },
            Country::Panama => { 168u16 },
            Country::PapuaNewGuinea => { 169u16 },
            Country::ParacelIslands => { 170u16 },
            Country::Paraguay => { 171u16 },
            Country::Peru => { 172u16 },
            Country::Philippines => { 173u16 },
            Country::PitcairnIslands => { 174u16 },
            Country::Poland => { 175u16 },
            Country::Portugal => { 176u16 },
            Country::PuertoRico => { 177u16 },
            Country::Qatar => { 178u16 },
            Country::Reunion => { 179u16 },
            Country::Romania => { 180u16 },
            Country::Rwanda => { 181u16 },
            Country::StKittsAndNevis => { 182u16 },
            Country::StHelena => { 183u16 },
            Country::StLucia => { 184u16 },
            Country::StPierreAndMiquelon => { 185u16 },
            Country::StVincentAndTheGrenadines => { 186u16 },
            Country::SanMarino => { 187u16 },
            Country::SaoTomeAndPrincipe => { 188u16 },
            Country::SaudiArabia => { 189u16 },
            Country::Senegal => { 190u16 },
            Country::Seychelles => { 191u16 },
            Country::SierraLeone => { 192u16 },
            Country::Singapore => { 193u16 },
            Country::SolomonIslands => { 194u16 },
            Country::Somalia => { 195u16 },
            Country::SouthGeorgiaSouthSandwichIslands => { 196u16 },
            Country::SouthAfrica => { 197u16 },
            Country::Spain => { 198u16 },
            Country::SpratlyIslands => { 199u16 },
            Country::SriLanka => { 200u16 },
            Country::Sudan => { 201u16 },
            Country::Suriname => { 202u16 },
            Country::SvalbardNorway => { 203u16 },
            Country::Swaziland => { 204u16 },
            Country::Sweden => { 205u16 },
            Country::Switzerland => { 206u16 },
            Country::Syria => { 207u16 },
            Country::Taiwan => { 208u16 },
            Country::Tanzania => { 209u16 },
            Country::Thailand => { 210u16 },
            Country::Togo => { 211u16 },
            Country::Tokelau => { 212u16 },
            Country::Tonga => { 213u16 },
            Country::TrinidadAndTobago => { 214u16 },
            Country::TromelinIsland => { 215u16 },
            Country::PacificIslands => { 216u16 },
            Country::Tunisia => { 217u16 },
            Country::Turkey => { 218u16 },
            Country::TurksCaicosIslands => { 219u16 },
            Country::Tuvalu => { 220u16 },
            Country::Uganda => { 221u16 },
            Country::CommonwealthOfIndependentStates => { 222u16 },
            Country::UnitedArabEmirates => { 223u16 },
            Country::UnitedKingdom => { 224u16 },
            Country::UnitedStates => { 225u16 },
            Country::Uruguay => { 226u16 },
            Country::Vanuatu => { 227u16 },
            Country::VaticanCity => { 228u16 },
            Country::Venezuela => { 229u16 },
            Country::Vietnam => { 230u16 },
            Country::VirginIslands => { 231u16 },
            Country::WakeIsland => { 232u16 },
            Country::WallisFutuna => { 233u16 },
            Country::WesternSahara => { 234u16 },
            Country::WestBank => { 235u16 },
            Country::WesternSamoa => { 236u16 },
            Country::Yemen => { 237u16 },
            Country::Zaire => { 241u16 },
            Country::Zambia => { 242u16 },
            Country::Zimbabwe => { 243u16 },
            Country::Armenia => { 244u16 },
            Country::Azerbaijan => { 245u16 },
            Country::Belarus => { 246u16 },
            Country::BosniaHercegovina => { 247u16 },
            Country::ClippertonIsland => { 248u16 },
            Country::Croatia => { 249u16 },
            Country::Estonia => { 250u16 },
            Country::Georgia => { 251u16 },
            Country::Kazakhstan => { 252u16 },
            Country::Kyrgyzstan => { 253u16 },
            Country::Latvia => { 254u16 },
            Country::Lithuania => { 255u16 },
            Country::Macedonia => { 256u16 },
            Country::MidwayIslands => { 257u16 },
            Country::Moldova => { 258u16 },
            Country::Montenegro => { 259u16 },
            Country::Russia => { 260u16 },
            Country::SerbiaMontenegro => { 261u16 },
            Country::Slovenia => { 262u16 },
            Country::Tajikistan => { 263u16 },
            Country::Turkmenistan => { 264u16 },
            Country::Ukraine => { 265u16 },
            Country::Uzbekistan => { 266u16 },
        }
    }
}

// TODO to common/model
pub struct VectorF32 {
    pub first_vector_component : f32,
    pub second_vector_component : f32,
    pub third_vector_component : f32,
}

// TODO to common/model
pub struct Location {
    pub x_coordinate : f64,
    pub y_coordinate : f64,
    pub z_coordinate : f64,
}

// TODO to common/model
// TODO alias to vectorf32?
pub struct Orientation {
    pub psi : f32,
    pub theta : f32,
    pub phi : f32,
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

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum EntityPaintScheme {
    UniformColor = 0,
    Camouflage = 1,
}

impl From<u8> for EntityPaintScheme {
    fn from(value: u8) -> Self {
        match value {
            1 => EntityPaintScheme::Camouflage,
            0 | _ => EntityPaintScheme::UniformColor,
        }
    }
}

impl From<EntityPaintScheme> for u16 {
    fn from(value: EntityPaintScheme) -> Self {
        match value {
            EntityPaintScheme::UniformColor => { 0u16 }
            EntityPaintScheme::Camouflage => { 1u16 }
        }
    }
}

impl Default for EntityPaintScheme {
    fn default() -> Self {
        EntityPaintScheme::UniformColor
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum EntityMobilityKill {
    NoMobilityKill = 0,
    MobilityKill = 1,
}

impl From<u8> for EntityMobilityKill {
    fn from(value: u8) -> Self {
        match value {
            1 => EntityMobilityKill::MobilityKill,
            0 | _ => EntityMobilityKill::NoMobilityKill,
        }
    }
}

impl From<EntityMobilityKill> for u16 {
    fn from(value: EntityMobilityKill) -> Self {
        match value {
            EntityMobilityKill::NoMobilityKill => { 0u16 }
            EntityMobilityKill::MobilityKill => { 1u16 }
        }
    }
}

impl Default for EntityMobilityKill {
    fn default() -> Self {
        EntityMobilityKill::NoMobilityKill
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum EntityFirePower {
    NoFirePowerKill = 0,
    FirePowerKill = 1,
}

impl From<u8> for EntityFirePower {
    fn from(value: u8) -> Self {
        match value {
            1 => EntityFirePower::FirePowerKill,
            0 | _ => EntityFirePower::NoFirePowerKill,
        }
    }
}

impl From<EntityFirePower> for u16 {
    fn from(value: EntityFirePower) -> Self {
        match value {
            EntityFirePower::NoFirePowerKill => { 0u16 }
            EntityFirePower::FirePowerKill => { 1u16 }
        }
    }
}

impl Default for EntityFirePower {
    fn default() -> Self {
        EntityFirePower::NoFirePowerKill
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum EntityDamage {
    NoDamage = 0,
    SlightDamage = 1,
    ModerateDamage = 2,
    Destroyed = 3,
}

impl From<u8> for EntityDamage {
    fn from(value: u8) -> Self {
        match value {
            0 => EntityDamage::NoDamage,
            1 => EntityDamage::SlightDamage,
            2 => EntityDamage::ModerateDamage,
            3 => EntityDamage::Destroyed,
            _unspecified_value => EntityDamage::NoDamage,
        }
    }
}

impl From<EntityDamage> for u16 {
    fn from(value: EntityDamage) -> Self {
        match value {
            EntityDamage::NoDamage => { 0u16 }
            EntityDamage::SlightDamage => { 1u16  }
            EntityDamage::ModerateDamage => { 2u16 }
            EntityDamage::Destroyed => { 3u16 }
        }
    }
}

impl Default for EntityDamage {
    fn default() -> Self {
        EntityDamage::NoDamage
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum EntitySmoke {
    NotSmoking = 0,
    SmokePlumeRising = 1,
    EmittingEngineSmoke = 2,
    EmittingEngineSmokeAndSmokePlumeRising = 3,
}

impl From<u8> for EntitySmoke {
    fn from(value: u8) -> Self {
        match value {
            0 => EntitySmoke::NotSmoking,
            1 => EntitySmoke::SmokePlumeRising,
            2 => EntitySmoke::EmittingEngineSmoke,
            3 => EntitySmoke::EmittingEngineSmokeAndSmokePlumeRising,
            _unspecified_value => EntitySmoke::NotSmoking,
        }
    }
}

impl From<EntitySmoke> for u16 {
    fn from(value: EntitySmoke) -> Self {
        match value {
            EntitySmoke::NotSmoking => { 0u16 }
            EntitySmoke::SmokePlumeRising => { 1u16 }
            EntitySmoke::EmittingEngineSmoke => { 2u16 }
            EntitySmoke::EmittingEngineSmokeAndSmokePlumeRising => { 3u16 }
        }
    }
}

impl Default for EntitySmoke {
    fn default() -> Self {
        EntitySmoke::NotSmoking
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum EntityTrailingEffect {
    None = 0,
    Small = 1,
    Medium = 2,
    Large = 3,
}

impl From<u8> for EntityTrailingEffect {
    fn from(value: u8) -> Self {
        match value {
            0 => EntityTrailingEffect::None,
            1 => EntityTrailingEffect::Small,
            2 => EntityTrailingEffect::Medium,
            3 => EntityTrailingEffect::Large,
            _unspecified_value => EntityTrailingEffect::None,
        }
    }
}

impl From<EntityTrailingEffect> for u16 {
    fn from(value: EntityTrailingEffect) -> Self {
        match value {
            EntityTrailingEffect::None => { 0u16 }
            EntityTrailingEffect::Small => { 1u16 }
            EntityTrailingEffect::Medium => { 2u16 }
            EntityTrailingEffect::Large => { 3u16 }
        }
    }
}

impl Default for EntityTrailingEffect {
    fn default() -> Self {
        EntityTrailingEffect::None
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
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

impl From<u8> for EntityHatchState {
    fn from(value: u8) -> Self {
        match value {
            0 => EntityHatchState::NotApplicable,
            1 => EntityHatchState::Closed,
            2 => EntityHatchState::Popped,
            3 => EntityHatchState::PoppedAndPersonVisible,
            4 => EntityHatchState::Open,
            5 => EntityHatchState::OpenAndPersonVisible,
            6 => EntityHatchState::Unused1,
            7 => EntityHatchState::Unused2,
            _unspecified_value => EntityHatchState::NotApplicable,
        }
    }
}

impl From<EntityHatchState> for u16 {
    fn from(value: EntityHatchState) -> Self {
        match value {
            EntityHatchState::NotApplicable => { 0u16 }
            EntityHatchState::Closed => { 1u16 }
            EntityHatchState::Popped => { 2u16 }
            EntityHatchState::PoppedAndPersonVisible => { 3u16 }
            EntityHatchState::Open => { 4u16 }
            EntityHatchState::OpenAndPersonVisible => { 5u16 }
            EntityHatchState::Unused1 => { 6u16 }
            EntityHatchState::Unused2 => { 7u16 }
        }
    }
}

impl Default for EntityHatchState {
    fn default() -> Self {
        EntityHatchState::NotApplicable
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
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

impl From<u8> for EntityLights {
    fn from(value: u8) -> Self {
        match value {
            0 => EntityLights::None,
            1 => EntityLights::RunningLightsOn,
            2 => EntityLights::NavigationLightsOn,
            3 => EntityLights::FromationLightsOn,
            4 => EntityLights::Unused1,
            5 => EntityLights::Unused2,
            6 => EntityLights::Unused3,
            7 => EntityLights::Unused4,
            _unspecified_value => EntityLights::None,
        }
    }
}

impl From<EntityLights> for u16 {
    fn from(value: EntityLights) -> Self {
        match value {
            EntityLights::None => { 0u16 }
            EntityLights::RunningLightsOn => { 1u16 }
            EntityLights::NavigationLightsOn => { 2u16 }
            EntityLights::FromationLightsOn => { 3u16 }
            EntityLights::Unused1 => { 4u16 }
            EntityLights::Unused2 => { 5u16 }
            EntityLights::Unused3 => { 6u16 }
            EntityLights::Unused4 => { 7u16 }
        }
    }
}

impl Default for EntityLights {
    fn default() -> Self {
        EntityLights::None
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum EntityFlamingEffect {
    None = 0,
    FlamesPresent = 1,
}

impl From<u8> for EntityFlamingEffect {
    fn from(value: u8) -> Self {
        match value {
            1 => EntityFlamingEffect::FlamesPresent,
            0 | _ => EntityFlamingEffect::None,
        }
    }
}

impl From<EntityFlamingEffect> for u16 {
    fn from(value: EntityFlamingEffect) -> Self {
        match value {
            EntityFlamingEffect::None => { 0u16 }
            EntityFlamingEffect::FlamesPresent => { 1u16 }
        }
    }
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

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Launcher {
    NotRaised,
    Raised,
}

impl From<u8> for Launcher {
    fn from(value: u8) -> Self {
        match value {
            1 => Launcher::Raised,
            0 | _ => Launcher::NotRaised,
        }
    }
}

impl From<Launcher> for u16 {
    fn from(value: Launcher) -> Self {
        match value {
            Launcher::NotRaised => { 0u16 }
            Launcher::Raised => { 1u16 }
        }
    }
}

impl Default for Launcher {
    fn default() -> Self {
        Launcher::NotRaised
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Camouflage {
    Desert,
    Winter,
    Forest,
    Unspecified(u8),
}

impl From<u8> for Camouflage {
    fn from(value: u8) -> Self {
        match value {
            0 => Camouflage::Desert,
            1 => Camouflage::Winter,
            2 => Camouflage::Forest,
            unspecified_value => Camouflage::Unspecified(unspecified_value),
        }
    }
}

impl From<Camouflage> for u16 {
    fn from(value: Camouflage) -> Self {
        match value {
            Camouflage::Desert => { 0u16 }
            Camouflage::Winter => { 1u16 }
            Camouflage::Forest => { 2u16 }
            Camouflage::Unspecified(value) => { value as u16 }
        }
    }
}


impl Default for Camouflage {
    fn default() -> Self {
        Camouflage::Desert
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Concealed {
    NotConcealed,
    Concealed,
}

impl From<u8> for Concealed {
    fn from(value: u8) -> Self {
        match value {
            1 => Concealed::Concealed,
            0 | _ => Concealed::NotConcealed,
        }
    }
}

impl From<Concealed> for u16 {
    fn from(value: Concealed) -> Self {
        match value {
            Concealed::NotConcealed => { 0u16 }
            Concealed::Concealed => { 1u16 }
        }
    }
}

impl Default for Concealed {
    fn default() -> Self {
        Concealed::NotConcealed
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FrozenStatus {
    NotFrozen,
    Frozen,
}

impl From<u8> for FrozenStatus {
    fn from(value: u8) -> Self {
        match value {
            1 => FrozenStatus::Frozen,
            0 | _ => FrozenStatus::NotFrozen,
        }
    }
}

impl From<FrozenStatus> for u16 {
    fn from(value: FrozenStatus) -> Self {
        match value {
            FrozenStatus::NotFrozen => { 0u16 }
            FrozenStatus::Frozen => { 1u16 }
        }
    }
}

impl Default for FrozenStatus {
    fn default() -> Self {
        FrozenStatus::NotFrozen
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PowerPlantStatus {
    Off,
    On,
}

impl From<u8> for PowerPlantStatus {
    fn from(value: u8) -> Self {
        match value {
            1 => PowerPlantStatus::On,
            0 | _ => PowerPlantStatus::Off,
        }
    }
}

impl From<PowerPlantStatus> for u16 {
    fn from(value: PowerPlantStatus) -> Self {
        match value {
            PowerPlantStatus::Off => { 0u16 }
            PowerPlantStatus::On => { 1u16 }
        }
    }
}

impl Default for PowerPlantStatus {
    fn default() -> Self {
        PowerPlantStatus::Off
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum State {
    Active,
    Deactivated,
}

impl From<u8> for State {
    fn from(value: u8) -> Self {
        match value {
            1 => State::Deactivated,
            0 | _ => State::Active,
        }
    }
}

impl From<State> for u16 {
    fn from(value: State) -> Self {
        match value {
            State::Active => { 0u16 }
            State::Deactivated => { 1u16 }
        }
    }
}

impl Default for State {
    fn default() -> Self {
        State::Active
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Tent {
    NotExtended,
    Extended,
}

impl From<u8> for Tent {
    fn from(value: u8) -> Self {
        match value {
            1 => Tent::Extended,
            0 | _ => Tent::NotExtended,
        }
    }
}

impl From<Tent> for u16 {
    fn from(value: Tent) -> Self {
        match value {
            Tent::NotExtended => { 0u16 }
            Tent::Extended => { 1u16 }
        }
    }
}

impl Default for Tent {
    fn default() -> Self {
        Tent::NotExtended
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Ramp {
    Up,
    Down,
}

impl From<u8> for Ramp {
    fn from(value: u8) -> Self {
        match value {
            1 => Ramp::Down,
            0 | _ => Ramp::Up,
        }
    }
}

impl From<Ramp> for u16 {
    fn from(value: Ramp) -> Self {
        match value {
            Ramp::Up => { 0u16 }
            Ramp::Down => { 1u16 }
        }
    }
}

impl Default for Ramp {
    fn default() -> Self {
        Ramp::Up
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Afterburner {
    NotOn,
    On,
}

impl From<u8> for Afterburner {
    fn from(value: u8) -> Self {
        match value {
            1 => Afterburner::On,
            0 | _ => Afterburner::NotOn,
        }
    }
}

impl From<Afterburner> for u16 {
    fn from(value: Afterburner) -> Self {
        match value {
            Afterburner::NotOn => { 0u16 }
            Afterburner::On => { 1u16 }
        }
    }
}

impl Default for Afterburner {
    fn default() -> Self {
        Afterburner::NotOn
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LaunchFlash {
    NotPresent,
    Present,
}

impl From<u8> for LaunchFlash {
    fn from(value: u8) -> Self {
        match value {
            1 => LaunchFlash::Present,
            0 | _ => LaunchFlash::NotPresent,
        }
    }
}

impl From<LaunchFlash> for u16 {
    fn from(value: LaunchFlash) -> Self {
        match value {
            LaunchFlash::NotPresent => { 0u16 }
            LaunchFlash::Present => { 1u16 }
        }
    }
}

impl Default for LaunchFlash {
    fn default() -> Self {
        LaunchFlash::NotPresent
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LifeFormsState {
    Null,
    UprightStandingStill,
    UprightWalking,
    UprightRunning,
    Kneeling,
    Prone,
    Crawling,
    Swimming,
    Parachuting,
    Jumping,
}

impl From<u8> for LifeFormsState {
    fn from(value: u8) -> Self {
        match value {
            1 => LifeFormsState::UprightStandingStill,
            2 => LifeFormsState::UprightWalking,
            3 => LifeFormsState::UprightRunning,
            4 => LifeFormsState::Kneeling,
            5 => LifeFormsState::Prone,
            6 => LifeFormsState::Crawling,
            7 => LifeFormsState::Swimming,
            8 => LifeFormsState::Parachuting,
            9 => LifeFormsState::Jumping,
            0 | _ => LifeFormsState::Null,
        }
    }
}

impl From<LifeFormsState> for u16 {
    fn from(value: LifeFormsState) -> Self {
        match value {
            LifeFormsState::Null => { 0u16 }
            LifeFormsState::UprightStandingStill => { 1u16 }
            LifeFormsState::UprightWalking => { 2u16 }
            LifeFormsState::UprightRunning => { 3u16 }
            LifeFormsState::Kneeling => { 4u16 }
            LifeFormsState::Prone => { 5u16 }
            LifeFormsState::Crawling => { 6u16 }
            LifeFormsState::Swimming => { 7u16 }
            LifeFormsState::Parachuting => { 8u16 }
            LifeFormsState::Jumping => { 9u16 }
        }
    }
}

impl Default for LifeFormsState {
    fn default() -> Self {
        LifeFormsState::UprightStandingStill
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ActivityState {
    Active,
    Deactivated,
}

impl From<u8> for ActivityState {
    fn from(value: u8) -> Self {
        match value {
            1 => ActivityState::Deactivated,
            0 | _ => ActivityState::Active,
        }
    }
}

impl From<ActivityState> for u16 {
    fn from(value: ActivityState) -> Self {
        match value {
            ActivityState::Active => { 0u16 }
            ActivityState::Deactivated => { 1u16 }
        }
    }
}

impl Default for ActivityState {
    fn default() -> Self {
        ActivityState::Active
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Weapon {
    NotPresent,
    Stowed,
    Deployed,
    FiringPosition,
}

impl From<u8> for Weapon {
    fn from(value: u8) -> Self {
        match value {
            1 => Weapon::Stowed,
            2 => Weapon::Deployed,
            3 => Weapon::FiringPosition,
            0 | _ => Weapon::NotPresent,
        }
    }
}

impl From<Weapon> for u16 {
    fn from(value: Weapon) -> Self {
        match value {
            Weapon::NotPresent => { 0u16 }
            Weapon::Stowed => { 1u16 }
            Weapon::Deployed => { 2u16 }
            Weapon::FiringPosition => { 3u16 }
        }
    }
}

impl Default for Weapon {
    fn default() -> Self {
        Weapon::NotPresent
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Density {
    Clear,
    Hazy,
    Dense,
    VeryDense,
    Opaque,
}

impl From<u8> for Density {
    fn from(value: u8) -> Self {
        match value {
            1 => Density::Hazy,
            2 => Density::Dense,
            3 => Density::VeryDense,
            4 => Density::Opaque,
            0 | _ => Density::Clear,
        }
    }
}

impl From<Density> for u16 {
    fn from(value: Density) -> Self {
        match value {
            Density::Clear => { 0u16 }
            Density::Hazy => { 1u16 }
            Density::Dense => { 2u16 }
            Density::VeryDense => { 3u16 }
            Density::Opaque => { 4u16 }
        }
    }
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

#[derive(Copy, Clone, Debug, PartialEq)]
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

impl From<u8> for DrAlgorithm {
    fn from(value: u8) -> Self {
        match value {
            0 => DrAlgorithm::Other,
            1 => DrAlgorithm::Static,
            2 => DrAlgorithm::DrmFPW,
            3 => DrAlgorithm::DrmRPW,
            4 => DrAlgorithm::DrmRVW,
            5 => DrAlgorithm::DrmFVW,
            6 => DrAlgorithm::DrmFPB,
            7 => DrAlgorithm::DrmRPB,
            8 => DrAlgorithm::DrmRVB,
            9 => DrAlgorithm::DrmFVB,
            _unspecified_value => DrAlgorithm::Other,
        }
    }
}

impl From<DrAlgorithm> for u8 {
    fn from(value: DrAlgorithm) -> Self {
        match value {
            DrAlgorithm::Other => { 0u8 }
            DrAlgorithm::Static => { 1u8 }
            DrAlgorithm::DrmFPW => { 2u8 }
            DrAlgorithm::DrmRPW => { 3u8 }
            DrAlgorithm::DrmRVW => { 4u8 }
            DrAlgorithm::DrmFVW => { 5u8 }
            DrAlgorithm::DrmFPB => { 6u8 }
            DrAlgorithm::DrmRPB => { 7u8 }
            DrAlgorithm::DrmRVB => { 8u8 }
            DrAlgorithm::DrmFVB => { 9u8 }
        }
    }
}

// TODO which one?
impl Default for DrAlgorithm {
    fn default() -> Self {
        DrAlgorithm::DrmFPW
    }
}

pub struct EntityMarking {
    pub marking_character_set : EntityMarkingCharacterSet,
    pub marking_string : String, // 11 byte String
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum EntityMarkingCharacterSet {
    Unused = 0,
    ASCII = 1,
    ArmyMarking = 2,
    DigitChevron = 3,
}

impl From<u8> for EntityMarkingCharacterSet {
    fn from(value: u8) -> Self {
        match value {
            0 => EntityMarkingCharacterSet::Unused,
            1 => EntityMarkingCharacterSet::ASCII,
            2 => EntityMarkingCharacterSet::ArmyMarking,
            3 => EntityMarkingCharacterSet::DigitChevron,
            _unspecified_value => EntityMarkingCharacterSet::Unused,
        }
    }
}

impl From<EntityMarkingCharacterSet> for u8 {
    fn from(value: EntityMarkingCharacterSet) -> Self {
        match value {
            EntityMarkingCharacterSet::Unused => { 0u8 }
            EntityMarkingCharacterSet::ASCII => { 1u8 }
            EntityMarkingCharacterSet::ArmyMarking => { 2u8 }
            EntityMarkingCharacterSet::DigitChevron => { 3u8 }
        }
    }
}

impl Default for EntityMarkingCharacterSet {
    fn default() -> Self {
        EntityMarkingCharacterSet::ASCII
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

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ApTypeDesignator {
    Articulated = 0,
    Attached = 1,
}

impl From<u8> for ApTypeDesignator {
    fn from(value: u8) -> Self {
        match value {
            1 => ApTypeDesignator::Attached,
            0 | _ => ApTypeDesignator::Articulated,
        }
    }
}

impl From<ApTypeDesignator> for u8 {
    fn from(value: ApTypeDesignator) -> Self {
        match value {
            ApTypeDesignator::Articulated => { 0u8 }
            ApTypeDesignator::Attached => { 1u8 }
        }
    }
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

#[derive(Copy, Clone, Debug, PartialEq)]
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

impl From<u8> for ApTypeMetric {
    fn from(value: u8) -> Self {
        match value {
            1 => ApTypeMetric::Position,
            2 => ApTypeMetric::PositionRate,
            3 => ApTypeMetric::Extension,
            4 => ApTypeMetric::ExtensionRate,
            5 => ApTypeMetric::X,
            6 => ApTypeMetric::XRate,
            7 => ApTypeMetric::Y,
            8 => ApTypeMetric::YRate,
            9 => ApTypeMetric::Z,
            10 => ApTypeMetric::ZRate,
            11 => ApTypeMetric::Azimuth,
            12 => ApTypeMetric::AzimuthRate,
            13 => ApTypeMetric::Elevation,
            14 => ApTypeMetric::ElevationRate,
            15 => ApTypeMetric::Rotation,
            16 => ApTypeMetric::RotationRate,
            0 | _ => ApTypeMetric::Unspecified,
        }
    }
}

impl From<ApTypeMetric> for u32 {
    fn from(value: ApTypeMetric) -> Self {
        match value {
            ApTypeMetric::Unspecified => { 0u32 }
            ApTypeMetric::Position => { 1u32 }
            ApTypeMetric::PositionRate => { 2u32 }
            ApTypeMetric::Extension => { 3u32 }
            ApTypeMetric::ExtensionRate => { 4u32 }
            ApTypeMetric::X => { 5u32 }
            ApTypeMetric::XRate => { 6u32 }
            ApTypeMetric::Y => { 7u32 }
            ApTypeMetric::YRate => { 8u32 }
            ApTypeMetric::Z => { 9u32 }
            ApTypeMetric::ZRate => { 10u32 }
            ApTypeMetric::Azimuth => { 11u32 }
            ApTypeMetric::AzimuthRate => { 12u32 }
            ApTypeMetric::Elevation => { 13u32 }
            ApTypeMetric::ElevationRate => { 14u32 }
            ApTypeMetric::Rotation => { 15u32 }
            ApTypeMetric::RotationRate => { 16u32 }
        }
    }
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

impl EntityState {
    pub fn builder() -> EntityStateBuilder {
        EntityStateBuilder::new()
    }

    // pub fn serialize() ->
}