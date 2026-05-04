pub(crate) mod constants;
pub mod core;
pub mod fixed_parameters;
pub mod impls;
pub(crate) mod other_extension_record;
pub(crate) mod other_pdu;
pub(crate) mod utils;

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

pub use constants::*;
pub use core::*;
pub use other_pdu::*;

// TODO port convenience functions for EntityId and EntityType from gen2
// TODO model the hierarchy of EntityType (eg domain depends on kind, etc)
// TODO Tests for PDUs with extension records
// TODO test writers
// TODO consistency tests for all PDUs
// TODO model/generate the hierarchy for EntityType elements (enum hierarchy dependencies)
