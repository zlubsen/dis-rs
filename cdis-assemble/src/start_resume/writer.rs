use crate::start_resume::model::StartResume;
use crate::types::writer::serialize_clock_time;
use crate::writing::SerializeCdis;
use crate::{BitBuffer, SerializeCdisPdu};

impl SerializeCdisPdu for StartResume {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = self.originating_id.serialize(buf, cursor);
        let cursor = self.receiving_id.serialize(buf, cursor);
        let cursor = serialize_clock_time(buf, cursor, &self.real_world_time);
        let cursor = serialize_clock_time(buf, cursor, &self.simulation_time);
        let cursor = self.request_id.serialize(buf, cursor);

        cursor
    }
}
