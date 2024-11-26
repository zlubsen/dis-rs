pub mod parser;
pub mod model;
pub mod writer;
pub mod builder;

#[cfg(test)]
mod tests {
    use bytes::BytesMut;
    use crate::enumerations::{PduType, SignalEncodingClass, SignalEncodingType};
    use crate::common::model::{Pdu, PduHeader};
    use crate::common::parser::parse_pdu;
    use crate::common::model::{DisTimeStamp};
    use crate::model::EntityId;
    use crate::signal::model::{EncodingScheme, Signal};

    #[test]
    fn signal_internal_consistency() {
        let header = PduHeader::new_v6(1, PduType::Signal);

        let body = Signal::builder()
            .with_encoding_scheme(EncodingScheme::EncodedAudio {encoding_class: SignalEncodingClass::EncodedAudio, encoding_type: SignalEncodingType::_16bitLinearPCM2sComplement_BigEndian_4})
            .with_samples(20)
            .with_sample_rate(20000)
            .with_radio_number(10)
            .with_radio_reference_id(EntityId::new(10, 10, 123))
            .with_data(vec![0x10, 0x10, 0x10])
            .build()
            .into_pdu_body();
        let original_pdu = Pdu::finalize_from_parts(header, body, DisTimeStamp::new_absolute_from_secs(100));
        let pdu_length = original_pdu.header.pdu_length;

        let mut buf = BytesMut::with_capacity(pdu_length as usize);

        original_pdu.serialize(&mut buf).unwrap();

        let parsed = parse_pdu(&buf);
        match parsed {
            Ok(ref pdu) => {
                assert_eq!(&original_pdu, pdu);
            }
            Err(ref _err) => {
                println!("{_err}");
                assert!(false);
            }
        }
    }
}