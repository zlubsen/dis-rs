use nom::IResult;
use nom::multi::count;
use nom::number::complete::{be_f32, be_u16, be_u8};
use crate::common::service_request::model::ServiceRequest;
use crate::common::parser::{entity_id, entity_type};
use crate::common::model::PduBody;
use crate::enumerations::ServiceRequestServiceTypeRequested;
use crate::service_request::model::SupplyQuantity;

pub fn service_request_body(input: &[u8]) -> IResult<&[u8], PduBody> {
    let (input, requesting_id) = entity_id(input)?;
    let (input, servicing_id) = entity_id(input)?;
    let (input, service_type_requested) = be_u8(input)?;
    let service_type_requested = ServiceRequestServiceTypeRequested::from(service_type_requested);
    let (input, nr_of_supplies) = be_u8(input)?;
    let (input, _padding) = be_u16(input)?;
    let (input, supplies) = count(supply_quantity, nr_of_supplies.into())(input)?;

    let body = ServiceRequest::builder()
        .with_requesting_id(requesting_id)
        .with_servicing_id(servicing_id)
        .with_service_type_requested(service_type_requested)
        .with_supplies(supplies)
        .build();

    Ok((input, body.into_pdu_body()))
}

fn supply_quantity(input: &[u8]) -> IResult<&[u8], SupplyQuantity> {
    let (input, supply_type) = entity_type(input)?;
    let (input, quantity) = be_f32(input)?;

    Ok((input, SupplyQuantity::default()
        .with_supply_type(supply_type)
        .with_quantity(quantity)))
}