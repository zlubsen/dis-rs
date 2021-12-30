use nom::IResult;
use nom::number::complete::{u8, f32};
use nom::number::Endianness::Big;
use crate::dis::v6::entity_state::model::EntityState;
use crate::dis::v6::model::{Pdu, PduHeader};

pub fn entity_state_body(input: &[u8], header: PduHeader) -> IResult<&[u8], Pdu> {
    let (input, entity_id) = entity_id(input)?;
    let (input, force_id) = force_id(input)?;
    let (input, articulated_parts_no) = u8(input)?;
    let (input, entity_type) = entity_type(input)?;
    let (input, alternative_entity_type) = entity_type(input)?;
    let (input, entity_linear_velocity) = f32(Big)(input)?;
    let (input, entity_location) = location(input)?; // struct - 3x f64 be
    let (input, entity_orientation) = orientation(input)?; // struct - 3x f32 be
    let (input, entity_appearance) = appearance(input)?; // struct
    let (input, dead_reckoning_parameters) = dr_parameters(input)?; // struct
    let (input, entity_marking) = entity_marking(input)?; // struct
    let (input, entity_capabilities) = entity_capabilities(input)?; // struct
    let (input, articulation_parameter) =  match articulated_parts_no {
        0 => (input, None),
        n => articulation_records(&articulated_parts_no)(input),
    };

    Ok((input, Pdu::EntityState(
        EntityState::builder()
            // TODO insert all fields
            .build())))
}