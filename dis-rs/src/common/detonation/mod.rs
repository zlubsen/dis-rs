pub mod builder;
pub mod model;
pub mod parser;
pub mod writer;

#[cfg(test)]
mod tests {
    use crate::common::detonation::model::Detonation;
    use crate::common::model::DisTimeStamp;
    use crate::common::model::{Pdu, PduHeader};
    use crate::common::parser::parse_pdu;
    use crate::enumerations::PduType;
    use bytes::BytesMut;

    #[test]
    fn detonation_internal_consistency() {
        let header = PduHeader::new_v6(1, PduType::Detonation);

        let body = Detonation::builder().build().into_pdu_body();
        let original_pdu =
            Pdu::finalize_from_parts(header, body, DisTimeStamp::new_absolute_from_secs(100));
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
