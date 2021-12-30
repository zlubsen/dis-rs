pub enum DisError {
    InsufficientHeaderLength(usize, usize), // the buffer is too small to contain a valid DIS header; (usize expected, usize found)
    InsufficientPduLength(usize, usize), // the buffer is too small to contain a valid DIS Pdu based on the header; (usize expected, usize found)
    MalformedPdu,
    InvalidProtocolVersionValue(u8),
    InvalidPduTypeValue(u8),
    InvalidProtocolFamilyValue(u8),
}