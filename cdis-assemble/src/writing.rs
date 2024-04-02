use crate::{BitBuffer, CdisBody, CdisError, CdisPdu, SerializeCdis, SerializeCdisPdu};

impl SerializeCdisPdu for CdisPdu {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> Result<usize, CdisError> {
        let cursor = self.header.serialize(buf, cursor);
        let cursor = self.body.serialize(buf, cursor)?;

        Ok(cursor)
    }
}

impl SerializeCdisPdu for CdisBody {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> Result<usize, CdisError> {
        let cursor = match self {
            CdisBody::EntityState(body) => { body.serialize(buf, cursor) }
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
            _ => { cursor }
        }?;

        Ok(cursor)
    }
}
