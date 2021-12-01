

struct EntityState {
    header : PduHeader, // struct
    entity_id : EntityId, // struct
    force_id : ForceId, // enum
    articulated_parts_no : u8,
    entity_type : EntityType, // struct
    alternative_entity_type : EntityType, // struct
    entity_linear_velocity : LinearVelocity, // struct
    entity_location : Location, // struct
    entity_orientation : Orientation, // struct
    entity_appearance : Appearance, // struct
    dead_reckoning_parameters : DrParameters, // struct
    entity_marking : EntityMarking, // struct
    entity_capabilities : EntityCapabilities, // struct
    articulation_parameter : Option<List<ArticulationParameter>>, // optional list of records
}

struct PduHeader {
    protocol_version : ProtocolVersion, // enum
    exercise_id : u8,
    pdu_type : PduType, // enum
    protocol_family : ProtocolFamily, // enum
    time_stamp : u32,
    pdu_length : u16,
    padding_field : u16,
}

enum ProtocolVersion {
    Other = 0,
    VERSION_1_0_MAY_92 = 1,             // DIS PDU version 1.0 (May 92)
    IEEE_1278_1993 = 2,                 // IEEE 1278-1993
    VERSION_2_0_THIRD_DRAFT = 3,        // DIS PDU version 2.0 - third draft (May 93)
    VERSION_2_0_FOURTH_DRAFT = 4,       // DIS PDU version 2.0 - fourth draft (revised) March 16, 1994
    IEEE_1278_1_1995 = 5,               // IEEE 1278.1-1995
}

enum PduType {
	Other = 0,
	Entity_State = 1,
    Fire = 2,
    Detonation = 3,
    Collision = 4,
    ServiceRequest = 5,
    ResupplyOffer = 6,
    ResupplyReceived = 7,
    ResupplyCancel = 8,
    RepairComplete = 9,
    RepairResponse = 10,
    CreateEntity = 11,
    RemoveEntity = 12,
    StartResume = 13,
    StopFreeze = 14,
    Acknowledge = 15,
    ActionRequest = 16,
    ActionResponse = 17,
    DataQuery = 18,
    SetData = 19,
    Data = 20,
    EventReport = 21,
    Comment = 22,
    ElectromagneticEmission = 23,
    Designator = 24,
    Transmitter = 25,
    Signal = 26,
    Receiver = 27,
    AnnounceObject = 129,
    DeleteObject = 130,
    DescribeApplication = 131,
    DescribeEvent = 132,
    DescribeObject = 133,
    RequestEvent = 134,
    RequestObject = 135,
    TimeSpacePositionIndicatorFI = 140,
    AppearanceFI = 141,
    ArticulatedPartsFI = 142,
    FireFI = 143,
    DetonationFI = 144,
    PointObjectState = 150,
    LinearObjectState = 151,
    ArealObjectState = 152,
    Environment = 153,
    TransferControlRequest = 155,
    TransferControl = 156,
    TransferControlAcknowledge = 157,
    IntercomControl = 160,
    IntercomSignal = 161,
    Aggregate = 170,
}

enum ProtocolFamily {
    Other = 0,
	EntityInformationInteraction = 1,
	ExperimentalCGF = 129,
	ExperimentalEntityInteractionInformationFieldInstrumentation = 130,
	ExperimentalWarfareFieldInstrumentation = 131,
	ExperimentalEnvironmentObjectInformationInteraction = 132,
	ExperimentalEntityManagement = 133,
	Warfare = 2,
	Logistics = 3,
	RadioCommunication = 4,
	SimulationManagement = 5,
	DistributedEmissionRegeneration = 6,
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

struct LinearVelocity {
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
    // TODO
}

enum EntityMobilityKill {
    // TODO
}

enum EntityFirePower {
    // TODO
}

enum EntityDamage {
    // TODO
}

enum EntitySmoke {
    // TODO
}

enum EntityTrailingEffect {
    // TODO
}

enum EntityHatchState {
    // TODO
}

enum EntityLights {
    // TODO
}

enum EntityFlamingEffect {
    // TODO
}

struct DrParameters {
    // TODO
}

struct EntityMarking {
    // TODO
}

struct EntityCapabilities {
    // TODO
}

struct ArticulationParameter {
    // TODO
}