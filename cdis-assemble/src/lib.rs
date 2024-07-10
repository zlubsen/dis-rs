use thiserror::Error;
use dis_rs::enumerations::PduType;
use dis_rs::model::TimeStamp;
use crate::entity_state::model::EntityState;
use crate::records::model::{CdisHeader, CdisRecord, EntityId};
use crate::unsupported::Unsupported;

pub mod types;
pub mod records;
pub mod constants;

pub mod collision;
pub mod create_entity;
pub mod detonation;
pub mod entity_state;
pub mod fire;
pub mod remove_entity;
pub mod start_resume;
pub mod unsupported;

pub(crate) mod parsing;
pub(crate) mod writing;
pub mod codec;

pub use parsing::parse;
pub use writing::SerializeCdisPdu;
pub use writing::BitBuffer;
pub use writing::create_bit_buffer;
use crate::collision::model::Collision;
use crate::create_entity::model::CreateEntity;
use crate::detonation::model::Detonation;
use crate::fire::model::Fire;
use crate::remove_entity::model::RemoveEntity;
use crate::start_resume::model::StartResume;

pub trait BodyProperties {
    type FieldsPresent;
    type FieldsPresentOutput;
    const FIELDS_PRESENT_LENGTH: usize;
    fn fields_present_field(&self) -> Self::FieldsPresentOutput;

    fn body_length_bits(&self) -> usize;

    fn fields_present_length(&self) -> usize {
        Self::FIELDS_PRESENT_LENGTH
    }

    fn into_cdis_body(self) -> CdisBody;
}

pub trait CdisInteraction {
    fn originator(&self) -> Option<&EntityId>;
    fn receiver(&self) -> Option<&EntityId>;
}

#[derive(Clone, Debug, PartialEq)]
pub struct CdisPdu {
    pub header: CdisHeader,
    pub body: CdisBody,
}

impl CdisPdu {
    pub fn finalize_from_parts(header: CdisHeader, body: CdisBody, time_stamp: Option<impl Into<TimeStamp>>) -> Self {
        let time_stamp: TimeStamp = if let Some(time_stamp) = time_stamp {
            time_stamp.into()
        } else { header.timestamp };
        Self {
            header: CdisHeader {
                timestamp: time_stamp,
                length: (header.record_length() + body.body_length()) as u16,
                ..header
            },
            body
        }
    }

    /// Calculates the on-wire length of the C-DIS PDU in bits.
    pub fn pdu_length(&self) -> usize {
        self.header.record_length()
        + self.body.body_length()
    }
}

impl CdisInteraction for CdisPdu {
    fn originator(&self) -> Option<&EntityId> {
        self.body.originator()
    }

    fn receiver(&self) -> Option<&EntityId> {
        self.body.receiver()
    }
}

#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, PartialEq)]
pub enum CdisBody {
    Unsupported(Unsupported),
    EntityState(EntityState),
    Fire(Fire),
    Detonation(Detonation),
    Collision(Collision),
    CreateEntity(CreateEntity),
    RemoveEntity(RemoveEntity),
    StartResume(StartResume),
    StopFreeze,
    Acknowledge,
    ActionRequest,
    ActionResponse,
    DataQuery,
    SetData,
    Data,
    EventReport,
    Comment,
    ElectromagneticEmission,
    Designator,
    Transmitter,
    Signal,
    Receiver,
    Iff,
}

impl CdisBody {
    pub fn body_length(&self) -> usize {
        match self {
            CdisBody::Unsupported(_) => { 0 }
            CdisBody::EntityState(body) => { body.body_length_bits() }
            CdisBody::Fire(body) => { body.body_length_bits() }
            CdisBody::Detonation(body) => { body.body_length_bits() }
            CdisBody::Collision(body) => { body.body_length_bits() }
            CdisBody::CreateEntity(body) => { body.body_length_bits() }
            CdisBody::RemoveEntity(body) => { body.body_length_bits() }
            CdisBody::StartResume(body) => { body.body_length_bits() }
            CdisBody::StopFreeze => { 0 }
            CdisBody::Acknowledge => { 0 }
            CdisBody::ActionRequest => { 0 }
            CdisBody::ActionResponse => { 0 }
            CdisBody::DataQuery => { 0 }
            CdisBody::SetData => { 0 }
            CdisBody::Data => { 0 }
            CdisBody::EventReport => { 0 }
            CdisBody::Comment => { 0 }
            CdisBody::ElectromagneticEmission => { 0 }
            CdisBody::Designator => { 0 }
            CdisBody::Transmitter => { 0 }
            CdisBody::Signal => { 0 }
            CdisBody::Receiver => { 0 }
            CdisBody::Iff => { 0 }
        }
    }
}

impl CdisInteraction for CdisBody {
    fn originator(&self) -> Option<&EntityId> {
        match self {
            CdisBody::Unsupported(_) => { None }
            CdisBody::EntityState(body) => { body.originator() }
            CdisBody::Fire(body) => { body.originator() }
            CdisBody::Detonation(body) => { body.originator() }
            CdisBody::Collision(body) => { body.originator() }
            CdisBody::CreateEntity(body) => { body.originator() }
            CdisBody::RemoveEntity(body) => { body.originator() }
            CdisBody::StartResume(body) => { body.originator() }
            CdisBody::StopFreeze => { None }
            CdisBody::Acknowledge => { None }
            CdisBody::ActionRequest => { None }
            CdisBody::ActionResponse => { None } 
            CdisBody::DataQuery => { None } 
            CdisBody::SetData => { None } 
            CdisBody::Data => { None } 
            CdisBody::EventReport => { None } 
            CdisBody::Comment => { None } 
            CdisBody::ElectromagneticEmission => { None } 
            CdisBody::Designator => { None } 
            CdisBody::Transmitter => { None } 
            CdisBody::Signal => { None } 
            CdisBody::Receiver => { None } 
            CdisBody::Iff => { None } 
        }
    }

    fn receiver(&self) -> Option<&EntityId> {
        match self {
            CdisBody::Unsupported(_) => { None }
            CdisBody::EntityState(body) => { body.receiver() }
            CdisBody::Fire(body) => { body.receiver() }
            CdisBody::Detonation(body) => { body.receiver() }
            CdisBody::Collision(body) => { body.receiver() }
            CdisBody::CreateEntity(body) => { body.receiver() }
            CdisBody::RemoveEntity(body) => { body.receiver() }
            CdisBody::StartResume(body) => { body.receiver() }
            CdisBody::StopFreeze => { None } 
            CdisBody::Acknowledge => { None } 
            CdisBody::ActionRequest => { None } 
            CdisBody::ActionResponse => { None } 
            CdisBody::DataQuery => { None } 
            CdisBody::SetData => { None } 
            CdisBody::Data => { None } 
            CdisBody::EventReport => { None } 
            CdisBody::Comment => { None } 
            CdisBody::ElectromagneticEmission => { None } 
            CdisBody::Designator => { None } 
            CdisBody::Transmitter => { None } 
            CdisBody::Signal => { None } 
            CdisBody::Receiver => { None } 
            CdisBody::Iff => { None } 
        }
    }
}

#[derive(Clone, Debug, PartialEq, Error)]
pub enum CdisError {
    #[error("{0}")]
    ParseError(String), // the parsing of a CDIS PDU resulted in an error
    #[error("The buffer does not contain enough bytes for a valid C-DIS header. {0} bits available.")]
    InsufficientHeaderLength(u16), // the input was too small to contain a valid CDIS header; (u16 found)
    #[error("C-DIS PDU has insufficient length. Expected {0} bits, found {1} bits.")]
    InsufficientPduLength(u16, u16), // the input was too small to contain a valid CDIS PDU based on the header and parsing; (u16 expected, u16 found)
    #[error("C-DIS PDU is larger than size of the buffer for serialisation. Needs {0} bits, available {1} bits.")]
    InsufficientBufferSize(u16, usize), // the buffer for serialisation has insufficient capacity to hold the provided CDIS PDU; (u16 PDU size, usize available capacity)
    #[error("Encountered a C-DIS PDU of an unsupported type: {0}.")]
    UnsupportedPdu(u8), // encountered a CDIS PDU of an unsupported type; (u8 PduType found)
}

/// Trait that indicates whether a PDU is supported in the C-DIS standard
pub trait Supported {
    /// Returns true when a PDUs having a certain PduType is supported by the C-DIS standard, false otherwise.
    fn is_supported(&self) -> bool;
}

impl Supported for PduType {
    fn is_supported(&self) -> bool {
        matches!(self,
            PduType::EntityState |
            PduType::Fire |
            PduType::Detonation |
            PduType::Collision |
            PduType::CreateEntity |
            PduType::RemoveEntity |
            PduType::StartResume |
            PduType::StopFreeze |
            PduType::Acknowledge |
            PduType::ActionRequest |
            PduType::ActionResponse |
            PduType::DataQuery |
            PduType::SetData |
            PduType::Data |
            PduType::EventReport |
            PduType::Comment |
            PduType::ElectromagneticEmission |
            PduType::Designator |
            PduType::Transmitter |
            PduType::Signal |
            PduType::Receiver |
            PduType::IFF
        )
    }
}

/// Trait that indicates whether a PDU is implemented in C-DIS
pub trait Implemented {
    /// Returns true when the library implements PDUs having a certain PduType, false otherwise.
    fn is_implemented(&self) -> bool;
}

impl Implemented for PduType {
    /// A PduType is properly implemented by the C-DIS library when:
    /// - There is a model for the pdu body
    /// - CdisBody enum is adapted, including the trait implementation for `CdisInteraction` and method `body_length(..)`
    /// - There is a parser, and it is called in function `crate::parsing::cdis_body(..)`
    /// - There is a serializer, and it is called in the `SerializeCdisPdu` trait impl for `CdisBody` in `crate::writing`.
    /// - The codec implementations are present, and are called in `crate::codec` in the `CdisBody::encode` and `CdisBody::decode` implementations.
    fn is_implemented(&self) -> bool {
        match self {
            PduType::EntityState |
            PduType::Fire |
            PduType::Detonation |
            PduType::Collision |
            PduType::CreateEntity |
            PduType::RemoveEntity |
            PduType::StartResume => { true }
            // PduType::StopFreeze |
            // PduType::Acknowledge |
            // PduType::ActionRequest |
            // PduType::ActionResponse |
            // PduType::DataQuery |
            // PduType::SetData |
            // PduType::Data |
            // PduType::EventReport |
            // PduType::Comment |
            // PduType::ElectromagneticEmission |
            // PduType::Designator |
            // PduType::Transmitter |
            // PduType::Signal |
            // PduType::Receiver |
            // PduType::IFF
            _ => { false }
        }
    }
}

#[cfg(test)]
mod tests {
    use dis_rs::enumerations::PduType;
    use crate::{CdisBody, Implemented, Supported};

    #[test]
    fn ensure_supported_pdus() {
        assert_eq!(PduType::EntityState.is_supported(), true);
        assert_eq!(PduType::Fire.is_supported(), true);
        assert_eq!(PduType::Detonation.is_supported(), true);
        assert_eq!(PduType::Collision.is_supported(), true);
        assert_eq!(PduType::CreateEntity.is_supported(), true);
        assert_eq!(PduType::RemoveEntity.is_supported(), true);
        assert_eq!(PduType::StartResume.is_supported(), true);
        assert_eq!(PduType::StopFreeze.is_supported(), true);
        assert_eq!(PduType::Acknowledge.is_supported(), true);
        assert_eq!(PduType::ActionRequest.is_supported(), true);
        assert_eq!(PduType::ActionResponse.is_supported(), true);
        assert_eq!(PduType::DataQuery.is_supported(), true);
        assert_eq!(PduType::SetData.is_supported(), true);
        assert_eq!(PduType::Data.is_supported(), true);
        assert_eq!(PduType::EventReport.is_supported(), true);
        assert_eq!(PduType::Comment.is_supported(), true);
        assert_eq!(PduType::ElectromagneticEmission.is_supported(), true);
        assert_eq!(PduType::Designator.is_supported(), true);
        assert_eq!(PduType::Transmitter.is_supported(), true);
        assert_eq!(PduType::Signal.is_supported(), true);
        assert_eq!(PduType::Receiver.is_supported(), true);
        assert_eq!(PduType::IFF.is_supported(), true);

        assert_eq!(PduType::Other.is_supported(), false);
        assert_eq!(PduType::ServiceRequest.is_supported(), false);
        assert_eq!(PduType::ResupplyOffer.is_supported(), false);
        assert_eq!(PduType::ResupplyReceived.is_supported(), false);
        assert_eq!(PduType::ResupplyCancel.is_supported(), false);
        assert_eq!(PduType::RepairComplete.is_supported(), false);
        assert_eq!(PduType::RepairResponse.is_supported(), false);
        assert_eq!(PduType::UnderwaterAcoustic.is_supported(), false);
        assert_eq!(PduType::SupplementalEmissionEntityState.is_supported(), false);
        assert_eq!(PduType::IntercomSignal.is_supported(), false);
        assert_eq!(PduType::IntercomControl.is_supported(), false);
        assert_eq!(PduType::AggregateState.is_supported(), false);
        assert_eq!(PduType::IsGroupOf.is_supported(), false);
        assert_eq!(PduType::TransferOwnership.is_supported(), false);
        assert_eq!(PduType::IsPartOf.is_supported(), false);
        assert_eq!(PduType::MinefieldState.is_supported(), false);
        assert_eq!(PduType::MinefieldQuery.is_supported(), false);
        assert_eq!(PduType::MinefieldData.is_supported(), false);
        assert_eq!(PduType::MinefieldResponseNACK.is_supported(), false);
        assert_eq!(PduType::EnvironmentalProcess.is_supported(), false);
        assert_eq!(PduType::GriddedData.is_supported(), false);
        assert_eq!(PduType::PointObjectState.is_supported(), false);
        assert_eq!(PduType::LinearObjectState.is_supported(), false);
        assert_eq!(PduType::ArealObjectState.is_supported(), false);
        assert_eq!(PduType::TSPI.is_supported(), false);
        assert_eq!(PduType::Appearance.is_supported(), false);
        assert_eq!(PduType::ArticulatedParts.is_supported(), false);
        assert_eq!(PduType::LEFire.is_supported(), false);
        assert_eq!(PduType::LEDetonation.is_supported(), false);
        assert_eq!(PduType::CreateEntityR.is_supported(), false);
        assert_eq!(PduType::RemoveEntityR.is_supported(), false);
        assert_eq!(PduType::StartResumeR.is_supported(), false);
        assert_eq!(PduType::StopFreezeR.is_supported(), false);
        assert_eq!(PduType::AcknowledgeR.is_supported(), false);
        assert_eq!(PduType::ActionRequestR.is_supported(), false);
        assert_eq!(PduType::ActionResponseR.is_supported(), false);
        assert_eq!(PduType::DataQueryR.is_supported(), false);
        assert_eq!(PduType::SetDataR.is_supported(), false);
        assert_eq!(PduType::DataR.is_supported(), false);
        assert_eq!(PduType::EventReportR.is_supported(), false);
        assert_eq!(PduType::CommentR.is_supported(), false);
        assert_eq!(PduType::RecordR.is_supported(), false);
        assert_eq!(PduType::SetRecordR.is_supported(), false);
        assert_eq!(PduType::RecordQueryR.is_supported(), false);
        assert_eq!(PduType::CollisionElastic.is_supported(), false);
        assert_eq!(PduType::EntityStateUpdate.is_supported(), false);
        assert_eq!(PduType::DirectedEnergyFire.is_supported(), false);
        assert_eq!(PduType::EntityDamageStatus.is_supported(), false);
        assert_eq!(PduType::InformationOperationsAction.is_supported(), false);
        assert_eq!(PduType::InformationOperationsReport.is_supported(), false);
        assert_eq!(PduType::Attribute.is_supported(), false);
        assert_eq!(PduType::Unspecified(0).is_supported(), false);
    }

    #[test]
    fn validate_implemented_pdus() {
        assert!(PduType::EntityState.is_implemented());
        assert!(PduType::Fire.is_implemented());
        assert!(PduType::Detonation.is_implemented());
        assert!(PduType::Collision.is_implemented());
        assert!(PduType::CreateEntity.is_implemented());
        assert!(PduType::RemoveEntity.is_implemented());
        
        assert_eq!(PduType::StartResume.is_implemented() || CdisBody::StartResume.body_length() != 0, false);
        assert_eq!(PduType::StopFreeze.is_implemented() || CdisBody::StopFreeze.body_length() != 0, false);
        assert_eq!(PduType::Acknowledge.is_implemented() || CdisBody::Acknowledge.body_length() != 0, false);
        assert_eq!(PduType::ActionRequest.is_implemented() || CdisBody::ActionRequest.body_length() != 0, false);
        assert_eq!(PduType::ActionResponse.is_implemented() || CdisBody::ActionResponse.body_length() != 0, false);
        assert_eq!(PduType::DataQuery.is_implemented() || CdisBody::DataQuery.body_length() != 0, false);
        assert_eq!(PduType::SetData.is_implemented() || CdisBody::SetData.body_length() != 0, false);
        assert_eq!(PduType::Data.is_implemented() || CdisBody::Data.body_length() != 0, false);
        assert_eq!(PduType::EventReport.is_implemented() || CdisBody::EventReport.body_length() != 0, false);
        assert_eq!(PduType::Comment.is_implemented() || CdisBody::Comment.body_length() != 0, false);
        assert_eq!(PduType::ElectromagneticEmission.is_implemented() || CdisBody::ElectromagneticEmission.body_length() != 0, false);
        assert_eq!(PduType::Designator.is_implemented() || CdisBody::Designator.body_length() != 0, false);
        assert_eq!(PduType::Transmitter.is_implemented() || CdisBody::Transmitter.body_length() != 0, false);
        assert_eq!(PduType::Signal.is_implemented() || CdisBody::Signal.body_length() != 0, false);
        assert_eq!(PduType::Receiver.is_implemented() || CdisBody::Receiver.body_length() != 0, false);
        assert_eq!(PduType::IFF.is_implemented() || CdisBody::Iff.body_length() != 0, false);
    }
}
