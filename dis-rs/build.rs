use std::{env, fs};
use std::ops::{RangeInclusive};
use std::path::Path;

use quick_xml::Reader;
use proc_macro2::{Ident, Literal, TokenStream};

const SISO_REF_FILE : &str = "./enumerations/SISO-REF-010.xml";

/// Array containing all the uids of enumerations that should be generated.
/// Each entry is a tuple containing:
/// - the uid,
/// - an Optional string literal to override the name of the resulting enum,
/// - an Optional data size (in bits),
/// - a bool flag to indicate that the value of an enum item must be appended to the name.
///
/// For example, the 'DISPDUType' enum (having uid 4) has an override
/// to 'PduType', which is nicer in code. The entry thus is (4, Some("PduType"), None)
///
/// Also, the 'Articulated Parts-Type Metric' enum has a defined size of 5,
/// but needs to be aligned with a 32-bit field.
///
/// Finally, some enums have variants that result in empty names (`""`) or duplicate names (such as 'Emitter Name').
/// The bool flag will append `"_value"` to the name of the variant to make it unique
const ENUM_UIDS: [(usize, Option<&str>, Option<usize>, bool); 152] = [
    (3, Some("ProtocolVersion"), None, false),   // Protocol Version
    (4, Some("PduType"), None, false),           // PDU Type
    (5, Some("ProtocolFamily"), None, false),    // PDU Family
    (6, Some("ForceId"), None, false), // Force Id
    (7, None, None, false), // Entity Kind
    (8, None, None, false), // Domain
    // 9-28 // (Sub-)Categories
    (29, None, None, false), // Country
    // 30 // Entity Types records
    // 31-43 // Bitfields, see `BITFIELD_UIDS`
    (44, None, None, false), // Dead Reckoning Algorithm
    (45, None, None, false), // Entity Marking Character Set
    // 46-54 do not exist
    (55, None, None, false), // Entity Capabilities (together with bitfields 450-462)
    (56, None, None, false), // Variable Parameter Record Type
    (57, None, None, false), // Attached Parts
    (58, None, Some(32), false), // Articulated Parts-Type Metric
    (59, None, None, false), // Articulated Parts-Type Class
    (60, None, None, false), // Munition Descriptor-Warhead
    (61, None, None, true), // Munition Descriptor-Fuse
    (62, None, None, false), // Detonation result
    (63, None, None, false), // Service Type Requested
    (64, None, None, true), // Repair Complete-Repair
    (65, None, None, false), // Repair Complete-Repair Result
    (66, Some("VariableRecordType"), None, true), // Variable Record Types
    (67, None, None, false), // Stop/Freeze Reason
    (69, Some("AcknowledgeFlag"), None, false), // Acknowledge-Acknowledge Flag
    (70, Some("ResponseFlag"), None, false), // Acknowledge-Response Flag
    (71, Some("ActionId"), None, false), // Action Request-Action ID
    (72, Some("RequestStatus"), None, false), // Action Request-Request Status
    (73, Some("EventType"), None, false), // Event Report-Event Type
    (74, None, None, false), // Required Reliability Service
    (75, None, None, true), // Emitter Name
    (76, None, None, true), // Emitter System Function
    (77, None, None, false), // Electromagnetic Emission-State Update Indicator
    (78, None, None, false), // Electromagnetic Emission-Beam Function
    (79, None, None, false), // High Density Track/Jam
    (80, None, None, false), // Designator System Name
    (81, Some("DesignatorCode"), None, false), // Designator Code
    (82, Some("IffSystemType"), None, false), // IFF-System Type
    (83, Some("IffSystemName"), None, false), // IFF-System Name
    (84, Some("IffSystemMode"), None, false), // IFF-System Mode
    // 87, 96-98 // IFF stuff
    // 100-106, // Subcategories
    (143, None, None, false), // UA-State/Change Update Indicator
    (144, None, None, false), // UA-Acoustic System Name
    (145, None, None, false), // UA-Acoustic Emitter System Function
    (146, None, None, false), // UA-Active Emission Parameter Index
    (147, None, None, false), // UA-Scan Pattern
    (148, None, None, false), // UA-Passive Parameter Index
    (150, None, None, false), // UA-Additional Passive Activity Parameter Index
    (155, None, None, false), // Transmitter Major Modulation
    (156, None, None, false), // Transmitter-Detail-Amplitude Modulation
    (157, Some("TransmitterDetailAmplitudeAngleModulation"), None, false), // Transmitter-Detail-Amplitude and Angle Modulation
    (158, Some("TransmitterDetailAngleModulation"), None, false), // Transmitter-Detail-Angle modulation
    (159, None, None, false), // Transmitter-Detail-Combination Modulation
    (160, None, None, false), // Transmitter-Detail-Pulse Modulation
    (161, None, None, false), // Transmitter-Detail-Unmodulated Modulation
    (162, None, None, false), // Transmitter-Detail-Carrier Phase Shift Modulation
    (163, None, None, false), // Transmitter-Modulation Type System
    (164, None, None, false), // Transmitter Transmit State
    (165, None, None, false), // Transmitter Input Source
    (166, None, None, false), // Transmitter Crypto System
    (167, None, None, false), // Transmitter Antenna Pattern Type
    (168, None, None, false), // Transmitter Antenna Pattern Reference System
    (177, None, None, false), // Signal User Protocol Identification Number
    (178, Some("SignalTdlType"), None, true), // Signal TDL Type
    (179, Some("ReceiverState"), None, false), // Receiver Receiver State
    (189, None, None, false), // Collision Type
    (204, None, None, false), // Aggregate State-Aggregate State
    (205, None, None, false), // Aggregate State-Formation
    (206, None, None, false), // Aggregate State-Aggregate Kind
    // (207, None, None, false), // Aggregate State-Aggregate Types -- not supported
    (208, None, None, false), // Aggregate State-Subcategory
    (209, None, None, false), // Aggregate State-Specific
    (210, None, None, false), // IsPartOf-Nature
    (211, None, None, false), // IsPartOf-Position
    (212, Some("StationName"), None, false), // IsPartOf-Station Name
    (213, None, None, false), // IsGroupOf-Grouped Entity Category
    (224, None, None, true), // Transfer Control-Transfer Type
    (270, None, Some(16), false), // Signal Encoding Class
    (271, None, Some(16), true), // Signal Encoding Type
    (281, Some("APAStatus"), None, false), // APA Parameter Index-APA Status
    (282, Some("SeparationReasonForSeparation"), None, false), // Separation VP-Reason for Separation
    (283, Some("SeparationPreEntityIndicator"), None, false), // Separation VP-Pre-Entity Indicator
    (295, Some("AttributeActionCode"), None, false), // Attribute Action Code
    (296, Some("DrParametersType"), None, false), // Dead Reckoning Parameters Type
    (301, Some("TransferredEntityIndicator"), None, false), // DIS-PDU Status-Transferred Entity Indicator (TEI)
    (302, Some("LvcIndicator"), None, false), // DIS-PDU Status-LVC Indicator (LVC)
    (303, Some("CoupledExtensionIndicator"), None, false), // DIS-PDU Status-Coupled Extension Indicator (CEI)
    (304, Some("FireTypeIndicator"), None, false), // DIS-PDU Status-Fire Type Indicator (FTI)
    (305, Some("DetonationTypeIndicator"), None, false), // DIS-PDU Status-Detonation Type Indicator (DTI)
    (306, Some("RadioAttachedIndicator"), None, false), // Radio Attached Indicator
    (307, Some("IntercomAttachedIndicator"), None, false), // DIS-PDU Status-Intercom Attached Indicator (IAI)
    (308, Some("IffSimulationMode"), None, false), // DIS-PDU Status-IFF Simulation Mode (ISM)
    (310, None, None, false), // Explosive Material Categories
    (318, None, None, false), // Beam Status-Beam State
    (319, None, None, false), // Entity Association-Association Status
    (320, Some("ChangeIndicator"), None, false), // Entity VP Record-Change Indicator
    (321, None, None, false), // Entity Association-Group Member Type
    (323, None, None, false), // Entity Association-Physical Association Type
    (324, None, None, false), // Entity Association-Physical Connection Type
    (334, None, None, false), // Record Query-R-Event Type
    (335, Some("UAPropulsionPlantConfiguration"), None, false), // UA Propulsion Plant Configuration-Configuration
    (339, Some("IffApplicableModes"), None, false), // IFF Applicable Modes
    (346, Some("Mode5IffMission"), None, false), // IFF Mission
    (347, Some("ModeSTransmitState"), Some(8), false), // Mode S Interrogator Status Transmit State
    (350, None, None, false), // Mode 5 Reply
    (351, None, None, false), // Antenna Selection
    (353, None, None, false), // Mode S Squitter Type
    (354, None, None, false), // Mode S Squitter Type
    (355, None, None, false), // Mode S Squitter Record Source
    (356, None, None, false), // Aircraft Present Domain
    (357, None, None, false), // Aircraft Identification Type
    (358, None, None, true), // Capability Report
    (359, None, None, true), // Navigation Source
    (361, Some("Mode5SAltitudeResolution"), None, false), // Mode 5/S Altitude Resolution
    (369, None, None, false), // Data Category
    (378, None, None, false), // Appearance-Paint Scheme
    (379, None, None, false), // Appearance-Damage
    (380, None, None, false), // Mode 5 Message Formats Status
    (381, None, None, false), // Appearance-Trailing Effects
    (382, None, None, false), // Appearance-Hatch
    (383, None, None, false), // Appearance-Launcher/Operational
    (384, None, None, false), // Appearance-Camouflage Type
    (385, None, None, false), // Appearance-Concealed Position
    (386, None, None, false), // Appearance-Entity or Object State
    (387, None, None, false), // Appearance-Canopy
    (388, None, None, false), // Appearance-Subsurface Hatch
    (389, Some("Active Interrogation Indicator"), None, false), // DIS-PDU Status-Active Interrogation Indicator (AII)
    (390, None, None, false), // Appearance-Lifeform Health
    (391, None, None, false), // Appearance-Life Form Compliance Status
    (392, None, None, false), // Appearance-Life Form Posture
    (393, None, None, false), // Appearance-Life Form Weapon/Implement
    (394, None, None, false), // Appearance-Concealed Movement
    (395, None, None, false), // Appearance-Environmental Density
    (396, None, None, false), // Mode 5 Platform Type
    (397, None, None, false), // Appearance-Anti-Collision Day/Night
    (398, None, None, false), // Appearance-Navigation/Position Brightness
    (399, None, None, false), // Appearance-Supply Deployed
    (400, None, None, false), // Appearance-NVG Mode
    (401, None, None, false), // Parachute
    (402, None, None, false), // Flare/Smoke Color
    (403, None, None, false), // Flare/Smoke Status
    (404, None, None, false), // Spot Chaff Status
    (405, None, None, false), // Appearance-Object General-Damage
    (406, None, None, false), // Appearance-Object General-Predistributed
    (407, None, None, false), // Appearance-Object Specific-Breach State
    (408, None, None, false), // Appearance-Object Specific-Chemical Type
    (409, None, None, false), // Appearance-Linear Object Tank Ditch Breach
    (410, None, None, false), // Appearance-Linear Object Lane Marker Visible
    (411, None, None, false), // Appearance-Object General-IED Present
    (412, None, None, false), // Mode 5 Level Selection
    (415, None, None, false), // Attached Part-Detached Indicator
    (423, None, None, false), // Mode 5 Location Errors
    (426, None, None, false), // Cover/Shroud Status
    // 427 - 448, 478 - 479 // SubCategories (EntityType)
    // 481 - 482, 505 - 527 // Specifics (EntityType)
    (589, None, None, false), // Transmitter-Detail-SATCOM-Modulation
    // 800 // Link 16 Version
    // 801 // Aircraft ID Source
    (802, None, None, false), // Clothing IR Signature
    // 803-887 // Do not exist
    (889, None, None, false), // Damage Area
];

const BITFIELD_UIDS : [RangeInclusive<usize>; 3] = [
    450..=462, // Capabilities
    // 483..=487, // Point Object Appearances
    // 488..=489, // Linear Object Appearances
    31..=43, // Appearances
    68..=68, // StopFreeze Frozen Behavior
    // 149..=149, // UA-Propulsion Plant Configuration -- does not compile as of yet
];

/// Some enums cross-reference "record" elements.
/// Such records are not generated by this script
/// and must be excluded using the `SKIP_XREF_UIDS` array.
///
/// The EnumItem::CrossRef variant that would normally be constructed
/// from the XML structure will be changed to a regular EnumItem::Basic
const SKIP_XREF_UIDS : [usize; 1] = [
    220
];

#[derive(Debug, Clone)]
pub enum GenerationItem {
    Enum(Enum),
    Bitfield(Bitfield),
}

impl <'a> GenerationItem {
    pub fn uid(&self) -> usize {
        match self {
            GenerationItem::Enum(e) => { e.uid }
            GenerationItem::Bitfield(b) => { b.uid }
        }
    }

    pub fn name(&'a self) -> &'a str {
        match self {
            GenerationItem::Enum(e) => { e.name.as_str() }
            GenerationItem::Bitfield(b) => { b.name.as_str() }
        }
    }

    pub fn size(&self) -> usize {
        match self {
            GenerationItem::Enum(e) => { e.size }
            GenerationItem::Bitfield(b) => { b.size }
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
    pub postfix_items: bool,
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
    pub deprecated: bool,
}

#[derive(Debug, Clone)]
pub struct RangeEnumItem {
    pub description: String,
    pub range: RangeInclusive<usize>,
    pub deprecated: bool,
}

#[derive(Debug, Clone)]
pub struct CrossRefEnumItem {
    pub description: String,
    pub value: usize,
    pub xref: usize,
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
}

fn main() {
    let mut reader = Reader::from_file(
        Path::new(SISO_REF_FILE)
    ).unwrap();
    reader.trim_text(true);

    // Extract enums and bitfields from the source file
    let generation_items = extraction::extract(&mut reader);

    // Generate all code for enums
    let generated = generation::generate(&generation_items);

    // format generated code using prettyplease
    let ast = syn::parse_file(&generated.to_string()).expect("Error parsing generated code for pretty printing.");
    let contents = prettyplease::unparse(&ast);

    // Save to file
    let dest_path = Path::new(&env::var("OUT_DIR").unwrap()).join("enumerations.rs");
    fs::write(
        &dest_path,
        contents
    ).unwrap();
}

fn format_name_postfix(value: &str, uid: usize, needs_postfix: bool) -> String {
    // Remove / replace the following characters
    #[allow(clippy::collapsible_str_replace)]
    let intermediate = value
        .replace(' ', "")
        .replace('-', "")
        .replace('/', "")
        .replace('.', "_")
        .replace(',', "_")
        .replace('\'', "")
        .replace('#', "")
        .replace("&quot;", "")
        .replace("&amp;", "")
        .replace(';', "")
        .replace('(', "_")
        .replace(')', "_")
        .replace('=', "_");

    // Prefix values starting with a digit with '_'
    // .unwrap_or('x') is a hack to fail when `intermediate` is empty. is_some_and() is unstable at this time.
    let starts_with_digit = intermediate.chars().next().unwrap_or('x').is_ascii_digit();
    let is_empty = intermediate.is_empty();

    let prefix = String::from(if starts_with_digit { "_" } else { "" });
    let name = is_empty.then(|| String::from("Unnamed")).unwrap_or(intermediate);
    let postfix = needs_postfix.then(|| format!("_{}", uid)).unwrap_or(String::new());
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
        .replace('-',"")
        .replace('/',"_")
        .replace('#',"")
        .replace('(', "")
        .replace(')', "")
        .replace('=', "_")
        .replace('\'', "")
}

mod extraction {
    use std::fs::File;
    use std::io::BufReader;
    use std::ops::RangeInclusive;
    use std::str::FromStr;
    use quick_xml::events::{BytesStart, Event};
    use quick_xml::name::QName;
    use quick_xml::Reader;
    use crate::{BasicEnumItem, Bitfield, BITFIELD_UIDS, BitfieldItem, CrossRefEnumItem, Enum, ENUM_UIDS, EnumItem, GenerationItem, RangeEnumItem, SKIP_XREF_UIDS};

    const ENUM_ELEMENT: QName = QName(b"enum");
    const ELEMENT_ATTR_UID: QName = QName(b"uid");
    const ELEMENT_ATTR_NAME: QName = QName(b"name");
    const ELEMENT_ATTR_SIZE: QName = QName(b"size");
    const ENUM_ROW_ELEMENT: QName = QName(b"enumrow");
    const ENUM_ROW_RANGE_ELEMENT : QName = QName(b"enumrow_range");
    const ENUM_ROW_ATTR_VALUE : QName = QName(b"value");
    const ENUM_ROW_ATTR_VALUE_MIN : QName = QName(b"value_min");
    const ENUM_ROW_ATTR_VALUE_MAX : QName = QName(b"value_max");
    const ENUM_ROW_ATTR_DESC : QName = QName(b"description");
    const ENUM_ROW_ATTR_XREF : QName = QName(b"xref");
    const ENUM_ROW_ATTR_DEPR : QName = QName(b"deprecated");
    const BITFIELD_ELEMENT : QName = QName(b"bitfield");
    const BITFIELD_ROW_ELEMENT : QName = QName(b"bitfieldrow");
    const BITFIELD_ROW_ATTR_NAME : QName = QName(b"name");
    const BITFIELD_ROW_ATTR_BIT_POSITION : QName = QName(b"bit_position");
    const BITFIELD_ROW_ATTR_LENGTH : QName = QName(b"length");
    const BITFIELD_ROW_ATTR_XREF : QName = QName(b"xref");

    pub fn extract(reader: &mut Reader<BufReader<File>>) -> Vec<GenerationItem> {
        let mut buf = Vec::new();
        let mut items = Vec::new();
        let mut current_item = None;

        // find all enumerations that we want to generate
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref element)) => {
                    match element.name() {
                        ENUM_ELEMENT => {
                            current_item = if let Ok(extracted) = extract_enum(element, reader) {
                                Some(GenerationItem::Enum(extracted))
                            } else { None }
                        },
                        ENUM_ROW_ELEMENT => {
                            current_item = if let (Some(GenerationItem::Enum(mut current)), Ok(item)) = (current_item, extract_enum_item(element, reader)) {
                                current.items.push(item);
                                Some(GenerationItem::Enum(current))
                            } else { None };
                        },
                        ENUM_ROW_RANGE_ELEMENT => {
                            current_item = if let (Some(GenerationItem::Enum(mut current)), Ok(item)) = (current_item, extract_enum_range_item(element, reader)) {
                                current.items.push(item);
                                Some(GenerationItem::Enum(current))
                            } else { None };
                        },
                        BITFIELD_ELEMENT => {
                            current_item = if let Ok(extracted) = extract_bitfield(element, reader) {
                                Some(GenerationItem::Bitfield(extracted))
                            } else { None }
                        },
                        BITFIELD_ROW_ELEMENT => {
                            current_item = if let (Some(GenerationItem::Bitfield(mut current)), Ok(item)) = (current_item, extract_bitfield_item(element, reader)) {
                                current.fields.push(item);
                                Some(GenerationItem::Bitfield(current))
                            } else { None }
                        }
                        _ => (),
                    }
                }
                Ok(Event::End(ref element)) => {
                    match element.name() {
                        ENUM_ELEMENT | BITFIELD_ELEMENT => {
                            // finish up the current enum element
                            if let Some(current) = current_item {
                                items.push(current.clone());
                            }
                            current_item = None
                        },
                        _ => (),
                    }
                }
                Ok(Event::Empty(ref element)) => {
                    match element.name() {
                        ENUM_ROW_ELEMENT => {
                            current_item = if let (Some(GenerationItem::Enum(mut current)), Ok(item)) = (current_item, extract_enum_item(element, reader)) {
                                current.items.push(item);
                                Some(GenerationItem::Enum(current))
                            } else { None };
                        },
                        ENUM_ROW_RANGE_ELEMENT => {
                            current_item = if let (Some(GenerationItem::Enum(mut current)), Ok(item)) = (current_item, extract_enum_range_item(element, reader)) {
                                current.items.push(item);
                                Some(GenerationItem::Enum(current))
                            } else { None };
                        },
                        BITFIELD_ROW_ELEMENT => {
                            current_item = if let (Some(GenerationItem::Bitfield(mut current)), Ok(item)) = (current_item, extract_bitfield_item(element, reader)) {
                                current.fields.push(item);
                                Some(GenerationItem::Bitfield(current))
                            } else { None }
                        }
                        _ => (),
                    }
                }
                Ok(Event::Eof) => break, // exits the loop when reaching end of file
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (), // There are several other `Event`s we do not consider here
            }
        }
        items
    }

    fn extract_enum(element: &BytesStart, reader: &Reader<BufReader<File>>) -> Result<Enum, ()> {
        let uid = if let Ok(Some(attr_uid)) = element.try_get_attribute(ELEMENT_ATTR_UID) {
            Some(usize::from_str(&reader.decoder().decode(&attr_uid.value).unwrap()).unwrap())
        } else { None };
        let should_generate = ENUM_UIDS.iter().find(|&&tuple| tuple.0 == uid.unwrap());
        if should_generate.is_none() {
            // skip this enum, not to be generated
            return Err(());
        }
        let name_override = should_generate.unwrap().1;
        let size_override = should_generate.unwrap().2;
        let postfix_items = should_generate.unwrap().3;

        let name = if let Ok(Some(attr_name)) = element.try_get_attribute(ELEMENT_ATTR_NAME) {
            if let Some(name) = name_override {
                Some(name.to_string())
            } else {
                Some(String::from_utf8(attr_name.value.to_vec()).unwrap())
            }
        } else { None };

        let size = if let Ok(Some(attr_size)) = element.try_get_attribute(ELEMENT_ATTR_SIZE) {
            if let Some(size) = size_override {
                Some(size)
            } else {
                Some(usize::from_str(&reader.decoder().decode(&attr_size.value).unwrap()).unwrap())
            }
        } else { None };

        if let (Some(uid), Some(name), Some(size)) = (uid, name, size) {
            Ok(Enum {
                uid,
                name,
                size,
                items: vec![],
                postfix_items
            })
        } else {
            // something is wrong with the attributes of the element, skip it.
            Err(())
        }
    }

    fn extract_enum_item(element: &BytesStart, reader: &Reader<BufReader<File>>) -> Result<EnumItem, ()> {
        let value = if let Ok(Some(attr_value)) = element.try_get_attribute(ENUM_ROW_ATTR_VALUE) {
            Some(usize::from_str(&reader.decoder().decode(&attr_value.value).unwrap()).unwrap())
        } else { None };
        let description = if let Ok(Some(attr_desc)) = element.try_get_attribute(ENUM_ROW_ATTR_DESC) {
            Some(String::from_utf8(attr_desc.value.to_vec()).unwrap())
        } else { None };
        let xref = if let Ok(Some(attr_xref)) = element.try_get_attribute(ENUM_ROW_ATTR_XREF) {
            let xref_value = usize::from_str(&reader.decoder().decode(&attr_xref.value).unwrap()).unwrap();
            if SKIP_XREF_UIDS.contains(&xref_value) {
                None
            } else { Some(xref_value) }
        } else { None };
        let deprecated = matches!(element.try_get_attribute(ENUM_ROW_ATTR_DEPR), Ok(Some(_attr_depr)));

        match (value, description, xref) {
            (Some(value), Some(description), Some(xref)) => {
                Ok(EnumItem::CrossRef(CrossRefEnumItem {
                    description,
                    value,
                    xref,
                    deprecated
                }))
            }
            (Some(value), Some(description), None) => {
                Ok(EnumItem::Basic(BasicEnumItem {
                    description,
                    value,
                    deprecated
                }))
            }
            _ => {
                // something is wrong with the attributes of the element, skip it.
                Err(())
            }
        }
    }

    fn extract_enum_range_item(element: &BytesStart, reader: &Reader<BufReader<File>>) -> Result<EnumItem, ()> {
        let value_min = if let Ok(Some(attr_value)) = element.try_get_attribute(ENUM_ROW_ATTR_VALUE_MIN) {
            Some(usize::from_str(&reader.decoder().decode(&attr_value.value).unwrap()).unwrap())
        } else { None };
        let value_max = if let Ok(Some(attr_value)) = element.try_get_attribute(ENUM_ROW_ATTR_VALUE_MAX) {
            Some(usize::from_str(&reader.decoder().decode(&attr_value.value).unwrap()).unwrap())
        } else { None };
        let description = if let Ok(Some(attr_desc)) = element.try_get_attribute(ENUM_ROW_ATTR_DESC) {
            Some(String::from_utf8(attr_desc.value.to_vec()).unwrap())
        } else { None };
        let deprecated = matches!(element.try_get_attribute(ENUM_ROW_ATTR_DEPR), Ok(Some(_attr_depr)));

        if let (Some(value_min), Some(value_max), Some(description)) = (value_min, value_max, description) {
            Ok(EnumItem::Range(RangeEnumItem {
                description,
                range: RangeInclusive::new(value_min, value_max),
                deprecated
            }))
        } else {
            // something is wrong with the attributes of the element, skip it.
            Err(())
        }
    }

    fn extract_bitfield(element: &BytesStart, reader: &Reader<BufReader<File>>) -> Result<Bitfield, ()> {
        let uid = if let Ok(Some(attr_uid)) = element.try_get_attribute(ELEMENT_ATTR_UID) {
            Some(usize::from_str(&reader.decoder().decode(&attr_uid.value).unwrap()).unwrap())
        } else { None };
        if let Some(uid) = uid {
            if !BITFIELD_UIDS.iter().any(|range| range.contains(&uid)) {
                // uid is not in the list, skip this bitfield, not to be generated
                return Err(());
            }
        }

        let name = if let Ok(Some(attr_name)) = element.try_get_attribute(ELEMENT_ATTR_NAME) {
            Some(String::from_utf8(attr_name.value.to_vec()).unwrap())
        } else { None };
        let size = if let Ok(Some(attr_size)) = element.try_get_attribute(ELEMENT_ATTR_SIZE) {
            Some(usize::from_str(&reader.decoder().decode(&attr_size.value).unwrap()).unwrap())
        } else { None };

        if let (Some(uid), Some(name), Some(size)) = (uid, name, size) {
            Ok(Bitfield {
                uid,
                name,
                size,
                fields: vec![]
            })
        } else {
            // something is wrong with the attributes of the element, skip it.
            Err(())
        }
    }

    fn extract_bitfield_item(element: &BytesStart, reader: &Reader<BufReader<File>>) -> Result<BitfieldItem, ()> {
        let name = if let Ok(Some(attr_name)) = element.try_get_attribute(BITFIELD_ROW_ATTR_NAME) {
            Some(String::from_utf8(attr_name.value.to_vec()).unwrap())
        } else { None };
        let position = if let Ok(Some(attr_position)) = element.try_get_attribute(BITFIELD_ROW_ATTR_BIT_POSITION) {
            Some(usize::from_str(&reader.decoder().decode(&attr_position.value).unwrap()).unwrap())
        } else { None };
        let length = if let Ok(Some(attr_length)) = element.try_get_attribute(BITFIELD_ROW_ATTR_LENGTH) {
            usize::from_str(&reader.decoder().decode(&attr_length.value).unwrap()).unwrap()
        } else { 1 };
        let xref = if let Ok(Some(attr_xref)) = element.try_get_attribute(BITFIELD_ROW_ATTR_XREF) {
            Some(usize::from_str(&reader.decoder().decode(&attr_xref.value).unwrap()).unwrap())
        } else { None };

        if let (Some(name), Some(bit_position)) = (name, position) {
            Ok(BitfieldItem {
                name,
                bit_position,
                length,
                xref
            })
        } else {
            // something is wrong with the attributes of the element, skip it.
            Err(())
        }
    }
}

mod generation {
    use quote::{format_ident, quote};
    use crate::{Bitfield, BitfieldItem, Enum, EnumItem, format_field_name, format_name, format_name_postfix, GenerationItem, Ident, Literal, TokenStream};

    pub fn generate(items: &Vec<GenerationItem>) -> TokenStream {
        let mut generated_items = vec![];

        let lookup_xref = |xref:usize| {
            items.iter().find(|&it| { it.uid() == xref })
        };

        for item in items {
            match item {
                GenerationItem::Enum(e) => generated_items.push(generate_enum(e, lookup_xref)),
                GenerationItem::Bitfield(b) => generated_items.push(generate_bitfield(b, lookup_xref)),
            }
        }
        quote!(
            #[allow(clippy::identity_op)]
            #[allow(clippy::write_literal)]
            #[allow(clippy::match_single_binding)]
            pub mod enumerations {
                use std::fmt::{Display, Formatter};

                #(#generated_items)*
            }
        )
    }

    fn generate_enum<'a, F>(item: &Enum, lookup_xref: F) -> TokenStream
    where F: Fn(usize)->Option<&'a GenerationItem> {
        let formatted_name = format_name(item.name.as_str(), item.uid);
        let name_ident = format_ident!("{}", formatted_name);
        // generate enum declarations
        let decl = quote_enum_decl(item, lookup_xref);
        // generate From impls (2x)
        let from_impl = quote_enum_from_impl(item, &name_ident);
        let into_impl = quote_enum_into_impl(item, &name_ident);
        // generate Display impl
        let display_impl = quote_enum_display_impl(item, &name_ident);
        // generate Default impl
        let default_impl = quote_enum_default_impl(&name_ident);
        quote!(
            #decl

            #from_impl

            #into_impl

            #display_impl

            #default_impl

        )
    }

    fn quote_enum_decl<'a, F>(e: &Enum, lookup_xref: F) -> TokenStream
    where F: Fn(usize)->Option<&'a GenerationItem> {
        let name = format_name(e.name.as_str(), e.uid);
        let name_ident = format_ident!("{}", name);
        let arms = quote_enum_decl_arms(&e.items, e.size, e.postfix_items, lookup_xref);
        quote!(
            #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
            #[allow(non_camel_case_types)]
            pub enum #name_ident {
                #(#arms),*
            }
        )
    }

    fn quote_enum_decl_arms<'a, F>(items: &[EnumItem], data_size: usize, postfix_items: bool, lookup_xref: F) -> Vec<TokenStream>
    where F: Fn(usize)->Option<&'a GenerationItem> {
        let size_type = size_to_type(data_size);
        let size_ident = format_ident!("{}", size_type);

        let mut arms : Vec<TokenStream> = items.iter().map(|item| {
            match item {
                EnumItem::Basic(item) => {
                    let item_name = format_name_postfix(item.description.as_str(), item.value, postfix_items);
                    let item_ident = format_ident!("{}", item_name);
                    quote!(
                        #item_ident
                    )
                }
                EnumItem::Range(item) => {
                    let item_name = format_name(item.description.as_str(), *item.range.start());
                    let item_ident = format_ident!("{}", item_name);
                    quote!(
                        #item_ident(#size_ident)
                    )
                }
                EnumItem::CrossRef(item) => {
                    let item_name = format_name(item.description.as_str(), item.value);
                    let item_ident = format_ident!("{}", item_name);
                    if let Some(xref_item) = lookup_xref(item.xref) {
                        let xref_name = format_name(xref_item.name(), xref_item.size());
                        let xref_ident = format_ident!("{}", xref_name);
                        quote!(
                            #item_ident(#xref_ident)
                        )
                    } else { // cannot find reference, skip
                        quote!()
                    }
                }
            }
        }).collect();

        arms.push(quote!(
            Unspecified(#size_ident)
        ));
        arms
    }

    fn quote_enum_from_impl(e: &Enum, name_ident: &Ident) -> TokenStream {
        let arms = quote_enum_from_arms(name_ident, &e.items, e.size, e.postfix_items);
        let discriminant_type = size_to_type(e.size);
        let discriminant_ident = format_ident!("{}", discriminant_type);
        quote!(
            impl From<#discriminant_ident> for #name_ident {
                fn from(value: #discriminant_ident) -> Self {
                    match value {
                        #(#arms),*
                    }
                }
            }
        )
    }

    fn quote_enum_from_arms(name_ident: &Ident, items: &[EnumItem], data_size: usize, postfix_items: bool) -> Vec<TokenStream> {
        let mut arms: Vec<TokenStream> = items.iter().filter_map(|item| {
            match item {
                EnumItem::Basic(item) => {
                    let item_name = format_name_postfix(item.description.as_str(), item.value, postfix_items);
                    let item_ident = format_ident!("{}", item_name);
                    let discriminant_literal = discriminant_literal(item.value, data_size);
                    Some(quote!(
                        #discriminant_literal => #name_ident::#item_ident
                    ))
                }
                EnumItem::Range(item) => {
                    let item_name = format_name(item.description.as_str(), *item.range.start());
                    let item_ident = format_ident!("{}", item_name);
                    let discriminant_literal_min = discriminant_literal(*item.range.start(), data_size);
                    let discriminant_literal_max = discriminant_literal(*item.range.end(), data_size);
                    Some(quote!(
                        #discriminant_literal_min..=#discriminant_literal_max => #name_ident::#item_ident(value)
                    ))
                }
                EnumItem::CrossRef(_item) => {
                    // Manual impl, cannot be determined based on discriminant value alone (e.g., need domain enum for capabilities and appearance)
                    None
                }
            }
        }).collect();
        // For conversion from bytes to enum, add exhaustive arm resulting in the Unspecified variant of the enum
        let unspecified_ident = format_ident!("{}", "unspecified_value");
        arms.push(quote!(
            #unspecified_ident => #name_ident::Unspecified(#unspecified_ident)
        ));
        arms
    }

    fn quote_enum_into_impl(e: &Enum, name_ident: &Ident) -> TokenStream {
        let arms = quote_enum_into_arms(name_ident, &e.items, e.size, e.postfix_items);
        let discriminant_type = size_to_type(e.size);
        let discriminant_ident = format_ident!("{}", discriminant_type);
        quote!(
            impl From<#name_ident> for #discriminant_ident {
                fn from(value: #name_ident) -> Self {
                    match value {
                        #(#arms),*
                    }
                }
            }
        )
    }

    #[allow(clippy::unnecessary_filter_map)]
    fn quote_enum_into_arms(name_ident: &Ident, items: &[EnumItem], data_size: usize, postfix_items: bool) -> Vec<TokenStream> {
        let mut arms: Vec<TokenStream> = items.iter().filter_map(|item| {
            match item {
                EnumItem::Basic(item) => {
                    let item_name = format_name_postfix(item.description.as_str(), item.value, postfix_items);
                    let item_ident = format_ident!("{}", item_name);
                    let discriminant_literal = discriminant_literal(item.value, data_size);
                    Some(quote!(
                        #name_ident::#item_ident => #discriminant_literal
                    ))
                }
                EnumItem::Range(item) => {
                    let item_name = format_name(item.description.as_str(), *item.range.start());
                    let item_ident = format_ident!("{}", item_name);
                    let value_ident = format_ident!("{}", "specific_value");
                    Some(quote!(
                        #name_ident::#item_ident(#value_ident) => #value_ident
                    ))
                }
                EnumItem::CrossRef(item) => {
                    let item_name = format_name(item.description.as_str(), item.value);
                    let item_ident = format_ident!("{}", item_name);
                    let value_ident = format_ident!("{}", "contained");
                    Some(quote!(
                        #name_ident::#item_ident(#value_ident) => #value_ident.into()
                    ))
                }
            }
        }).collect();
        let unspecified_ident = format_ident!("{}", "unspecified_value");
        arms.push(quote!(
            #name_ident::Unspecified(#unspecified_ident) => #unspecified_ident
        ));
        arms
    }

    fn quote_enum_display_impl(e: &Enum, name_ident: &Ident) -> TokenStream {
        let arms = quote_enum_display_arms(&e.items, name_ident, e.postfix_items);
        quote!(
            impl Display for #name_ident {
                fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                    match self {
                        #(#arms),*
                    }
                }
            }
        )
    }

    #[allow(clippy::unnecessary_filter_map)]
    fn quote_enum_display_arms(items: &[EnumItem], name_ident: &Ident, postfix_items: bool) -> Vec<TokenStream> {
        let mut arms: Vec<TokenStream> = items.iter().filter_map(|item| {
            match item {
                EnumItem::Basic(item) => {
                    let item_description = item.description.as_str();
                    let item_name = format_name_postfix(item_description, item.value, postfix_items);
                    let item_ident = format_ident!("{}", item_name);

                    Some(quote!(
                        #name_ident::#item_ident => write!(f, #item_description)
                    ))
                }
                EnumItem::Range(item) => {
                    let item_description = item.description.as_str();
                    let item_name = format_name(item_description, *item.range.start());
                    let item_ident = format_ident!("{}", item_name);
                    let value_ident = format_ident!("{}", "specific_value");

                    Some(quote!(
                        #name_ident::#item_ident(#value_ident) => write!(f, "{} ({})", #item_description, #value_ident)
                    ))
                }
                EnumItem::CrossRef(item) => {
                    let item_description = item.description.as_str();
                    let item_name = format_name(item_description, item.value);
                    let item_ident = format_ident!("{}", item_name);
                    let _value_ident = format_ident!("{}", "contained");

                    Some(quote!(
                        // TODO Display impls for bitfield structs
                        // This is a placeholder
                        #name_ident::#item_ident(_) => write!(f, "#name_ident::#item_ident(_)",)
                        // #name_ident::#item_ident(#value_ident) => #value_ident.display()
                    ))
                }
            }
        }).collect();
        let unspecified_ident = format_ident!("{}", "unspecified_value");
        arms.push(quote!(
            #name_ident::Unspecified(#unspecified_ident) => write!(f, "Unspecified ({})", #unspecified_ident)
        ));
        arms
    }

    fn quote_enum_default_impl(name_ident: &Ident) -> TokenStream {
        quote!(
            impl Default for #name_ident {
                fn default() -> Self {
                    #name_ident::from(0)
                }
            }
        )
    }

    fn generate_bitfield<'a, F>(item: &Bitfield, lookup_xref: F) -> TokenStream
        where F: Fn(usize)->Option<&'a GenerationItem> {
        let decl = quote_bitfield_decl(item, &lookup_xref);
        let from = quote_bitfield_from_impl(item, &lookup_xref); // struct from u32
        let into = quote_bitfield_into_impl(item, &lookup_xref); // struct into u32
        // TODO let display = quote_bitfield_display_impl(item); // display values of fields or bitstring
        quote!(
            #decl

            #from

            #into
        )
    }

    fn quote_bitfield_decl<'a, F>(item: &Bitfield, lookup_xref: F) -> TokenStream
    where F: Fn(usize)->Option<&'a GenerationItem> {
        let formatted_name = format_name(item.name.as_str(), item.uid);
        let name_ident = format_ident!("{}", formatted_name);
        let fields = quote_bitfield_decl_fields(&item.fields, lookup_xref);
        quote!(
            #[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
            pub struct #name_ident {
                #(#fields),*
            }
        )
    }

    fn quote_bitfield_decl_fields<'a, F>(fields: &[BitfieldItem], lookup_xref: F) -> Vec<TokenStream>
    where F: Fn(usize)->Option<&'a GenerationItem> {
        let generated_fields: Vec<TokenStream> = fields.iter().map( |field| {
            let field_name = format_field_name(field.name.as_str());
            let field_ident = format_ident!("{}", field_name);
            let type_literal = if let Some(xref_uid) = field.xref {
                let xref = lookup_xref(xref_uid).unwrap_or_else(|| panic!("{}", xref_uid));
                format_ident!("{}", format_name(xref.name(), xref.uid()))
            } else { format_ident!("bool") };
            quote!(
                pub #field_ident : #type_literal
            )
        }).collect();
        generated_fields
    }

    fn quote_bitfield_from_impl<'a, F>(item: &Bitfield, lookup_xref: F) -> TokenStream
    where F: Fn(usize)->Option<&'a GenerationItem> {
        let formatted_name = format_name(item.name.as_str(), item.uid);
        let name_ident = format_ident!("{}", formatted_name);
        let size_type = size_to_type(item.size);
        let size_ident = format_ident!("{}", size_type);
        let field_assignments = quote_bitfield_from_fields(&item.fields, item.size, lookup_xref);
        let field_names: Vec<TokenStream> = item.fields.iter()
            .map(|field| {
                let ident = format_ident!("{}", format_field_name(field.name.as_str()));
                quote!(#ident)
            }).collect();
        quote!(
            impl From<#size_ident> for #name_ident {
                fn from(value: #size_ident) -> Self {
                    #(#field_assignments)*

                    Self {
                        #(#field_names),*
                    }
                }
            }
        )
    }

    fn quote_bitfield_from_fields<'a, F>(fields: &[BitfieldItem], data_size: usize, lookup_xref: F) -> Vec<TokenStream>
    where F: Fn(usize)->Option<&'a GenerationItem> {
        fields.iter().map(|field| {
            let field_name = format_field_name(&field.name);
            let field_ident = format_ident!("{}", field_name);
            let position_shift_literal = Literal::usize_unsuffixed(data_size - field.length - field.bit_position);
            let bitmask = Literal::usize_unsuffixed(2usize.pow(field.length as u32) - 1);
            if let Some(xref) = field.xref {
                let xref = lookup_xref(xref).unwrap();
                let xref_name = format_name(xref.name(), xref.uid());
                let xref_ident = format_ident!("{}", xref_name);
                let xref_data_size = size_to_type(xref.size());
                let xref_size_ident = format_ident!("{}", xref_data_size);
                quote!(
                    let #field_ident = #xref_ident::from(((value >> #position_shift_literal) & #bitmask) as #xref_size_ident);
                )
            } else {
                quote!(
                    let #field_ident = ((value >> #position_shift_literal) & #bitmask) != 0;
                )
            }
        }).collect()
    }

    fn quote_bitfield_into_impl<'a, F>(item: &Bitfield, lookup_xref: F) -> TokenStream
    where F: Fn(usize)->Option<&'a GenerationItem> {
        let formatted_name = format_name(item.name.as_str(), item.uid);
        let name_ident = format_ident!("{}", formatted_name);
        let size_type = size_to_type(item.size);
        let size_ident = format_ident!("{}", size_type);
        let field_assignments = quote_bitfield_into_fields(&item.fields, item.size, lookup_xref);
        let field_names: Vec<TokenStream> = item.fields.iter()
            .map(|field| {
                let ident = format_ident!("{}", format_field_name(field.name.as_str()));
                quote!(#ident)
            }).collect();
        let base_size_literal = discriminant_literal(0, item.size);
        quote!(
            impl From<#name_ident> for #size_ident {
                fn from(value: #name_ident) -> Self {
                    #(#field_assignments)*

                    #base_size_literal #( | #field_names)*
                }
            }
        )
    }

    fn quote_bitfield_into_fields<'a, F>(fields: &[BitfieldItem], data_size: usize, lookup_xref: F) -> Vec<TokenStream>
    where F: Fn(usize)->Option<&'a GenerationItem> {
        let field_size_type = size_to_type(data_size);
        let field_size_ident = format_ident!("{}", field_size_type);

        fields.iter().map(|field| {
            let field_name = format_field_name(&field.name);
            let field_ident = format_ident!("{}", field_name);
            let position_shift_literal = data_size - field.length - field.bit_position;
            if let Some(xref) = field.xref {
                let xref_size_type = size_to_type(lookup_xref(xref).unwrap().size());
                let xref_size_ident = format_ident!("{}", xref_size_type);
                quote!(
                    let #field_ident = (#xref_size_ident::from(value.#field_ident) as #field_size_ident) << #position_shift_literal;
                )
            } else {
                quote!(
                    let #field_ident = #field_size_ident::from( value.#field_ident) << #position_shift_literal;
                )
            }
        }).collect()
    }

    fn size_to_type(data_size: usize) -> &'static str {
        match data_size {
            64 => "u64",
            32 => "u32",
            16 => "u16",
            8 => "u8",
            _ => "u8",
        }
    }

    fn discriminant_literal(value: usize, data_size: usize) -> Literal {
        match data_size {
            64 => Literal::u64_suffixed(value as u64),
            32 => Literal::u32_suffixed(value as u32),
            16 => Literal::u16_suffixed(value as u16),
            8 => Literal::u8_suffixed(value as u8),
            _ => Literal::u8_suffixed(value as u8),
        }
    }
}
