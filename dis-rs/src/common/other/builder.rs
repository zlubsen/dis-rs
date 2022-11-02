use crate::common::model::{Pdu, PduBody};
use crate::common::model::PduHeader;
use crate::common::other::model::Other;

pub struct OtherBuilder {
    header : Option<PduHeader>,
    body : Option<Vec<u8>>,
}

impl OtherBuilder {
    pub fn new() -> OtherBuilder {
        OtherBuilder {
            header: None,
            body: None
        }
    }

    pub fn header(mut self, header: PduHeader) -> Self {
        self.header = Some(header);
        self
    }

    pub fn body(mut self, bytes : Vec<u8>) -> Self {
        self.body = Some(bytes);
        self
    }

    pub fn validate(&self) -> bool {
        self.header.is_some() && self.body.is_some()
    }

    pub fn build(self) -> Result<PduBody, ()> {
        if self.validate() {
            return Err(())
        }

        Ok(PduBody::Other(Other{ originating_entity_id: None, receiving_entity_id: None, body: self.body.expect("should be set")}))
    }

    pub fn build_with_header(self, header: PduHeader) -> Result<Pdu, ()> {
        if self.validate() {
            return Err(())
        }

        Ok(Pdu {
            header,
            body: PduBody::Other(Other{ originating_entity_id: None, receiving_entity_id: None, body: self.body.expect("should be set")}),
        })
    }
}