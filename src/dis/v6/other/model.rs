use crate::dis::v6::model::PduHeader;

pub struct Other {
    pub header: PduHeader,
    pub body: Vec<u8>,
}