use bytes::BytesMut;
use dis_rs::{
    enumerations::{PduType, SignalEncodingClass, SignalEncodingType},
    model::{Pdu, PduBody, PduHeader},
    signal::model::{EncodingScheme, Signal},
    Serialize,
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
                    encoding_class: SignalEncodingClass::Encodedaudio,
                    encoding_type: SignalEncodingType::_16bitLinearPCM2sComplement_LittleEndian_100,
                })
                .with_data(data.clone())
                .build(),
        ),
    };
    let mut buf = BytesMut::new();
    let _ = pdu.serialize(&mut buf);
    let pdus = dis_rs::parse(buf.as_bytes()).unwrap();
    
    assert_eq!(pdus.len(), 1);


    let s = if let PduBody::Signal(s) = &pdus.get(0).unwrap().body {
        Some(s)
    } else {
        None
    };

    assert_eq!(s.unwrap().data,data);
}
