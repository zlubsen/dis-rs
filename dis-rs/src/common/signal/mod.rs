pub mod builder;
pub mod model;
pub mod parser;
pub mod writer;

#[cfg(test)]
mod tests {
    use crate::common::model::DisTimeStamp;
    use crate::common::model::{Pdu, PduHeader};
    use crate::common::parser::parse_pdu;
    use crate::enumerations::{PduType, SignalEncodingClass, SignalEncodingType};
    use crate::model::EntityId;
    use crate::parser::parse_multiple_pdu;
    use crate::signal::model::{EncodingScheme, Signal};
    use bytes::BytesMut;

    #[test]
    fn signal_internal_consistency() {
        let header = PduHeader::new_v6(1, PduType::Signal);

        let body = Signal::builder()
            .with_encoding_scheme(EncodingScheme::EncodedAudio {
                encoding_class: SignalEncodingClass::EncodedAudio,
                encoding_type: SignalEncodingType::_16bitLinearPCM2sComplement_BigEndian_4,
            })
            .with_samples(20)
            .with_sample_rate(20000)
            .with_radio_number(10)
            .with_radio_reference_id(EntityId::new(10, 10, 123))
            .with_data(vec![0x10, 0x10, 0x10])
            .build()
            .into_pdu_body();
        let original_pdu =
            Pdu::finalize_from_parts(header, body, DisTimeStamp::new_absolute_from_secs(100));
        let pdu_length = original_pdu.header.pdu_length;
        let original_length = original_pdu.pdu_length();

        let mut buf = BytesMut::with_capacity(pdu_length as usize);

        let serialized_length = original_pdu.serialize(&mut buf).unwrap();

        assert_eq!(original_length, serialized_length);

        let parsed = parse_pdu(&buf);
        match parsed {
            Ok(ref pdu) => {
                assert_eq!(&original_pdu, pdu);
            }
            Err(ref err) => {
                panic!("Parse error: {err}");
            }
        }
    }

    #[test]
    fn signal_test_padding_parsing() {
        let header = PduHeader::new_v6(1, PduType::Signal);

        let body = Signal::builder()
            .with_encoding_scheme(EncodingScheme::EncodedAudio {
                encoding_class: SignalEncodingClass::EncodedAudio,
                encoding_type: SignalEncodingType::_16bitLinearPCM2sComplement_BigEndian_4,
            })
            .with_samples(20)
            .with_sample_rate(20000)
            .with_radio_number(10)
            .with_radio_reference_id(EntityId::new(10, 10, 123))
            .with_data(vec![0x10, 0x10, 0x10])
            .build()
            .into_pdu_body();
        let original_pdu =
            Pdu::finalize_from_parts(header, body, DisTimeStamp::new_absolute_from_secs(100));
        let pdu_length = original_pdu.header.pdu_length;
        let original_length = original_pdu.pdu_length();

        let mut buf = BytesMut::with_capacity(pdu_length as usize);

        let serialized_length = original_pdu.serialize(&mut buf).unwrap();

        let two_signals = [buf.clone(), buf.clone()].concat();

        assert_eq!(original_length, serialized_length);

        let parsed = parse_multiple_pdu(two_signals.as_slice());
        match parsed {
            Ok(ref pdu) => {
                assert_eq!(&original_pdu, &pdu[0]);
                assert_eq!(&original_pdu, &pdu[1]);
            }
            Err(ref err) => {
                panic!("Parse error: {err}");
            }
        }
    }
}
