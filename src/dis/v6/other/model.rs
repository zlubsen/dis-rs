use crate::dis::v6::model::PduHeader;
use crate::dis::v6::other::builder::OtherBuilder;

pub struct Other {
    pub header: PduHeader,
    pub body: Vec<u8>,
}

impl Other {
    pub fn builder() -> OtherBuilder {
        OtherBuilder::new()
    }
}