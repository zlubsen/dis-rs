include!(concat!(
    env!("OUT_DIR"),
    "/",
    env!("TARGET_GENERATED_SISO_REF_010_FILENAME")
));

include!(concat!(
    env!("OUT_DIR"),
    "/",
    env!("TARGET_GENERATED_SISO_1278_GEN3_FILENAME")
));

pub mod other {
    pub mod parser {
        use crate::PduBody;
        use nom::IResult;

        pub fn other_body(input: &[u8]) -> IResult<&[u8], PduBody> {
            todo!()
        }
    }
}
