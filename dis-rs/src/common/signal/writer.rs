use bytes::{BufMut, BytesMut};
use crate::common::{Serialize, SerializePdu, SupportedVersion};
use crate::common::signal::model::{EncodingScheme, Signal};
use crate::constants::{FOUR_OCTETS, ONE_BYTE_IN_BITS};
use crate::common::model::length_padded_to_num;

impl SerializePdu for Signal {
    fn serialize_pdu(&self, _version: SupportedVersion, buf: &mut BytesMut) -> u16 {
        let radio_ref_id_bytes = self.radio_reference_id.serialize(buf);
        buf.put_u16(self.radio_number);
        let encoding_scheme_bytes = self.encoding_scheme.serialize(buf);
        buf.put_u16(self.tdl_type.into());
        buf.put_u32(self.sample_rate);
        buf.put_u16((self.data.len() * ONE_BYTE_IN_BITS) as u16);
        buf.put_u16(self.samples);
        buf.put(&self.data[..]);
        let padded_record_lengths = length_padded_to_num(
            self.data.len(),
            FOUR_OCTETS);
        buf.put_bytes(0u8, padded_record_lengths.padding_length);

        radio_ref_id_bytes + 2 + encoding_scheme_bytes + 10 + padded_record_lengths.record_length as u16
    }
}

impl Serialize for EncodingScheme {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        match self {
            EncodingScheme::EncodedAudio { encoding_class, encoding_type } => {
                let class_bits = u16::from(*encoding_class) << 14;
                let type_bits = u16::from(*encoding_type);
                buf.put_u16(class_bits | type_bits);
                2
            }
            EncodingScheme::RawBinaryData { encoding_class, nr_of_messages } => {
                let class_bits = u16::from(*encoding_class) << 14;
                buf.put_u16(class_bits | *nr_of_messages);
                2
            }
            EncodingScheme::ApplicationSpecificData { encoding_class, .. } => {
                let class_bits = u16::from(*encoding_class) << 14;
                buf.put_u16(class_bits);
                2
            }
            EncodingScheme::DatabaseIndex { encoding_class, .. } => {
                let class_bits = u16::from(*encoding_class) << 14;
                buf.put_u16(class_bits);
                2
            }
            EncodingScheme::Unspecified { encoding_class } => {
                let class_bits = u16::from(*encoding_class) << 14;
                buf.put_u16(class_bits);
                2
            }
        }
    }
}