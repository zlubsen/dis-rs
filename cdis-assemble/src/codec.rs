pub trait Codec {
    /// The DIS PDU, Body, Record, ... that is to be converted.
    type Counterpart;

    fn encode(item: Self::Counterpart) -> Self;
    fn decode(&self) -> Self::Counterpart;
}