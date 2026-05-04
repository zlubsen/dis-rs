/// Hardcoded values for the discriminant type values of Adaptive Records.
/// Tuple consists of ( <type of the AdaptiveRecord> , <UID of the discriminant> , <primitive type of the discriminant> )
pub(crate) const ADAPTIVE_RECORD_DISCRIMINANT_TYPES: [(&str, usize, &str); 2] = [
    ("Net ID", 590, "u8"),                                  // Net ID Type
    ("IFF Interactive Transmission Parameters", 372, "u8"), // Transmission Indicator
];

/// Tuple: ( <fixed record field type> , <parser call arguments shim> , <parser declaration arguments shim> )
pub(crate) const SHIMS_FOR_DISCRIMINANT_DEPENDENT_RECORDS: [(&str, &str, &str); 4] = [
    (
        "Entity with Extended Appearance",
        "appearance_type, extended_appearance_type",
        "appearance_type: crate::enumerations::AppearanceType, extended_appearance_type: crate::enumerations::ExtendedAppearanceType",
    ),
    (
        "Basic Multiple Entity",
        "appearance_type",
        "appearance_type: crate::enumerations::AppearanceType",
    ),
    (
        "Moving Entity",
        "appearance_type",
        "appearance_type: crate::enumerations::AppearanceType",
    ),
    (
        "Accelerating Entity",
        "appearance_type",
        "appearance_type: crate::enumerations::AppearanceType",
    ),
];

/// Overrides the generation of `Default` Trait impls for `FixedRecord` types.
/// By default, the `Default` traits are derived. If a `FixedRecord` type is present in this array
/// the implementation is not derived and can be provided manually.
pub(crate) const FIXED_RECORD_SKIP_DEFAULT_IMPL: [&str; 1] = ["PDU Header"];
