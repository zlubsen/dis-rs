use crate::v6::entity_state::model::EntityCapabilities;
use nom::bits::bits;
use nom::bits::complete::take as take_bits;
use nom::bytes::complete::take as take_bytes;
use nom::error::Error;
use nom::IResult;

pub(crate) fn entity_capabilities(input: &[u8]) -> IResult<&[u8], EntityCapabilities> {
    let (input, (ammunition_supply, fuel_supply, recovery, repair, _pad_out)): (
        &[u8],
        (u8, u8, u8, u8, u8),
    ) = bits::<_, _, Error<(&[u8], usize)>, _, _>((
        take_bits(1usize),
        take_bits(1usize),
        take_bits(1usize),
        take_bits(1usize),
        take_bits(3usize),
    ))(input)?;
    let (input, _pad_3_bytes) = take_bytes(3usize)(input)?;

    Ok((
        input,
        EntityCapabilities {
            ammunition_supply: ammunition_supply != 0,
            fuel_supply: fuel_supply != 0,
            recovery: recovery != 0,
            repair: repair != 0,
        },
    ))
}
