pub mod model;
pub mod parser;
pub mod writer;
pub mod builder;

#[cfg(test)]
mod tests {
    use bytes::BytesMut;
    use crate::common::attribute::model::Attribute;
    use crate::enumerations::{AttributeActionCode, PduType, ProtocolVersion};
    use crate::common::model::{Pdu, PduHeader, SimulationAddress};
    use crate::common::parser::parse_pdu;
    use crate::common::Serialize;
    use crate::common::model::{DisTimeStamp};

    #[test]
    fn attribute_internal_consistency() {
        let header = PduHeader::new_v6(1, PduType::Attribute);

        let body = Attribute::builder()
            .with_originating_simulation_address(SimulationAddress::new(10, 10))
            .with_action_code(AttributeActionCode::NoStatement)
            .with_record_protocol_version(ProtocolVersion::IEEE1278_1A1998)
            .with_record_pdu_type(PduType::Attribute)
            .build()
            .into_pdu_body();
        let original_pdu = Pdu::finalize_from_parts(header, body, DisTimeStamp::new_absolute_from_secs(100));
        let pdu_length = original_pdu.header.pdu_length;

        let mut buf = BytesMut::with_capacity(pdu_length as usize);

        original_pdu.serialize(&mut buf);

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