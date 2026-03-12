use crate::common::model::EntityType;
use crate::enumerations::EntityCapabilities;
use crate::v7::entity_state::entity_capabilities_from_bytes;
use nom::number::complete::be_u32;
use nom::IResult;

pub fn entity_capabilities(
    entity_type: EntityType,
) -> impl Fn(&[u8]) -> IResult<&[u8], EntityCapabilities> {
    move |input: &[u8]| {
        let (input, capabilities) = be_u32(input)?;
        let capabilities = entity_capabilities_from_bytes(capabilities, &entity_type);

        Ok((input, capabilities))
    }
}
