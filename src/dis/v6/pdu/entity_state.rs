use super::*;

pub struct EntityState {
    header : PduHeader, // struct
    entity_id : EntityId, // struct
    force_id : ForceId, // enum
    articulated_parts_no : u8, // FIXME can be obtained from length of articulation_parameter field
    entity_type : EntityType, // struct
    alternative_entity_type : EntityType, // struct
    entity_linear_velocity : VectorF32, // struct
    entity_location : Location, // struct
    entity_orientation : Orientation, // struct
    entity_appearance : Appearance, // struct
    dead_reckoning_parameters : DrParameters, // struct
    entity_marking : EntityMarking, // struct
    entity_capabilities : EntityCapabilities, // struct
    articulation_parameter : Option<List<ArticulationParameter>>, // optional list of records
}

struct EntityId {
    simulation_address : SimulationAddress,
    entity_id : u16
}

struct SimulationAddress {
    site_id : u16,
    application_id : u16,
}

enum ForceId {
    Other = 0,
    Friendly = 1,
    Opposing = 2,
    Neutral = 3,
}

struct EntityType {
    kind : EntityKind,
    domain : u8,
    country : Country,
    category : u8,
    subcategory : u8,
    specific : u8,
    extra : u8,
}

enum EntityKind {
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

enum Country {
    Other = 0,
// TODO
// regex: (?<value>\d)+\s+(?<field>\S)+
// replace: \t${field} = ${value},
// 1	Afghanistan
// 10	Argentina
// 100	Indonesia
// 101	Iran
// 102	Iraq
// 104	Ireland
// 105	Israel
// 106	Italy
// 107	Cote D'Ivoire (aka Ivory Coast)
// 108	Jamaica
// 109	Jan Mayen (Norway)
// 11	Aruba
// 110	Japan
// 111	Jarvis Island (United States)
// 112	Jersey (United Kingdom)
// 113	Johnston Atoll (United States)
// 114	Jordan
// 115	Juan de Nova Island
// 116	Kenya
// 117	Kingman Reef (United States)
// 118	Kiribati
// 119	Korea, Democratic People's Republic of (North)
// 12	Ashmore and Cartier Islands (Australia)
// 120	Korea, Republic of (South)
// 121	Kuwait
// 122	Laos
// 123	Lebanon
// 124	Lesotho
// 125	Liberia
// 126	Libya
// 127	Liechtenstein
// 128	Luxembourg
// 129	Madagascar
// 13	Australia
// 130	Macau (Portugal)
// 131	Malawi
// 132	Malaysia
// 133	Maldives
// 134	Mali
// 135	Malta
// 136	Man, Isle of (United Kingdom)
// 137	Marshall Islands
// 138	Martinique (France)
// 139	Mauritania
// 14	Austria
// 140	Mauritius
// 141	Mayotte (France)
// 142	Mexico
// 143	Micronesia, Federative States of
// 144	Monaco
// 145	Mongolia
// 146	Montserrat (United Kingdom)
// 147	Morocco
// 148	Mozambique
// 149	Namibia (South West Africa)
// 15	Bahamas
// 150	Nauru
// 151	Navassa Island (United States)
// 152	Nepal
// 153	Netherlands
// 154	Netherlands Antilles (Curacao, Bonaire, Saba, Sint Maarten Sint Eustatius)
// 155	New Caledonia (France)
// 156	New Zealand
// 157	Nicaragua
// 158	Niger
// 159	Nigeria
// 16	Bahrain
// 160	Niue (New Zealand)
// 161	Norfolk Island (Australia)
// 162	Northern Mariana Islands (United States)
// 163	Norway
// 164	Oman
// 165	Pakistan
// 166	Palmyra Atoll (United States)
// 168	Panama
// 169	Papua New Guinea
// 17	Baker Island (United States)
// 170	Paracel Islands (International - Occupied by China, also claimed by Taiwan and Vietnam)
// 171	Paraguay
// 172	Peru
// 173	Philippines
// 174	Pitcairn Islands (United Kingdom)
// 175	Poland
// 176	Portugal
// 177	Puerto Rico (United States)
// 178	Qatar
// 179	Reunion (France)
// 18	Bangladesh
// 180	Romania
// 181	Rwanda
// 182	St. Kitts and Nevis
// 183	St. Helena (United Kingdom)
// 184	St. Lucia
// 185	St. Pierre and Miquelon (France)
// 186	St. Vincent and the Grenadines
// 187	San Marino
// 188	Sao Tome and Principe
// 189	Saudi Arabia
// 19	Barbados
// 190	Senegal
// 191	Seychelles
// 192	Sierra Leone
// 193	Singapore
// 194	Solomon Islands
// 195	Somalia
// 196	South Georgia and the South Sandwich Islands(United Kingdom)
// 197	South Africa
// 198	Spain
// 199	Spratly Islands (International - parts occupied and claimed by China,Malaysia, Philippines, Taiwan, Vietnam)
// 2	Albania
// 20	Bassas da India (France)
// 200	Sri Lanka
// 201	Sudan
// 202	Suriname
// 203	Svalbard (Norway)
// 204	Swaziland
// 205	Sweden
// 206	Switzerland
// 207	Syria
// 208	Taiwan
// 209	Tanzania
// 21	Belgium
// 210	Thailand
// 211	Togo
// 212	Tokelau (New Zealand)
// 213	Tonga
// 214	Trinidad and Tobago
// 215	Tromelin Island (France)
// 216	Pacific Islands, Trust Territory of the (Palau)
// 217	Tunisia
// 218	Turkey
// 219	Turks and Caicos Islands (United Kingdom)
// 22	Belize
// 220	Tuvalu
// 221	Uganda
// 222	Commonwealth of Independent States
// 223	United Arab Emirates
// 224	United Kingdom
// 225	United States
// 226	Uruguay
// 227	Vanuatu
// 228	Vatican City (Holy See)
// 229	Venezuela
// 23	Benin (aka Dahomey)
// 230	Vietnam
// 231	Virgin Islands (United States)
// 232	Wake Island (United States)
// 233	Wallis and Futuna (France)
// 234	Western Sahara
// 235	West Bank (Israel)
// 236	Western Samoa
// 237	Yemen
// 24	Bermuda (United Kingdom)
// 241	Zaire
// 242	Zambia
// 243	Zimbabwe
// 244	Armenia
// 245	Azerbaijan
// 246	Belarus
// 247	Bosnia and Hercegovina
// 248	Clipperton Island (France)
// 249	Croatia
// 25	Bhutan
// 250	Estonia
// 251	Georgia
// 252	Kazakhstan
// 253	Kyrgyzstan
// 254	Latvia
// 255	Lithuania
// 256	Macedonia
// 257	Midway Islands (United States)
// 258	Moldova
// 259	Montenegro
// 26	Bolivia
// 260	Russia
// 261	Serbia and Montenegro (Montenegro to separate)
// 262	Slovenia
// 263	Tajikistan
// 264	Turkmenistan
// 265	Ukraine
// 266	Uzbekistan
// 27	Botswana
// 28	Bouvet Island (Norway)
// 29	Brazil
// 3	Algeria
// 30	British Indian Ocean Territory (United Kingdom)
// 31	British Virgin Islands (United Kingdom)
// 32	Brunei
// 33	Bulgaria
// 34	Burkina (aka Burkina Faso or Upper Volta
// 35	Burma (Myanmar)
// 36	Burundi
// 37	Cambodia (aka Kampuchea)
// 38	Cameroon
// 39	Canada
// 4	American Samoa (United States)
// 40	Cape Verde, Republic of
// 41	Cayman Islands (United Kingdom)
// 42	Central African Republic
// 43	Chad
// 44	Chile
// 45	China, People's Republic of
// 46	Christmas Island (Australia)
// 47	Cocos (Keeling) Islands (Australia)
// 48	Colombia
// 49	Comoros
// 5	Andorra
// 50	Congo, Republic of
// 51	Cook Islands (New Zealand)
// 52	Coral Sea Islands (Australia)
// 53	Costa Rica
// 54	Cuba
// 55	Cyprus
// 56	Czechoslovakia (separating into Czech Republic and Slovak Republic)
// 57	Denmark
// 58	Djibouti
// 59	Dominica
// 6	Angola
// 60	Dominican Republic
// 61	Ecuador
// 62	Egypt
// 63	El Salvador
// 64	Equatorial Guinea
// 65	Ethiopia
// 66	Europa Island (France)
// 67	Falkland Islands (aka Islas Malvinas) (United Kingdom)
// 68	Faroe Islands (Denmark)
// 69	Fiji
// 7	Anguilla
// 70	Finland
// 71	France
// 72	French Guiana (France)
// 73	French Polynesia (France)
// 74	French Southern and Antarctic Islands (France)
// 75	Gabon
// 76	Gambia, The
// 77	Gaza Strip (Israel)
// 78	Germany
// 79	Ghana
// 8	Antarctica (International)
// 80	Gibraltar (United Kingdom)
// 81	Glorioso Islands (France)
// 82	Greece
// 83	Greenland (Denmark)
// 84	Grenada
// 85	Guadaloupe (France)
// 86	Guam (United States)
// 87	Guatemala
// 88	Guernsey (United Kingdom)
// 89	Guinea
// 9	Antigua and Barbuda
// 90	Guinea- Bissau
// 91	Guyana
// 92	Haiti
// 93	Heard Island and McDonald Islands (Australia)
// 94	Honduras
// 95	Hong Kong (United Kingdom)
// 96	Howland Island (United States)
// 97	Hungary
// 98	Iceland
// 99	India
}

struct VectorF32 {
    first_vector_component : f32,
    second_vector_component : f32,
    third_vector_component : f32,
}

struct Location {
    x_coordinate : f64,
    y_coordinate : f64,
    z_coordinate : f64,
}

struct Orientation {
    psi : f32,
    theta : f32,
    phi : f32,
}

struct Appearance {
    general_appearance : GeneralAppearance,
    specific_appearance : SpecificAppearance,
}

struct GeneralAppearance {
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

enum EntityPaintScheme {
    UniformColor = 0,
    Camouflage = 1,
}

enum EntityMobilityKill {
    NoMobilityKill = 0,
    MobilityKill = 1,
}

enum EntityFirePower {
    NoFirePowerKill = 0,
    FirePowerKill = 1,
}

enum EntityDamage {
    NoDamage = 0,
    SlightDamage = 1,
    ModerateDamage = 2,
    Destroyed = 3,
}

enum EntitySmoke {
    NotSmoking = 0,
    SmokePlumeRising = 1,
    EmittingEngineSmoke = 2,
    EmittingEngineSmokeAndSmokePlumeRising = 3,
}

enum EntityTrailingEffect {
    None = 0,
    Small = 1,
    Medium = 2,
    Large = 3,
}

enum EntityHatchState {
    NotApplicable = 0,
    Closed = 1,
    Popped = 2,
    PoppedAndPersonVisible = 3,
    Open = 4,
    OpenAndPersonVisible = 5,
    Unused1 = 6,
    Unused2 = 7,
}

enum EntityLights {
    None = 0,
    RunningLightsOn = 1,
    NavigationLightsOn = 2,
    FromationLightsOn = 3,
    Unused1 = 4,
    Unused2 = 5,
    Unused3 = 6,
    Unused4 = 7,
}

enum EntityFlamingEffect {
    None = 0,
    FlamesPresent = 1,
}

// TODO replace u16 with specific types for the variants
enum SpecificAppearance {
    LandPlatform(u16),
    AirPlatform(u16),
    SurfacePlatform(u16),
    SubsurfacePlatform(u16),
    SpacePlatform(u16),
    GuidedMunition(u16),
    LifeForm(u16),
    Environmental(u16),
}

struct DrParameters {
    algorithm : DrAlgorithm,
    other_parameters : DrOtherParameters,
    linear_acceleration : VectorF32,
    angular_velocity : VectorF32,
}

enum DrAlgorithm {
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

struct DrOtherParameters {
    // 120-bits padding
}

struct EntityMarking {
    marking_character_set : EntityMarkingCharacterSet,
    marking_string : [u8; 11], // 11 byte String
}

enum EntityMarkingCharacterSet {
    Unused = 0,
    ASCII = 1,
    ArmyMarking = 2,
    DigitChevron = 3,
}

struct EntityCapabilities {
    ammunition_supply : bool,
    fuel_supply : bool,
    recovery : bool,
    repair : bool,
    // 28-bits padding
}

struct ArticulationParameter {
    parameter_type_designator : ApTypeDesignator,
    parameter_change_indicator : u8,
    articulation_attachment_ic : u16,
    parameter_type_variant : ParameterTypeVariant,
    articulation_parameter_value : ArticulationParameterValue,
}

enum ApTypeDesignator {
    Articulated = 0,
    Attached = 1,
}

struct ParameterTypeVariant {
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

struct ArticulatedParts {
    low_bits : ApLowBits,
    high_bits : ApHighBits,
}

enum ApLowBits {
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

enum ApHighBits {
    Placeholder = 0,
// TODO
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