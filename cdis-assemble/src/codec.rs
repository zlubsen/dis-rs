use dis_rs::model::{Pdu, PduBody};
use crate::{CdisBody, CdisPdu};
use crate::entity_state::model::EntityState;
use crate::records::model::{CdisHeader};
use crate::unsupported::Unsupported;

pub trait Codec {
    /// The DIS PDU, Body, Record, ... that is to be converted.
    type Counterpart;
    const SCALING: f32 = 0.0;
    const SCALING_2: f32 = 0.0;
    const CONVERSION: f32 = 0.0;
    const NORMALISATION: f32 = 0.0;

    fn encode(item: &Self::Counterpart) -> Self;
    fn decode(&self) -> Self::Counterpart;
}

impl Codec for CdisPdu {
    type Counterpart = Pdu;

    fn encode(item: &Self::Counterpart) -> Self {
        Self {
            header: CdisHeader::encode(&item.header),
            body: CdisBody::encode(&item.body),
        }
    }

    fn decode(&self) -> Self::Counterpart {
        let header = self.header.decode();
        let ts = header.time_stamp;
        Self::Counterpart::finalize_from_parts(
            header,
            self.body.decode(),
            ts
        )
    }
}

impl Codec for CdisBody {
    type Counterpart = PduBody;

    fn encode(item: &Self::Counterpart) -> Self {
        match item {
            PduBody::Other(_) => { Self::Unsupported(Unsupported) }
            PduBody::EntityState(body) => { Self::EntityState(EntityState::encode(body)) }
            PduBody::Fire(_) => { Self::Unsupported(Unsupported) }
            PduBody::Detonation(_) => { Self::Unsupported(Unsupported) }
            PduBody::Collision(_) => { Self::Unsupported(Unsupported) }
            PduBody::ServiceRequest(_) => { Self::Unsupported(Unsupported) }
            PduBody::ResupplyOffer(_) => { Self::Unsupported(Unsupported) }
            PduBody::ResupplyReceived(_) => { Self::Unsupported(Unsupported) }
            PduBody::ResupplyCancel(_) => { Self::Unsupported(Unsupported) }
            PduBody::RepairComplete(_) => { Self::Unsupported(Unsupported) }
            PduBody::RepairResponse(_) => { Self::Unsupported(Unsupported) }
            PduBody::CreateEntity(_) => { Self::Unsupported(Unsupported) }
            PduBody::RemoveEntity(_) => { Self::Unsupported(Unsupported) }
            PduBody::StartResume(_) => { Self::Unsupported(Unsupported) }
            PduBody::StopFreeze(_) => { Self::Unsupported(Unsupported) }
            PduBody::Acknowledge(_) => { Self::Unsupported(Unsupported) }
            PduBody::ActionRequest(_) => { Self::Unsupported(Unsupported) }
            PduBody::ActionResponse(_) => { Self::Unsupported(Unsupported) }
            PduBody::DataQuery(_) => { Self::Unsupported(Unsupported) }
            PduBody::SetData(_) => { Self::Unsupported(Unsupported) }
            PduBody::Data(_) => { Self::Unsupported(Unsupported) }
            PduBody::EventReport(_) => { Self::Unsupported(Unsupported) }
            PduBody::Comment(_) => { Self::Unsupported(Unsupported) }
            PduBody::ElectromagneticEmission(_) => { Self::Unsupported(Unsupported) }
            PduBody::Designator(_) => { Self::Unsupported(Unsupported) }
            PduBody::Transmitter(_) => { Self::Unsupported(Unsupported) }
            PduBody::Signal(_) => { Self::Unsupported(Unsupported) }
            PduBody::Receiver(_) => { Self::Unsupported(Unsupported) }
            PduBody::IFF(_) => { Self::Unsupported(Unsupported) }
            PduBody::UnderwaterAcoustic(_) => { Self::Unsupported(Unsupported) }
            PduBody::SupplementalEmissionEntityState(_) => { Self::Unsupported(Unsupported) }
            PduBody::IntercomSignal => { Self::Unsupported(Unsupported) }
            PduBody::IntercomControl => { Self::Unsupported(Unsupported) }
            PduBody::AggregateState(_) => { Self::Unsupported(Unsupported) }
            PduBody::IsGroupOf(_) => { Self::Unsupported(Unsupported) }
            PduBody::TransferOwnership(_) => { Self::Unsupported(Unsupported) }
            PduBody::IsPartOf(_) => { Self::Unsupported(Unsupported) }
            PduBody::MinefieldState => { Self::Unsupported(Unsupported) }
            PduBody::MinefieldQuery => { Self::Unsupported(Unsupported) }
            PduBody::MinefieldData => { Self::Unsupported(Unsupported) }
            PduBody::MinefieldResponseNACK => { Self::Unsupported(Unsupported) }
            PduBody::EnvironmentalProcess => { Self::Unsupported(Unsupported) }
            PduBody::GriddedData => { Self::Unsupported(Unsupported) }
            PduBody::PointObjectState => { Self::Unsupported(Unsupported) }
            PduBody::LinearObjectState => { Self::Unsupported(Unsupported) }
            PduBody::ArealObjectState => { Self::Unsupported(Unsupported) }
            PduBody::TSPI => { Self::Unsupported(Unsupported) }
            PduBody::Appearance => { Self::Unsupported(Unsupported) }
            PduBody::ArticulatedParts => { Self::Unsupported(Unsupported) }
            PduBody::LEFire => { Self::Unsupported(Unsupported) }
            PduBody::LEDetonation => { Self::Unsupported(Unsupported) }
            PduBody::CreateEntityR(_) => { Self::Unsupported(Unsupported) }
            PduBody::RemoveEntityR(_) => { Self::Unsupported(Unsupported) }
            PduBody::StartResumeR(_) => { Self::Unsupported(Unsupported) }
            PduBody::StopFreezeR(_) => { Self::Unsupported(Unsupported) }
            PduBody::AcknowledgeR(_) => { Self::Unsupported(Unsupported) }
            PduBody::ActionRequestR(_) => { Self::Unsupported(Unsupported) }
            PduBody::ActionResponseR(_) => { Self::Unsupported(Unsupported) }
            PduBody::DataQueryR(_) => { Self::Unsupported(Unsupported) }
            PduBody::SetDataR(_) => { Self::Unsupported(Unsupported) }
            PduBody::DataR(_) => { Self::Unsupported(Unsupported) }
            PduBody::EventReportR(_) => { Self::Unsupported(Unsupported) }
            PduBody::CommentR(_) => { Self::Unsupported(Unsupported) }
            PduBody::RecordR(_) => { Self::Unsupported(Unsupported) }
            PduBody::SetRecordR(_) => { Self::Unsupported(Unsupported) }
            PduBody::RecordQueryR(_) => { Self::Unsupported(Unsupported) }
            PduBody::CollisionElastic(_) => { Self::Unsupported(Unsupported) }
            PduBody::EntityStateUpdate(_) => { Self::Unsupported(Unsupported) }
            PduBody::DirectedEnergyFire => { Self::Unsupported(Unsupported) }
            PduBody::EntityDamageStatus => { Self::Unsupported(Unsupported) }
            PduBody::InformationOperationsAction => { Self::Unsupported(Unsupported) }
            PduBody::InformationOperationsReport => { Self::Unsupported(Unsupported) }
            PduBody::Attribute(_) => { Self::Unsupported(Unsupported) }
        }
    }

    fn decode(&self) -> Self::Counterpart {
        match self {
            // TODO add 'Unimplemented' Body to dis-rs, as impl for remaining PduBody types
            CdisBody::EntityState(body) => { PduBody::EntityState(body.decode()) }
            // CdisBody::Fire => {}
            // CdisBody::Detonation => {}
            // CdisBody::Collision => {}
            // CdisBody::CreateEntity => {}
            // CdisBody::RemoveEntity => {}
            // CdisBody::StartResume => {}
            // CdisBody::StopFreeze => {}
            // CdisBody::Acknowledge => {}
            // CdisBody::ActionRequest => {}
            // CdisBody::ActionResponse => {}
            // CdisBody::DataQuery => {}
            // CdisBody::SetData => {}
            // CdisBody::Data => {}
            // CdisBody::EventReport => {}
            // CdisBody::Comment => {}
            // CdisBody::ElectromagneticEmission => {}
            // CdisBody::Designator => {}
            // CdisBody::Transmitter => {}
            // CdisBody::Signal => {}
            // CdisBody::Receiver => {}
            // CdisBody::Iff => {}
            CdisBody::Unsupported(_) | _ => { PduBody::Other(dis_rs::other::model::Other::builder().build())}
        }
    }
}