pub mod builder;
pub mod model;
pub mod parser;
pub mod writer;

#[cfg(test)]
mod tests {
    use crate::common::comment::model::Comment;
    use crate::common::model::DisTimeStamp;
    use crate::common::model::{EntityId, Pdu, PduHeader, VariableDatum};
    use crate::common::parser::parse_pdu;
    use crate::enumerations::{PduType, VariableRecordType};
    use bytes::BytesMut;

    #[test]
    fn comment_internal_consistency() {
        let header = PduHeader::new_v6(1, PduType::Comment);

        let body = Comment::builder()
            .with_origination_id(EntityId::new(1, 2, 3))
            .with_receiving_id(EntityId::new(4, 5, 6))
            .with_variable_datums(vec![VariableDatum::new(
                VariableRecordType::Age_34100,
                vec![12, 34],
            )])
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
            Err(ref _err) => {
                println!("{_err}");
                assert!(false);
            }
        }
    }
}
