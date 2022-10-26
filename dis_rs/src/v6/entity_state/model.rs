use crate::VectorF32;
use crate::enumerations::{DeadReckoningAlgorithm};

#[derive(Debug, PartialEq, Default)]
pub struct EntityCapabilities {
    pub ammunition_supply : bool,
    pub fuel_supply : bool,
    pub recovery : bool,
    pub repair : bool,
}

pub struct DrParameters {
    pub algorithm : DeadReckoningAlgorithm,
    pub other_parameters : [u8; 15],
    pub linear_acceleration : VectorF32,
    pub angular_velocity : VectorF32,
}

impl Default for DrParameters {
    fn default() -> Self {
        Self {
            algorithm: DeadReckoningAlgorithm::default(),
            other_parameters: [0u8; 15],
            linear_acceleration: VectorF32::default(),
            angular_velocity: VectorF32::default(),
        }
    }
}
