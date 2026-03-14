use quick_xml::events::attributes::Attribute;
use quick_xml::events::BytesStart;
use quick_xml::name::QName;
use quick_xml::reader::Reader;
use std::fs::File;
use std::io::BufReader;
use std::str::FromStr;

/// Helper function to extract an attribute from `element` as `String`
#[inline]
#[must_use]
pub fn extract_attr_as_string(element: &BytesStart, attr_name: QName) -> Option<String> {
    if let Ok(Some(attr)) = element.try_get_attribute(attr_name) {
        Some(convert_attr_to_string(&attr))
    } else {
        None
    }
}

/// Helper function to extract an attribute from `element` as `usize`
#[inline]
#[must_use]
pub fn extract_attr_as_usize(
    element: &BytesStart,
    attr_name: QName,
    reader: &Reader<BufReader<File>>,
) -> Option<usize> {
    if let Ok(Some(attr)) = element.try_get_attribute(attr_name) {
        Some(convert_attr_to_usize(reader, &attr))
    } else {
        None
    }
}

/// Helper function to extract an attribute from `element` containing one ore more UID values as 'String'
#[must_use]
pub fn extract_attr_as_enum_uid(element: &BytesStart, attr_name: QName) -> Option<Vec<usize>> {
    if let Ok(Some(attr)) = element.try_get_attribute(attr_name) {
        expand_uid_string(&convert_attr_to_string(&attr)).ok()
    } else {
        None
    }
}

/// Helper function to extract an attribute from `element` as `bool`
#[inline]
pub fn extract_attr_as_bool(
    element: &BytesStart,
    attr_name: QName,
    reader: &mut Reader<BufReader<File>>,
) -> Option<bool> {
    if let Ok(Some(attr)) = element.try_get_attribute(attr_name) {
        Some(convert_attr_to_bool(reader, &attr))
    } else {
        None
    }
}

/// Helper function to convert an Attribute value to `String`
#[inline]
fn convert_attr_to_string(attr: &Attribute) -> String {
    String::from_utf8(attr.value.to_vec()).expect("Expected valid UTF-8")
}

/// Helper function to convert an Attribute value to `usize`
///
/// # Panics
/// The function panics when the decoded attribute value cannot be parsed as valid UTF-8.
#[inline]
#[must_use]
pub fn convert_attr_to_usize(reader: &Reader<BufReader<File>>, attr: &Attribute) -> usize {
    usize::from_str(
        &reader
            .decoder()
            .decode(&attr.value)
            .expect("Expected valid UTF-8"),
    )
    .expect("Expected a value able to be parsed to 'usize'")
}

/// Helper function to extract an Attribute value as a `bool`
///
/// # Panics
/// The function panics when the decoded attribute value cannot be parsed as a `bool`.
#[inline]
#[must_use]
fn convert_attr_to_bool(reader: &Reader<BufReader<File>>, attr: &Attribute) -> bool {
    bool::from_str(
        &reader
            .decoder()
            .decode(&attr.value)
            .expect("Expected valid UTF-8"),
    )
    .expect("Expected a value able to be parsed to 'bool'")
}

/// Helper function to expand a list of UIDs in &str format,
/// consisting of single items and ranges, into a Vec<usize>.
///
/// Returns the enum IDs as defined by the `uid_string` format
/// contained in an `Option::Some`, and `Option::None` when
/// the value of `uid_string` equals `"None"` (as the attribute can be optional).
///
/// Example: `"1, 4, 6-8"` would expand to `1, 4, 6, 7, 8`.
///
/// # Errors
/// The function return `Error(())` when the attribute does not contain a list of UID values.
/// Values occurring in the definition XML files are `None`, `EREF`, `TBD` or an empty attribute.
///
/// # Panics
/// The function panics when the UID values cannot be parsed into valid `usize` values.
#[allow(clippy::result_unit_err)]
fn expand_uid_string(uid_string: &str) -> Result<Vec<usize>, ()> {
    const NONE_STRING: &str = "None";
    const EREF_STRING: &str = "EREF"; // TODO figure out what this value means
    const TBD_STRING: &str = "TBD"; // FIXME remove when the standard has settled all UIDs
    if [NONE_STRING, EREF_STRING, TBD_STRING].contains(&uid_string) || uid_string.is_empty() {
        return Err(());
    }

    Ok(uid_string
        .split(',')
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .flat_map(|part| {
            if let Some((start_str, end_str)) = part.split_once('-') {
                // Parse range
                let start: usize = start_str
                    .trim()
                    .parse()
                    .unwrap_or_else(|_| panic!("Invalid range start: '{start_str}'"));

                let end: usize = end_str
                    .trim()
                    .parse()
                    .unwrap_or_else(|_| panic!("Invalid range end: '{end_str}'"));

                assert!(
                    start <= end,
                    "{}",
                    format!("Invalid range '{part}': start > end")
                );
                (start..=end).collect::<Vec<usize>>()
            } else {
                // Parse single number
                let number: usize = part
                    .parse()
                    .unwrap_or_else(|_| panic!("Invalid number: '{part}'"));

                vec![number]
            }
        })
        .collect())
}

/// Maps DIS schema primitive enumeration field types to Rust primitive types
/// (equal to the data size of the DIS Enum - 8, 16 or 32 bits).
///
/// Valid types are `enum8`, `enum16`, and `enum32`.
///
/// # Errors
/// Returns an `Error(())` when the type argument `ty` does not match a valid DIS enum type.
#[allow(clippy::result_unit_err)]
pub fn enum_type_to_field_type(ty: &str) -> Result<String, ()> {
    let field_type = match ty {
        "enum8" => Ok("u8"),
        "enum16" => Ok("u16"),
        "enum32" => Ok("u32"),
        _ => Err(()),
    };

    field_type.map(ToString::to_string)
}

/// Formats the name of a PDU or Record from the defined format into the code representation
/// The basic approach is to remove non-alphabetic characters and whitespace.
///
/// Examples:
/// - Create Entity -> `CreateEntity`
/// - Electromagnetic Emission -> `ElectromagneticEmission`
/// - Start/Resume -> `StartResume`
#[must_use]
pub fn format_type_name(name: &str) -> String {
    let name = move_non_alpha_prefix_to_suffix(name);
    name.replace(['/', '-', ' '], "")
}

/// Formats the name of a field into the code representation
/// The basic approach is to remove non-alphabetic characters and convert `CamelCase` to `snake_case`.
#[must_use]
pub fn format_field_name(name: &str) -> String {
    let name = move_non_alpha_prefix_to_suffix(name);
    let name = replace_rust_keywords(name.to_lowercase().as_str());
    name.replace(['/', '(', ')'], " ").replace([' ', '-'], "_")
}

/// Formats the name for a PDU module. PDU module names are in `snake_case`.
#[must_use]
#[inline]
pub fn format_pdu_module_name(name: &str) -> String {
    name.replace(['/', '-'], "")
        .replace(' ', "_")
        .to_lowercase()
}

/// Transforms the name of a type such that there are no leading non-alphanumerical characters,
/// by moving the (hard-coded) prefix to the end of the type name.
///
/// Supported prefixes:
/// - `2D`
///
/// Example:
/// - `2DWindSample` => `WindSample2D`
#[inline]
fn move_non_alpha_prefix_to_suffix(name: &str) -> String {
    const STRING_2D: &str = "2D";
    if name.starts_with(STRING_2D) {
        let mut stripped = name
            .strip_prefix(STRING_2D)
            .expect("Prefix is checked beforehand.")
            .trim()
            .to_string();
        stripped.push(' ');
        stripped.push_str(STRING_2D);
        stripped
    } else {
        name.to_string()
    }
}

/// Alters (field) names that would result in compiler errors due to name clashes with Rust keywords.
/// Occurrences are prefixed with `field_`.
///
/// `name` is expected be formatted as a type name (CamelCase containing capitals and whitespace).
///
/// Supported keywords:
/// - `type`
#[inline]
fn replace_rust_keywords(name: &str) -> String {
    if ["type"].contains(&name) {
        let mut field_name = "field_".to_string();
        field_name.push_str(name);
        field_name
    } else {
        name.to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::{expand_uid_string, format_field_name, format_type_name};

    #[test]
    fn test_format_pdu_name() {
        // format PDUs
        assert_eq!(format_type_name("Create Entity"), "CreateEntity");
        assert_eq!(
            format_type_name("Electromagnetic Emission"),
            "ElectromagneticEmission"
        );
        // having a non-alphabetic character
        assert_eq!(format_type_name("Start/Resume"), "StartResume");
        // and some Records
        assert_eq!(format_type_name("Euler Angles"), "EulerAngles");
        assert_eq!(format_type_name("Entity Location"), "EntityLocation");
    }

    #[test]
    fn test_format_field_name() {
        assert_eq!(format_field_name("Entity Location"), "entity_location");
        assert_eq!(format_field_name("Ez"), "ez");
        assert_eq!(
            format_field_name("Emitter Name (Jammer)"),
            "emitter_name_jammer"
        );
        assert_eq!(format_field_name("X-coordinate"), "x_coordinate");
    }

    #[test]
    fn move_non_alpha_prefix_to_suffix() {
        let type_name = format_type_name("2D Wind Sample");
        let field_name = format_field_name("2D Wind Sample");

        assert_eq!(type_name.as_str(), "WindSample2D");
        assert_eq!(field_name.as_str(), "wind_sample_2d");
    }

    #[test]
    fn replace_rust_keywords() {
        let field_name = format_field_name("Type");

        assert_eq!(field_name.as_str(), "field_type");
    }

    #[test]
    fn test_expand_uid_string() {
        let single_uid = expand_uid_string("8");
        let multiple_uid = expand_uid_string("6, 7");
        let range_uid = expand_uid_string("10-12");
        let both = expand_uid_string("8, 10-12");

        assert_eq!(single_uid, Ok(vec![8]));
        assert_eq!(multiple_uid, Ok(vec![6, 7]));
        assert_eq!(range_uid, Ok(vec![10, 11, 12]));
        assert_eq!(both, Ok(vec![8, 10, 11, 12]));
    }

    #[test]
    fn test_expand_uid_string_errors() {
        let empty = expand_uid_string("");
        let none = expand_uid_string("None");
        let eref = expand_uid_string("EREF");
        let tbd = expand_uid_string("TBD");

        assert_eq!(empty, Err(()));
        assert_eq!(none, Err(()));
        assert_eq!(eref, Err(()));
        assert_eq!(tbd, Err(()));
    }
}
