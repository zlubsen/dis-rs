use crate::constants::{NINE_BITS, TWO_BITS};
use crate::parsing::{take_signed, BitInput};
use crate::receiver::model::Receiver;
use crate::records::parser::entity_identification;
use crate::types::parser::uvint16;
use crate::{BodyProperties, CdisBody};
use dis_rs::enumerations::ReceiverState;
use nom::bits::complete::take;
use nom::IResult;

pub(crate) fn receiver_body(input: BitInput) -> IResult<BitInput, CdisBody> {
    let (input, radio_reference_id) = entity_identification(input)?;
    let (input, radio_number) = uvint16(input)?;

    let (input, receiver_state): (BitInput, u16) = take(TWO_BITS)(input)?;
    let receiver_state = ReceiverState::from(receiver_state);

    let (input, received_power): (BitInput, isize) = take_signed(NINE_BITS)(input)?;

    let (input, transmitter_radio_reference_id) = entity_identification(input)?;
    let (input, transmitter_radio_number) = uvint16(input)?;

    Ok((
        input,
        Receiver {
            radio_reference_id,
            radio_number,
            receiver_state,
            received_power: received_power as i16,
            transmitter_radio_reference_id,
            transmitter_radio_number,
        }
        .into_cdis_body(),
    ))
}
