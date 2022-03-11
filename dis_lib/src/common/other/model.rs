use crate::common::other::builder::OtherBuilder;

pub struct Other {
    pub body: Vec<u8>,
}

impl Other {
    pub fn builder() -> OtherBuilder {
        OtherBuilder::new()
    }
}