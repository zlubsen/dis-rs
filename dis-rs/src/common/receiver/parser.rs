use nom::IResult;
use nom::number::complete::{be_f32, be_u16};
use crate::common::parser::entity_id;
use crate::common::model::PduBody;
use crate::common::receiver::model::Receiver;
use crate::enumerations::ReceiverState;

pub fn receiver_body(input: &[u8]) -> IResult<&[u8], PduBody> {
    let (input, radio_reference_id) = entity_id(input)?;
    let (input, radio_number) = be_u16(input)?;
    let (input, receiver_state) = be_u16(input)?;
    let receiver_state = ReceiverState::from(receiver_state);
    let (input, _padding) = be_u16(input)?;
    let (input, received_power) = be_f32(input)?;
    let (input, transmitter_radio_reference_id) = entity_id(input)?;
    let (input, transmitter_radio_number) = be_u16(input)?;

    let body = Receiver::builder()
        .with_radio_reference_id(radio_reference_id)
        .with_radio_number(radio_number)
        .with_receiver_state(receiver_state)
        .with_received_power(received_power)
        .with_transmitter_radio_reference_id(transmitter_radio_reference_id)
        .with_transmitter_radio_number(transmitter_radio_number)
        .build();

    Ok((input, body.into_pdu_body()))
}