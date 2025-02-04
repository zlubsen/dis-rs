use crate::common::model::PduBody;
use crate::common::parser::{entity_id, supply_quantity};
use crate::common::resupply_offer::model::ResupplyOffer;
use nom::multi::count;
use nom::number::complete::{be_u16, be_u8};
use nom::IResult;

pub(crate) fn resupply_offer_body(input: &[u8]) -> IResult<&[u8], PduBody> {
    let (input, requesting_id) = entity_id(input)?;
    let (input, servicing_id) = entity_id(input)?;
    let (input, nr_of_supplies) = be_u8(input)?;
    let (input, _padding) = be_u8(input)?;
    let (input, _padding) = be_u16(input)?;
    let (input, supplies) = count(supply_quantity, nr_of_supplies.into())(input)?;

    let body = ResupplyOffer::builder()
        .with_requesting_id(requesting_id)
        .with_servicing_id(servicing_id)
        .with_supplies(supplies)
        .build();

    Ok((input, body.into_pdu_body()))
}
