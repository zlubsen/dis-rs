pub mod model;
pub mod builder;
pub mod parser;

pub mod entity_state;

use bytes::BytesMut;
pub use parser::parse_peek_protocol_version;

/// Trait that implements writing the data structure to a buffer.
/// Return the number of bytes written to the buffer.
pub trait Serialize {
    fn serialize(&self, buf : &mut BytesMut) -> usize;
}