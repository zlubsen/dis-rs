use crate::dis::v7::model::PduHeader;
use crate::dis::v7::other::model::Other;

pub struct OtherBuilder {
    header : Option<PduHeader>,
    body : Option<Vec<u8>>,
}

impl OtherBuilder {
    pub fn new() -> OtherBuilder {
        OtherBuilder {
            header: None,
            body: None
        }
    }

    pub fn header(mut self, header: PduHeader) -> Self {
        self.header = Some(header);
        self
    }

    pub fn body(mut self, bytes : Vec<u8>) -> Self {
        self.body = Some(bytes);
        self
    }

    pub fn validate(&self) -> bool {
        return self.header.is_some() && self.body.is_some()
    }

    pub fn build(self) -> Result<Other, ()> {
        if self.validate() {
            Ok(Other {
                header: self.header.expect("should be set"),
                body: self.body.expect("should be set"),
            })
        } else {
            Err(())
        }
    }
}