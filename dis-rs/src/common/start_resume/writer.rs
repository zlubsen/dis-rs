use crate::common::start_resume::model::StartResume;
use crate::common::{Serialize, SerializePdu, SupportedVersion};
use bytes::{BufMut, BytesMut};

impl SerializePdu for StartResume {
    fn serialize_pdu(&self, _version: SupportedVersion, buf: &mut BytesMut) -> u16 {
        let originating_id_bytes = self.originating_id.serialize(buf);
        let receiving_id_bytes = self.receiving_id.serialize(buf);
        let real_world_bytes = self.real_world_time.serialize(buf);
        let sim_time_bytes = self.simulation_time.serialize(buf);
        buf.put_u32(self.request_id);

        originating_id_bytes + receiving_id_bytes + real_world_bytes + sim_time_bytes + 4
    }
}
