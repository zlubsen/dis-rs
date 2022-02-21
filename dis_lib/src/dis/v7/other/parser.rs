use nom::bytes::complete::take;
use nom::IResult;
use crate::dis::common::model::PDU_HEADER_LEN_BYTES;
use crate::dis::v7::model::{Pdu, PduHeader};
use crate::dis::v7::other::model::Other as OtherStruct;

pub fn other_body(header: PduHeader) -> impl Fn(&[u8]) -> IResult<&[u8], Pdu> {
    move | input: &[u8] | {
        let body_length_bytes = header.pdu_length as usize - PDU_HEADER_LEN_BYTES;
        let (input, body) = take(body_length_bytes)(input)?;
        // TODO review if this is what we want: if we cannot copy the slice, return an empty vec?
        let body = body.to_vec();
        Ok((input, Pdu::Other(OtherStruct { header, body })))
    }
}

#[cfg(test)]
mod tests {
    use crate::dis::common::model::{PDU_HEADER_LEN_BYTES, PduType, ProtocolFamily, ProtocolVersion};
    use crate::dis::v6::builder::PduHeaderBuilder;
    use crate::dis::v6::model::Pdu;
    use crate::dis::v6::other::parser::other_body;

    #[test]
    fn parse_other_body() {
        let header = PduHeaderBuilder::new()
            .protocol_version(ProtocolVersion::Ieee1278_1a1998)
            .exercise_id(1)
            .pdu_type(PduType::OtherPdu)
            .protocol_family(ProtocolFamily::Other)
            .pdu_length((PDU_HEADER_LEN_BYTES + 10) as u16)
            .time_stamp(0)
            .build().expect("Should be good");
        let input : [u8;10] = [0x01,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00];
        let (input, pdu) = other_body(header)(&input).expect("Should be Ok");
        if let Pdu::Other(pdu) = pdu {
            assert_eq!(pdu.body.len(), 10);
            assert_eq!(*pdu.body.get(0).unwrap(), 1u8);
        }
        assert!(input.is_empty());
    }
}