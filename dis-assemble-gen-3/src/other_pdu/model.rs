use crate::enumerations::{DISPDUType, DISProtocolFamily};
use crate::{BodyRaw, PduBody};
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
    #[must_use]
    pub fn protocol_family(&self) -> DISProtocolFamily {
        DISProtocolFamily::Other
    }
}

impl BodyRaw for Other {
    type Builder = ();

    fn builder() -> Self::Builder {
        todo!()
    }

    fn into_builder(self) -> Self::Builder {
        todo!()
    }

    fn into_pdu_body(self) -> PduBody {
        todo!()
    }

    fn body_length(&self) -> u16 {
        self.body.len() as u16
    }

    fn body_type(&self) -> DISPDUType {
        DISPDUType::Other
    }
}
