use nom::IResult;
use nom::bits::complete::take;
use dis_rs::enumerations::PduType;
use dis_rs::model::TimeStamp;
use dis_rs::parse_pdu_status_fields;
use crate::constants::{EIGHT_BITS, FOURTEEN_BITS, TWENTY_SIX_BITS, TWO_BITS};
use crate::records::model::{CdisHeader, CdisProtocolVersion};

pub(crate) fn cdis_header(input: (&[u8], usize)) -> IResult<(&[u8], usize), CdisHeader> {
    let (input, protocol_version) : ((&[u8], usize), u8) = take(TWO_BITS)(input)?;
    let (input, exercise_id) = uvint8(input)?;
    let (input, pdu_type) : ((&[u8], usize), u8) = take(EIGHT_BITS)(input)?;
    let (input, timestamp) : ((&[u8], usize), u32) = take(TWENTY_SIX_BITS)(input)?;
    let (input, length) : ((&[u8], usize), u16) = take(FOURTEEN_BITS)(input)?;
    let (input, pdu_status) : ((&[u8], usize), u8) = take(EIGHT_BITS)(input)?;
    let pdu_status = parse_pdu_status_fields(pdu_type, pdu_status);

    Ok((input, CdisHeader {
        protocol_version: CdisProtocolVersion::from(protocol_version),
        exercise_id,
        pdu_type: PduType::from(pdu_type),
        timestamp: TimeStamp::from(timestamp),
        length,
        pdu_status,
    }))
}