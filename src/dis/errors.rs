pub enum DisError {
    InsufficientHeaderLength(usize, usize), // the buffer is too small to contain a valid DIS header; (usize expected, usize found)
    InsufficientPduLength(usize, usize), // the buffer is too small to contain a valid DIS Pdu based on the header; (usize expected, usize found)
    MalformedPdu,
    InvalidProtocolVersionValue(u8),
    InvalidPduTypeValue(u8),
    InvalidProtocolFamilyValue(u8),
    InvalidEnumValue(usize, usize), // a field contains an enum value which is outside of the spec for that field; (usize max allowed, usize found)
}