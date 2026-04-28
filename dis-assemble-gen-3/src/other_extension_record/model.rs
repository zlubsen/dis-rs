#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub struct Other {
    pub body: Vec<u8>,
}

impl Other {
    #[expect(
        clippy::cast_possible_truncation,
        reason = "MTU of PDUs and Records is well within u16::MAX"
    )]
    pub fn record_length(&self) -> u16 {
        self.body.len() as u16
    }
}
