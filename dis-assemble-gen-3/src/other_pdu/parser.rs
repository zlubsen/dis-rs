use crate::{BodyRaw, PduBody};
use nom::IResult;

pub fn other_body(
    header: &crate::common_records::PDUHeader,
) -> impl Fn(&[u8]) -> IResult<&[u8], PduBody> + use<'_> {
    move |input: &[u8]| {
        let body_length = header
            .pdu_length
            .saturating_sub(header.pdu_header_length as u16);
        let (input, body) = nom::bytes::complete::take(body_length)(input)?;

        Ok((
            input,
            crate::other_pdu::model::Other {
                body: body.to_vec(),
            }
            .into_pdu_body(),
        ))
    }
}
