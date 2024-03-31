use dis_rs::enumerations::{DeadReckoningAlgorithm, ForceId};
use crate::records::model::{AngularVelocity, CdisEntityMarking, CdisVariableParameter, EntityId, EntityType, LinearAcceleration, LinearVelocity, Orientation, Units, WorldCoordinates};
use crate::types::model::UVINT32;

#[derive(Clone, Debug, PartialEq)]
pub struct EntityState {
    pub units: Units,
    pub full_update_flag: bool,
    pub entity_id: EntityId,
    pub force_id: Option<ForceId>,
    pub entity_type: Option<EntityType>,
    pub alternate_entity_type: Option<EntityType>,
    pub entity_linear_velocity: Option<LinearVelocity>,
    pub entity_location: Option<WorldCoordinates>,
    pub entity_orientation: Option<Orientation>,
    pub entity_appearance: Option<CdisEntityAppearance>,
    pub dr_algorithm: DeadReckoningAlgorithm,
    pub dr_params_other: Option<u128>,
    pub dr_params_entity_linear_acceleration: Option<LinearAcceleration>,
    pub dr_params_entity_angular_velocity: Option<AngularVelocity>,
    pub entity_marking: Option<CdisEntityMarking>,
    pub capabilities: Option<CdisEntityCapabilities>,
    pub variable_parameters: Vec<CdisVariableParameter>
}

/// The Entity Appearance field is not explicitly modeled because the interpretation of the on-wire value
/// depends on the EntityType, which is not yet known when that field is not
/// present in the received C-DIS PDU.
/// This struct wraps the type in the wire format.
#[derive(Clone, Debug, PartialEq)]
pub struct CdisEntityAppearance(pub u32);

/// The Capabilities field is not explicitly modeled because the interpretation of the on-wire value
/// depends on the EntityType, which is not yet known when that field is not
/// present in the received C-DIS PDU.
/// This struct wraps the type in the wire format.
#[derive(Clone, Debug, PartialEq)]
pub struct CdisEntityCapabilities(pub UVINT32);

/// The DR Parameters Other field is not explicitly modeled because the interpretation of the on-wire value
/// depends on the DR Algorithm.
/// This struct wraps the type in the wire format.
#[derive(Clone, Debug, PartialEq)]
pub struct CdisDRParametersOther(pub Vec<u8>);

impl From<u128> for CdisDRParametersOther {
    fn from(value: u128) -> Self {
        const FIRST_BYTES_IS_EMPTY: usize = 0;
        let mut bytes = value.to_be_bytes().to_vec();
        let _ = bytes.remove(FIRST_BYTES_IS_EMPTY);
        Self(bytes)
    }
}