use nom::bytes::complete::take;
use nom::IResult;
use crate::dis::v6::model::{Pdu, PDU_HEADER_LEN_BYTES, PduHeader};
use crate::dis::v6::other::model::Other as OtherStruct;

pub fn other_body(header: PduHeader) -> impl Fn(&[u8]) -> IResult<&[u8], Pdu> {
    move | input: &[u8] | {
        let body_length_bytes = header.pdu_length as usize - PDU_HEADER_LEN_BYTES;
        let (input, body) = take(body_length_bytes)(input)?;
        // TODO review if this is what we want: if we cannot copy the slice, return an empty vec?
        let body = body.to_vec();
        // let body: Vec<u8> = body.try_into().unwrap_or(Default::default());
        Ok((input, Pdu::Other(OtherStruct { header, body })))
    }
}