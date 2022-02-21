use crate::dis::v7::model::PduHeader;
use crate::dis::v7::other::builder::OtherBuilder;

pub struct Other {
    pub header: PduHeader,
    pub body: Vec<u8>,
}

impl Other {
    pub fn builder() -> OtherBuilder {
        OtherBuilder::new()
    }
}