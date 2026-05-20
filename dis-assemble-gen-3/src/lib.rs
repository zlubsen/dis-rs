#![allow(clippy::similar_names)]

pub(crate) mod constants;
pub mod core;
mod impls;
pub(crate) mod other_extension_record;
pub(crate) mod other_pdu;
pub mod symbolic_names;
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

// TODO ux functions for Extension Records: into_er_body(), ExtrecBody type to ExtRecTypes enum
// TODO consistency tests for all PDUs
// TODO model/generate the hierarchy for EntityType elements (enum hierarchy dependencies) from SISO-REF-010
// TODO model the hierarchy of EntityType (eg domain depends on kind, etc)
// TODO put symbolic names in common crate for gen2 and gen3
// TODO add/check symbolic names for variable parameters (and put in common crate for gen2 and gen3), tables 10, 11, 12.
// TODO further develop the core traits (as in gen 2: BodyInfo, Interaction)
// TODO impls - new: WorldCoordinates, EurlerAngles, PlacementAttributes
