use crate::common::{Body, Interaction};
use crate::common::other::builder::OtherBuilder;
use crate::common::model::EntityId;
use crate::enumerations::PduType;

/// A PduBody implementation that contains the body of the PDU as raw bytes, in a vec.
///
/// It also extracts the (optional) originator and receiver of
/// PDUs that convey an interaction between entities (such as Fire PDU),
/// or at least can be attributed to a sending entity (such as EntityState PDU), based on the PduType.
///
/// This struct is used to provide access to the received data in not (yet) supported PDUs.
pub struct Other {
    pub originating_entity_id : Option<EntityId>,
    pub receiving_entity_id : Option<EntityId>,
    pub body: Vec<u8>,
}

impl Body for Other {
    fn body_length(&self) -> u16 {
        self.body.len() as u16
    }

    fn body_type(&self) -> PduType {
        PduType::Other
    }
}

impl Other {
    pub fn builder() -> OtherBuilder {
        OtherBuilder::new()
    }

    pub fn new(bytes: Vec<u8>) -> Self {
        Other {
            originating_entity_id: None,
            receiving_entity_id: None,
            body: bytes,
        }
    }

    pub fn new_with_origin(bytes: Vec<u8>, origin: Option<EntityId>) -> Self {
        Other {
            originating_entity_id: origin,
            receiving_entity_id: None,
            body: bytes,
        }
    }

    pub fn new_with_receiver(bytes: Vec<u8>, origin: Option<EntityId>, receiver: Option<EntityId>) -> Self {
        Other {
            originating_entity_id: origin,
            receiving_entity_id: receiver,
            body: bytes,
        }
    }
}

impl Interaction for Other {
    fn originator(&self) -> Option<&EntityId> {
        if let Some(entity) = &self.originating_entity_id {
            Some(entity)
        } else { None }
    }

    fn receiver(&self) -> Option<&EntityId> {
        if let Some(entity) = &self.receiving_entity_id {
            Some(entity)
        } else { None }
    }
}