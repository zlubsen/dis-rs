use std::time::Instant;
use crate::codec::CodecOptions;
use crate::designator::model::Designator;

type Counterpart = dis_rs::designator::model::Designator;

#[derive(Debug)]
pub struct EncoderStateDesignator {
    pub heartbeat: Instant,
}

impl Default for EncoderStateDesignator {
    fn default() -> Self {
        Self {
            heartbeat: Instant::now()
        }
    }
}

#[derive(Debug)]
pub struct DecoderStateDesignator {
    pub heartbeat: Instant,
}

impl Default for DecoderStateDesignator {
    fn default() -> Self {
        Self {
            heartbeat: Instant::now()
        }
    }
}

impl Designator {
    fn encode(item: &Self::Counterpart, state: Option<&EncoderStateDesignator>, options: &CodecOptions) -> Self {
        todo!()
    }

    fn decode(&self, state: Option<&DecoderStateDesignator>, options: &CodecOptions) -> Self::Counterpart {
        todo!()
    }
}