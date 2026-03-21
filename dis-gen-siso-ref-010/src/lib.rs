use quick_xml::Reader;
use serde::Deserialize;
use std::collections::HashMap;
use std::ops::RangeInclusive;
use std::path::{Path, PathBuf};
use std::{env, fs};

mod extraction;
mod generation;

const OUT_DIR: &str = "OUT_DIR";
const TARGET_ENV_VAR: &str = "TARGET_GENERATED_SISO_REF_010_FILENAME";
const TARGET_OUT_FILE: &str = "siso_ref_010.rs";

const WRAP_ENUM_UID_OFFSET: usize = 0xFFFF_0000_0000_0000;
const WRAP_ENUM_TRIM_NAME: &str = " Type";
const WRAP_ENUM_SIZE: usize = 32; // Now a hardcoded size, as both capabilities and (extended) appearance are 32 bits.

// TODO 1) generate enums based on 'overrides' instead of listing all that need to be generated (inverse) (521 items).
//  - less usage of name overrides
//  - auto infer data size overrides
//  - flag indicating to not generate the UID
//  - uniform way of handling for all UID types (enums, bitfields, records)
//  - adding `footnotes` to the code; check what we now do with `description`s of bitfieldrows
// V 2) generate all bitfields (60 items).
// TODO 3) generate the record types as well (31 items).

/// Array containing all the uids of enumerations that should be generated.
/// Each entry is a tuple containing:
/// - the uid,
/// - an Optional string literal to override the name of the resulting enum,
/// - an Optional data size (in bits),
/// - a bool flag to indicate that the value of an enum item must be appended to the name.
///
/// For example, the '`DISPDUType`' enum (having uid 4) has an override
/// to `PduType`, which is nicer in code. The entry thus is (4, Some("PduType"), None, false)
///
/// Also, the 'Articulated Parts-Type Metric' enum has a defined size of 5,
/// but needs to be aligned with a 32-bit field.
///
/// Finally, some enums have variants that result in empty names (`""`) or duplicate names (such as 'Emitter Name').
/// The bool flag will append `"_value"` to the name of the variant to make it unique
// const ENUM_UIDS: [(usize, Option<&str>, Option<usize>, bool); 155] = [
//     (3, Some("ProtocolVersion"), None, false), // DIS-Protocol Version
//     (4, Some("PduType"), None, false),         // DIS-PDU Type
//     (5, Some("ProtocolFamily"), None, false),  // DIS-PDU Family
//     (6, Some("ForceId"), None, false),         // Force ID
//     (7, None, None, false),                    // Entity Kind
//     (8, None, None, false),                    // Domain
//     // 9-28 // (Sub-)Categories
//     (29, None, None, false), // Country
//     // 30 // Entity Types records
//     // 31-43 // Bitfields, see `BITFIELD_UIDS`
//     (44, None, None, false),                      // Dead Reckoning Algorithm
//     (45, None, None, false),                      // Entity Marking Character Set
//     (46, None, None, false),                      // Location Definition
//     (52, None, None, false),                      // Entity Clamping Type
//     (53, None, None, false),                      // Vertical Reference
//     (55, None, None, false), // Entity Capabilities (together with bitfields 450-462)
//     (56, None, None, false), // Variable Parameter Record Type
//     (57, None, None, false), // Attached Parts
//     (58, None, Some(32), false), // Articulated Parts-Type Metric
//     (59, None, None, false), // Articulated Parts-Type Class
//     (60, None, None, false), // Munition Descriptor-Warhead
//     (61, None, None, true),  // Munition Descriptor-Fuse
//     (62, None, None, false), // Detonation result
//     (63, None, None, false), // Service Type Requested
//     (64, None, None, true),  // Repair Complete-Repair
//     (65, None, None, false), // Repair Complete-Repair Result
//     (66, Some("VariableRecordType"), None, true), // Variable Record Types
//     (67, None, None, false), // Stop/Freeze Reason
//     (69, Some("AcknowledgeFlag"), None, false), // Acknowledge-Acknowledge Flag
//     (70, Some("ResponseFlag"), None, false), // Acknowledge-Response Flag
//     (71, Some("ActionId"), None, false), // Action Request-Action ID
//     (72, Some("RequestStatus"), None, false), // Action Request-Request Status
//     (73, Some("EventType"), None, false), // Event Report-Event Type
//     (74, None, None, false), // Required Reliability Service
//     (75, None, None, true),  // Emitter Name
//     (76, None, None, true),  // Emitter System Function
//     (77, None, None, false), // Electromagnetic Emission-State Update Indicator
//     (78, None, None, false), // Electromagnetic Emission-Beam Function
//     (79, None, None, false), // High Density Track/Jam
//     (80, None, None, false), // Designator System Name
//     (81, Some("DesignatorCode"), None, false), // Designator Code
//     (82, Some("IffSystemType"), None, false), // IFF-System Type
//     (83, Some("IffSystemName"), None, false), // IFF-System Name
//     (84, Some("IffSystemMode"), None, false), // IFF-System Mode
//     // 87, 96-98 // IFF stuff
//     // 100-106, // Subcategories
//     // (141, None, None, false), // Appearance Type
//     // (142, None, None, false), // Extended Appearance Type
//     (143, None, None, false), // UA-State/Change Update Indicator
//     (144, None, None, false), // UA-Acoustic System Name
//     (145, None, None, false), // UA-Acoustic Emitter System Function
//     (146, None, None, false), // UA-Active Emission Parameter Index
//     (147, None, None, false), // UA-Scan Pattern
//     (148, None, None, false), // UA-Passive Parameter Index
//     (150, None, None, false), // UA-Additional Passive Activity Parameter Index
//     // (151, None, None, false), // Channel Type
//     // (152, None, None, false), // Channel Detail
//     // (153, None, None, false), // Transmitter Waveform Type
//     // (154, None, None, false), // Transmitter Waveform Detail
//     (155, None, None, false), // Transmitter Major Modulation
//     (156, None, None, false), // Transmitter-Detail-Amplitude Modulation
//     (
//         157,
//         Some("TransmitterDetailAmplitudeAngleModulation"),
//         None,
//         false,
//     ), // Transmitter-Detail-Amplitude and Angle Modulation
//     (158, Some("TransmitterDetailAngleModulation"), None, false), // Transmitter-Detail-Angle modulation
//     (159, None, None, false), // Transmitter-Detail-Combination Modulation
//     (160, None, None, false), // Transmitter-Detail-Pulse Modulation
//     (161, None, None, false), // Transmitter-Detail-Unmodulated Modulation
//     (162, None, None, false), // Transmitter-Detail-Carrier Phase Shift Modulation
//     (163, None, None, false), // Transmitter-Modulation Type System
//     (164, None, None, false), // Transmitter Transmit State
//     (165, None, None, false), // Transmitter Input Source
//     (166, None, None, false), // Transmitter Crypto System
//     (167, None, None, false), // Transmitter Antenna Pattern Type
//     (168, None, None, false), // Transmitter Antenna Pattern Reference System
//     // (169, None, None, false), // Surrogate Group
//     // (176, None, None, false), // Message Type Identifier
//     (177, None, None, false), // Signal User Protocol Identification Number
//     (178, Some("SignalTdlType"), None, true), // Signal TDL Type
//     (179, Some("ReceiverState"), None, false), // Receiver Receiver State
//     (189, None, None, false), // Collision Type
//     (204, None, None, false), // Aggregate State-Aggregate State
//     (205, None, None, false), // Aggregate State-Formation
//     (206, None, None, false), // Aggregate State-Aggregate Kind
//     // (207, None, None, false), // Aggregate State-Aggregate Types -- not supported
//     (208, None, None, false),                // Aggregate State-Subcategory
//     (209, None, None, false),                // Aggregate State-Specific
//     (210, None, None, false),                // IsPartOf-Nature
//     (211, None, None, false),                // IsPartOf-Position
//     (212, Some("StationName"), None, false), // IsPartOf-Station Name
//     (213, None, None, false),                // IsGroupOf-Grouped Entity Category
//     (224, None, None, true),                 // Transfer Control-Transfer Type
//     (270, None, Some(16), false),            // Signal Encoding Class
//     (271, None, Some(16), true),             // Signal Encoding Type
//     (281, Some("APAStatus"), None, false),   // APA Parameter Index-APA Status
//     (282, Some("SeparationReasonForSeparation"), None, false), // Separation VP-Reason for Separation
//     (283, Some("SeparationPreEntityIndicator"), None, false),  // Separation VP-Pre-Entity Indicator
//     (295, Some("AttributeActionCode"), None, false),           // Attribute Action Code
//     (296, Some("DrParametersType"), None, false),              // Dead Reckoning Parameters Type
//     (301, Some("TransferredEntityIndicator"), None, false), // DIS-PDU Status-Transferred Entity Indicator (TEI)
//     (302, Some("LvcIndicator"), None, false),               // DIS-PDU Status-LVC Indicator (LVC)
//     (303, Some("CoupledExtensionIndicator"), None, false), // DIS-PDU Status-Coupled Extension Indicator (CEI)
//     (304, Some("FireTypeIndicator"), None, false), // DIS-PDU Status-Fire Type Indicator (FTI)
//     (305, Some("DetonationTypeIndicator"), None, false), // DIS-PDU Status-Detonation Type Indicator (DTI)
//     (306, Some("RadioAttachedIndicator"), None, false),  // Radio Attached Indicator
//     (307, Some("IntercomAttachedIndicator"), None, false), // DIS-PDU Status-Intercom Attached Indicator (IAI)
//     (308, Some("IffSimulationMode"), None, false), // DIS-PDU Status-IFF Simulation Mode (ISM)
//     (310, None, None, false),                      // Explosive Material Categories
//     (318, None, None, false),                      // Beam Status-Beam State
//     (319, None, None, false),                      // Entity Association-Association Status
//     (320, Some("ChangeIndicator"), None, false),   // Entity VP Record-Change Indicator
//     (321, None, None, false),                      // Entity Association-Group Member Type
//     (323, None, None, false),                      // Entity Association-Physical Association Type
//     (324, None, None, false),                      // Entity Association-Physical Connection Type
//     (334, None, None, false),                      // Record Query-R-Event Type
//     (335, Some("UAPropulsionPlantConfiguration"), None, false), // UA Propulsion Plant Configuration-Configuration
//     (339, Some("IffApplicableModes"), None, false),             // IFF Applicable Modes
//     (346, Some("Mode5IffMission"), None, false),                // IFF Mission
//     (347, Some("ModeSTransmitState"), Some(8), false), // Mode S Interrogator Status Transmit State
//     (350, None, None, false),                          // Mode 5 Reply
//     (351, None, None, false),                          // Antenna Selection
//     (353, None, None, false),                          // Mode S Squitter Type
//     (354, None, None, false),                          // Mode S Squitter Type
//     (355, None, None, false),                          // Mode S Squitter Record Source
//     (356, None, None, false),                          // Aircraft Present Domain
//     (357, None, None, false),                          // Aircraft Identification Type
//     (358, None, None, true),                           // Capability Report
//     (359, None, None, true),                           // Navigation Source
//     (361, Some("Mode5SAltitudeResolution"), None, false), // Mode 5/S Altitude Resolution
//     (369, None, None, false),                          // Data Category
//     (378, None, None, false),                          // Appearance-Paint Scheme
//     (379, None, None, false),                          // Appearance-Damage
//     (380, None, None, false),                          // Mode 5 Message Formats Status
//     (381, None, None, false),                          // Appearance-Trailing Effects
//     (382, None, None, false),                          // Appearance-Hatch
//     (383, None, None, false),                          // Appearance-Launcher/Operational
//     (384, None, None, false),                          // Appearance-Camouflage Type
//     (385, None, None, false),                          // Appearance-Concealed Position
//     (386, None, None, false),                          // Appearance-Entity or Object State
//     (387, None, None, false),                          // Appearance-Canopy
//     (388, None, None, false),                          // Appearance-Subsurface Hatch
//     (389, Some("Active Interrogation Indicator"), None, false), // DIS-PDU Status-Active Interrogation Indicator (AII)
//     (390, None, None, false),                                   // Appearance-Lifeform Health
//     (391, None, None, false), // Appearance-Life Form Compliance Status
//     (392, None, None, false), // Appearance-Life Form Posture
//     (393, None, None, false), // Appearance-Life Form Weapon/Implement
//     (394, None, None, false), // Appearance-Concealed Movement
//     (395, None, None, false), // Appearance-Environmental Density
//     (396, None, None, false), // Mode 5 Platform Type
//     (397, None, None, false), // Appearance-Anti-Collision Day/Night
//     (398, None, None, false), // Appearance-Navigation/Position Brightness
//     (399, None, None, false), // Appearance-Supply Deployed
//     (400, None, None, false), // Appearance-NVG Mode
//     (401, None, None, false), // Parachute
//     (402, None, None, false), // Flare/Smoke Color
//     (403, None, None, false), // Flare/Smoke Status
//     (404, None, None, false), // Spot Chaff Status
//     (405, None, None, false), // Appearance-Object General-Damage
//     (406, None, None, false), // Appearance-Object General-Predistributed
//     (407, None, None, false), // Appearance-Object Specific-Breach State
//     (408, None, None, false), // Appearance-Object Specific-Chemical Type
//     (409, None, None, false), // Appearance-Linear Object Tank Ditch Breach
//     (410, None, None, false), // Appearance-Linear Object Lane Marker Visible
//     (411, None, None, false), // Appearance-Object General-IED Present
//     (412, None, None, false), // Mode 5 Level Selection
//     (415, None, None, false), // Attached Part-Detached Indicator
//     (423, None, None, false), // Mode 5 Location Errors
//     (426, None, None, false), // Cover/Shroud Status
//     // 427 - 448, 478 - 479 // SubCategories (EntityType)
//     // 481 - 482, 505 - 527 // Specifics (EntityType)
//     (589, None, None, false), // Transmitter-Detail-SATCOM-Modulation
//     // 800 // Link 16 Version
//     // 801 // Aircraft ID Source
//     (802, None, None, false), // Clothing IR Signature
//     // 803-887 // Do not exist
//     (889, None, None, false), // Damage Area
// ];

// const BITFIELD_UIDS: [RangeInclusive<usize>; 4] = [
//     31..=43, // Appearances
//     68..=68, // StopFreeze Frozen Behavior
//     // 230..=239, // Point Object Appearance - Linear Object Appearance - Areal Object Appearance
//     450..=462, // Capabilities
//     // 483..=487, // Point Object Appearances
//     // 488..=489, // Linear Object Appearances
//     // 149..=149, // UA-Propulsion Plant Configuration -- TODO does not compile as of yet
//     // TODO 54 - Cultural Feature General Appearance
//     // TODO 480 - Non-Human Life Forms Appearance
//     591..=591, // NET ID Record
// ];

/// Some enums cross-reference "record" elements.
/// Such records are not generated by this script
/// and must be excluded using the `SKIP_XREF_UIDS` array.
///
/// The `EnumItem::CrossRef` variant that would normally be constructed
/// from the XML structure will be changed to a regular `EnumItem::Basic`
const SKIP_XREF_UIDS: [usize; 1] = [220];

#[derive(Debug, Clone)]
pub enum GenerationItem {
    Enum(Enum),
    Bitfield(Bitfield),
}

impl<'a> GenerationItem {
    #[must_use]
    pub fn uid(&self) -> usize {
        match self {
            GenerationItem::Enum(e) => e.uid,
            GenerationItem::Bitfield(b) => b.uid,
        }
    }

    #[must_use]
    pub fn name(&'a self) -> &'a str {
        match self {
            GenerationItem::Enum(e) => e.name.as_str(),
            GenerationItem::Bitfield(b) => b.name.as_str(),
        }
    }

    #[must_use]
    pub fn size(&self) -> usize {
        match self {
            GenerationItem::Enum(e) => e.size,
            GenerationItem::Bitfield(b) => b.size,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Enum {
    pub uid: usize,
    pub name: String,
    pub size: usize,
    pub items: Vec<EnumItem>,
    pub discriminant: Option<String>,
}

#[derive(Debug, Clone)]
pub enum EnumItem {
    Basic(BasicEnumItem),
    Range(RangeEnumItem),
    CrossRef(CrossRefEnumItem),
}

#[derive(Debug, Clone)]
pub struct BasicEnumItem {
    pub description: String,
    pub value: usize,
    #[allow(
        unused,
        reason = "Deprecated items are included for full compatibility."
    )]
    pub deprecated: bool,
}

#[derive(Debug, Clone)]
pub struct RangeEnumItem {
    pub description: String,
    pub range: RangeInclusive<usize>,
    #[allow(
        unused,
        reason = "Deprecated items are included for full compatibility."
    )]
    pub deprecated: bool,
}

#[derive(Debug, Clone)]
pub struct CrossRefEnumItem {
    pub description: String,
    pub value: usize,
    pub xref: usize,
    #[allow(
        unused,
        reason = "Deprecated items are included for full compatibility."
    )]
    pub deprecated: bool,
}

#[derive(Debug, Clone)]
pub struct Bitfield {
    pub uid: usize,
    pub name: String,
    pub size: usize,
    pub fields: Vec<BitfieldItem>,
}

#[derive(Debug, Clone)]
pub struct BitfieldItem {
    pub name: String,
    pub bit_position: usize,
    pub length: usize,
    pub xref: Option<usize>,
    pub description: String,
}

type Overrides = HashMap<usize, UidOverride>;

/// An `UidOverride` record is used to store overrides for UID items.
/// Available configurations are:
/// - `name`: Change the name of the to-be generated item (e.g. `Some Very Long Name` becomes `ShorterName`);
/// - `size`: Change the data size of the to-be generated item (e.g. '5' becomes '16').
/// - `postfix_value` Postfix fields of enumeration items with their value, because duplicate fields names exist in the SISO-REF-010 definitions.
/// - `skip`: Completely skip the item of being generated.
/// - `xref`: Handling of cross-referenced (`xref`) UID items in the enum variant. By default these xref'ed items are not considered.
///     - the value `"embed"` contains the xref UID in the enum variant.
///     - the value `"wrapper"` generates the enum as default, and also generates a similar enum that does embed the xrefs, for use in enum type and adaptive fields, respectively.
#[derive(Default, Debug)]
struct UidOverride {
    name: Option<String>,
    size: Option<usize>, // TODO see if we can derive this override when extracting or generating.
    postfix_value: bool,
    skip: bool,
    xref: OverrideXrefHandling,
}

impl From<OverrideEntry> for UidOverride {
    fn from(entry: OverrideEntry) -> Self {
        Self {
            name: entry.name,
            size: entry.size,
            postfix_value: entry.postfix.unwrap_or(false),
            skip: entry.skip.unwrap_or(false),
            xref: OverrideXrefHandling::from(&entry.xref),
        }
    }
}

#[derive(Default, Debug, PartialEq)]
enum OverrideXrefHandling {
    #[default]
    None,
    Embed,
    Wrap,
}

impl From<&Option<String>> for OverrideXrefHandling {
    fn from(value: &Option<String>) -> Self {
        const EMBED_CONFIG_STR: &str = "embed";
        const WRAPPER_CONFIG_STR: &str = "wrapper";
        match value {
            Some(s) if s == EMBED_CONFIG_STR => OverrideXrefHandling::Embed,
            Some(s) if s == WRAPPER_CONFIG_STR => OverrideXrefHandling::Wrap,
            None | _ => OverrideXrefHandling::None,
        }
    }
}

#[derive(Debug, Deserialize)]
struct OverrideConfig {
    overrides: HashMap<usize, OverrideEntry>,
}

#[derive(Debug, Deserialize)]
struct OverrideEntry {
    name: Option<String>,
    size: Option<usize>,
    postfix: Option<bool>,
    skip: Option<bool>,
    xref: Option<String>,
}

fn init_overrides(path: &Path) -> Overrides {
    let config_str = fs::read_to_string(path).expect("Failed to read overrides file");
    let config: OverrideConfig =
        toml::from_str(&config_str).expect("Failed to parse overrides file");

    let mut overrides = Overrides::new();

    for entry in config.overrides {
        overrides.insert(entry.0, UidOverride::from(entry.1));
    }

    overrides
}

/// This is the main entry point for generating the enumerations
/// as defined in the SISO-REF-010 reference document.
///
/// The `siso_ref_010_file` argument is a `&str` to the XML defining the enumerations.
///
/// # Panics
/// When errors are encountered when parsing the XML the function panics.
/// As this function is to be called from within a build script,
/// aborting by panicking is acceptable.
#[must_use]
pub fn execute(siso_ref_010_dir: &str) -> HashMap<usize, String> {
    let ref_path: PathBuf = [siso_ref_010_dir, "SISO-REF-010.xml"].iter().collect();
    let config_path: PathBuf = [siso_ref_010_dir, "overrides.toml"].iter().collect();
    let overrides = init_overrides(config_path.as_path());

    let mut reader = Reader::from_file(ref_path.as_path()).unwrap();
    reader.config_mut().trim_text(true);

    // Extract enums and bitfields from the source file
    let generation_items = extraction::extract(&mut reader);

    let wrapper_items = generation_items
        .iter()
        .filter_map(|item| {
            if let GenerationItem::Enum(e) = item {
                if overrides
                    .get(&e.uid)
                    .is_some_and(|entry| entry.xref == OverrideXrefHandling::Wrap)
                {
                    let wrapper_item = Enum {
                        uid: e.uid + WRAP_ENUM_UID_OFFSET,
                        name: e.name.trim_end_matches(WRAP_ENUM_TRIM_NAME).to_string(),
                        size: WRAP_ENUM_SIZE,
                        items: e.items.clone(),
                        discriminant: Some(e.name.clone()),
                    };
                    Some(wrapper_item)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect::<Vec<Enum>>();
    println!("wrapper uids: {wrapper_items:?}");
    let mut overrides = overrides;
    let mut generation_items = generation_items;
    for it in wrapper_items {
        overrides.insert(
            it.uid,
            UidOverride {
                xref: OverrideXrefHandling::Embed,
                ..Default::default()
            },
        );
        generation_items.push(GenerationItem::Enum(it));
    }
    let overrides = overrides;
    let generation_items = generation_items;

    // Build the index of UIDs and their code names
    let uid_index = generate_uid_index(&generation_items);

    // Generate the code for the enumerations
    generate_and_save(&generation_items, &overrides);

    uid_index
}

/// Generate an index of UIDs to the name of the generated code items
/// for use in further generation functions.
fn generate_uid_index(generation_items: &Vec<GenerationItem>) -> HashMap<usize, String> {
    let mut uid_index = HashMap::new();

    for item in generation_items {
        uid_index.insert(item.uid(), format_name(item.name(), item.uid()));
    }

    uid_index.insert(93, "Enumeration<u8>".to_string()); // FIXME not defined in SISO-REF-010 v36
    uid_index.insert(94, "Enumeration<u8>".to_string()); // FIXME not defined in SISO-REF-010 v36
    uid_index.insert(95, "Enumeration<u8>".to_string()); // FIXME not defined in SISO-REF-010 v36
    uid_index.insert(490, "Enumeration<u8>".to_string()); // FIXME not defined in SISO-REF-010 v36
    uid_index.insert(544, "Enumeration<u8>".to_string()); // FIXME not defined in SISO-REF-010 v36
    uid_index.insert(554, "Enumeration<u8>".to_string()); // FIXME not defined in SISO-REF-010 v36
    uid_index.insert(567, "Enumeration<u8>".to_string()); // FIXME not defined in SISO-REF-010 v36
    uid_index.insert(569, "Enumeration<u8>".to_string()); // FIXME not defined in SISO-REF-010 v36
    uid_index.insert(570, "Enumeration<u8>".to_string()); // FIXME not defined in SISO-REF-010 v36
    uid_index.insert(571, "Enumeration<u8>".to_string()); // FIXME not defined in SISO-REF-010 v36
    uid_index.insert(572, "Enumeration<u8>".to_string()); // FIXME not defined in SISO-REF-010 v36
    uid_index.insert(573, "Enumeration<u8>".to_string()); // FIXME not defined in SISO-REF-010 v36
    uid_index.insert(579, "Enumeration<u8>".to_string()); // FIXME not defined in SISO-REF-010 v36
    uid_index.insert(657, "Enumeration<u8>".to_string()); // FIXME not defined in SISO-REF-010 v36
    uid_index.insert(665, "Enumeration<u8>".to_string()); // FIXME not defined in SISO-REF-010 v36
    uid_index.insert(702, "Enumeration<u8>".to_string()); // FIXME not defined in SISO-REF-010 v36
    uid_index.insert(790, "Enumeration<u8>".to_string()); // FIXME not defined in SISO-REF-010 v36
    uid_index.insert(791, "Enumeration<u8>".to_string()); // FIXME not defined in SISO-REF-010 v36
    uid_index.insert(880, "Enumeration<u8>".to_string()); // FIXME not defined in SISO-REF-010 v36
    uid_index.insert(881, "Enumeration<u8>".to_string()); // FIXME not defined in SISO-REF-010 v36
    uid_index.insert(882, "Enumeration<u8>".to_string()); // FIXME not defined in SISO-REF-010 v36
    uid_index.insert(883, "Enumeration<u8>".to_string()); // FIXME not defined in SISO-REF-010 v36
    uid_index.insert(884, "Enumeration<u8>".to_string()); // FIXME not defined in SISO-REF-010 v36

    uid_index
}

/// Generates code for all provided `GenerationItem`s, formats the code and stores it in `OUT_DIR`.
fn generate_and_save(generation_items: &[GenerationItem], overrides: &Overrides) {
    // Generate all code for enums
    let generated = generation::generate(generation_items, &overrides);

    // format generated code using prettyplease
    let ast = syn::parse_file(&generated.to_string())
        .expect("Error parsing generated code for pretty printing.");
    let contents = prettyplease::unparse(&ast);

    // Save to file
    let dest_path = Path::new(&env::var(OUT_DIR).unwrap()).join(TARGET_OUT_FILE);
    fs::write(dest_path, contents).unwrap();

    // Set file name to an environment variable, for inclusion in the to-be compiled library
    println!("cargo:rustc-env={TARGET_ENV_VAR}={TARGET_OUT_FILE}");
}

fn format_name_postfix(value: &str, uid: usize, needs_postfix: bool) -> String {
    #[allow(clippy::collapsible_str_replace)]
    let intermediate: String = value
        // Remove / replace the following characters
        .replace('-', "")
        .replace('/', "")
        .replace('.', "_")
        .replace(',', "_")
        .replace('\'', "")
        .replace('#', "")
        .replace("&quot;", "")
        .replace("&amp;", "")
        .replace(';', "")
        .replace(':', "")
        .replace('(', "_")
        .replace(')', "_")
        .replace('{', "_")
        .replace('}', "_")
        .replace('=', "_")
        .replace('–', "_")
        .replace('+', "plus")
        .replace('%', "pct")
        // Split by white space (1), capitalize each substring (2), then merge (3).
        // Example procedure for "Life form":
        // 1 | Split      : ["Life", "form"]
        // 2 | Capitalize : ["Life", "Form"]
        // 3 | Merge      : "LifeForm"
        .split(' ')
        .map(|string| {
            let mut chars = string.chars();
            match chars.next() {
                // Empty string
                None => String::new(),
                // Uppercase character and concatenate
                Some(char) => format!("{}{}", char.to_uppercase(), chars.as_str()),
            }
        })
        .map(|string| string.trim_end_matches('_').to_string()) // trim trailing underscores
        .collect();

    // Prefix values starting with a digit with '_'
    // FIXME .unwrap_or('x') is a hack to fail when `intermediate` is empty. is_some_and() is unstable at this time.
    let starts_with_digit = intermediate.chars().next().unwrap_or('x').is_ascii_digit();
    let is_empty = intermediate.is_empty();

    let prefix = String::from(if starts_with_digit { "_" } else { "" });
    let name = if is_empty {
        String::from("Unnamed")
    } else {
        intermediate
    };
    let postfix = if needs_postfix {
        format!("_{uid}")
    } else {
        String::new()
    };
    let intermediate = [prefix, name, postfix].join("");

    // When there are multiple parenthesis sections, replace them with '_' (such as Countries)
    intermediate.replace("__", "_")
}

fn format_name(value: &str, uid: usize) -> String {
    format_name_postfix(value, uid, false)
}

fn format_field_name(name: &str) -> String {
    #[allow(clippy::collapsible_str_replace)]
    name.to_lowercase()
        .replace(" / ", "_")
        .replace(' ', "_")
        .replace('-', "")
        .replace('/', "_")
        .replace('#', "")
        .replace('(', "")
        .replace(')', "")
        .replace('=', "_")
        .replace('\'', "")
}

fn escape_description(description: &str) -> String {
    description.replace('{', "{{").replace('}', "}}")
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_format_name() {}
}
