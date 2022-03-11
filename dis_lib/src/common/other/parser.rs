use nom::bytes::complete::take;
use nom::IResult;
use crate::common::model::{PduBody, PduHeader};
use crate::common::other::model::Other;
use crate::common::symbolic_names::PDU_HEADER_LEN_BYTES;

pub fn other_body(header: &PduHeader) -> impl Fn(&[u8]) -> IResult<&[u8], PduBody> + '_ {
    move | input: &[u8] | {
        let body_length_bytes = header.pdu_length as usize - PDU_HEADER_LEN_BYTES;
        let (input, body) = take(body_length_bytes)(input)?;
        // TODO review if this is what we want: if we cannot copy the slice, return an empty vec?
        let body = body.to_vec();
        Ok((input, PduBody::Other(Other { body })))
    }
}

#[cfg(test)]
mod tests {
    use crate::common::builder::PduHeaderBuilder;
    use crate::common::model::{PduBody, PduType, ProtocolFamily, ProtocolVersion};
    use crate::common::other::parser::other_body;
    use crate::common::symbolic_names::PDU_HEADER_LEN_BYTES;

    #[test]
    fn parse_other_body() {
        let header = PduHeaderBuilder::new()
            .protocol_version(ProtocolVersion::Ieee1278_1a_1998)
            .exercise_id(1)
            .pdu_type(PduType::OtherPdu)
            .protocol_family(ProtocolFamily::Other)
            .pdu_length((PDU_HEADER_LEN_BYTES + 10) as u16)
            .time_stamp(0)
            .build().expect("Should be good");
        let input : [u8;10] = [0x01,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00];
        let (input, body) = other_body(&header)(&input).expect("Should be Ok");
        if let PduBody::Other(pdu) = body {
            assert_eq!(pdu.body.len(), 10);
            assert_eq!(*pdu.body.get(0).unwrap(), 1u8);
        }
        assert!(input.is_empty());
    }
}