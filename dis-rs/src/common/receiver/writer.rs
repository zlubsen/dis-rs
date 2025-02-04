use crate::common::receiver::model::Receiver;
use crate::common::{Serialize, SerializePdu, SupportedVersion};
use bytes::{BufMut, BytesMut};

impl SerializePdu for Receiver {
    fn serialize_pdu(&self, _version: SupportedVersion, buf: &mut BytesMut) -> u16 {
        let rx_ref_id_bytes = self.radio_reference_id.serialize(buf);
        buf.put_u16(self.radio_number);
        buf.put_u16(self.receiver_state.into());
        buf.put_u16(0u16);
        buf.put_f32(self.received_power);
        let tx_ref_id_bytes = self.transmitter_radio_reference_id.serialize(buf);
        buf.put_u16(self.transmitter_radio_number);

        12 + rx_ref_id_bytes + tx_ref_id_bytes
    }
}
