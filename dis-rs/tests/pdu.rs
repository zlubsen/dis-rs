use bytes::BytesMut;
use dis_rs::{
    enumerations::{PduType, SignalEncodingClass, SignalEncodingType},
    model::{Pdu, PduBody, PduHeader},
    signal::model::{EncodingScheme, Signal},
    DisError,
};
use nom::AsBytes;

#[test]
fn test_pdu() {
    let data = vec![1, 2, 3, 4];
    let pdu = Pdu {
        header: PduHeader::new_v7(1, PduType::Signal),
        body: PduBody::Signal(
            Signal::builder()
                .with_encoding_scheme(EncodingScheme::EncodedAudio {
                    encoding_class: SignalEncodingClass::EncodedAudio,
                    encoding_type: SignalEncodingType::_16bitLinearPCM2sComplement_LittleEndian_100,
                })
                .with_data(data.clone())
                .build(),
        ),
    };
    let mut buf = BytesMut::with_capacity(pdu.pdu_length() as usize);
    let _ = pdu.serialize(&mut buf);
    let pdus = dis_rs::parse(buf.as_bytes()).unwrap();

    assert_eq!(pdus.len(), 1);

    let s = if let PduBody::Signal(s) = &pdus.first().unwrap().body {
        Some(s)
    } else {
        None
    };

    assert_eq!(s.unwrap().data, data);
}

#[test]
fn test_two_pdus() {
    let data = vec![1, 2, 3, 4];
    let pdu = Pdu {
        header: PduHeader::new_v7(1, PduType::Signal),
        body: PduBody::Signal(
            Signal::builder()
                .with_encoding_scheme(EncodingScheme::EncodedAudio {
                    encoding_class: SignalEncodingClass::EncodedAudio,
                    encoding_type: SignalEncodingType::_16bitLinearPCM2sComplement_LittleEndian_100,
                })
                .with_data(data.clone())
                .build(),
        ),
    };
    let mut buf_a = BytesMut::with_capacity(pdu.pdu_length() as usize);
    let _ = pdu.serialize(&mut buf_a);

    let mut buf_b = BytesMut::with_capacity(pdu.pdu_length() as usize);
    let _ = pdu.serialize(&mut buf_b);

    let buf = [buf_a, buf_b].concat();

    let pdus = dis_rs::parse(buf.as_bytes()).unwrap();

    assert_eq!(pdus.len(), 2);

    let s = if let PduBody::Signal(s) = &pdus.get(1).unwrap().body {
        Some(s)
    } else {
        None
    };

    assert_eq!(s.unwrap().data, data);
}

#[test]
fn test_insufficient_buffer_capacity() {
    let data = vec![1, 2, 3, 4];
    let pdu = Pdu {
        header: PduHeader::new_v7(1, PduType::Signal),
        body: PduBody::Signal(
            Signal::builder()
                .with_encoding_scheme(EncodingScheme::EncodedAudio {
                    encoding_class: SignalEncodingClass::EncodedAudio,
                    encoding_type: SignalEncodingType::_16bitLinearPCM2sComplement_LittleEndian_100,
                })
                .with_data(data.clone())
                .build(),
        ),
    };
    const SOME_SMALL_AMOUNT_OF_BYTES: usize = 20;
    let mut buf = BytesMut::with_capacity(SOME_SMALL_AMOUNT_OF_BYTES);
    let serialisation_result = pdu.serialize(&mut buf);

    assert_eq!(
        serialisation_result,
        Err(DisError::InsufficientBufferSize(
            pdu.pdu_length(),
            SOME_SMALL_AMOUNT_OF_BYTES
        ))
    );
}
