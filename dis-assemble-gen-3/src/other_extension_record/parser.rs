use nom::IResult;

pub fn other_body(
    record_length: usize,
) -> impl Fn(&[u8]) -> IResult<&[u8], crate::ExtensionRecordBody> {
    move |input: &[u8]| {
        let body_length = record_length.saturating_sub(2);
        let (input, body) = nom::bytes::complete::take(body_length)(input)?;

        Ok((
            input,
            crate::ExtensionRecordBody::Other(crate::other_extension_record::model::Other {
                body: body.to_vec(),
            }),
        ))
    }
}
