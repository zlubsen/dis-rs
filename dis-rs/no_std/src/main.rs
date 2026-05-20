#![no_std]
#![no_main]

extern crate alloc;

use bytes::BytesMut;
use cortex_m as _;
use cortex_m_rt::entry;
use defmt_semihosting as _;
use dis_rs::{
    BodyInfo,
    entity_state::model::{
        DrEulerAngles, DrOtherParameters, DrParameters, EntityMarking, EntityState,
    },
    enumerations::{
        AirPlatformAppearance, AirPlatformCapabilities, ArticulatedPartsTypeClass,
        ArticulatedPartsTypeMetric, Country, CoupledExtensionIndicator, DeadReckoningAlgorithm,
        EntityCapabilities, EntityKind, EntityMarkingCharacterSet, ForceId, LvcIndicator,
        PlatformDomain, ProtocolVersion, TransferredEntityIndicator,
    },
    model::{
        ArticulatedPart, EntityId, EntityType, Location, Orientation, Pdu, PduBody, PduHeader,
        PduStatus, SimulationAddress, TimeUnits, Timestamp, VariableParameter, VectorF32,
    },
};
use embedded_alloc::LlffHeap as Heap;

#[global_allocator]
static HEAP: Heap = Heap::empty();

#[entry]
#[allow(clippy::too_many_lines)]
fn main() -> ! {
    // Initialize allocator
    defmt::info!("initializing allocator");
    unsafe {
        embedded_alloc::init!(HEAP, 8192);
    }
    defmt::info!("allocator initialized");

    // PDU body
    defmt::info!("constructing pdu body");
    let pdu_body = PduBody::EntityState(EntityState {
        entity_id: EntityId {
            simulation_address: SimulationAddress {
                site_id: 1,
                application_id: 2,
            },
            entity_id: 3,
        },
        force_id: ForceId::Friendly,
        entity_type: EntityType {
            kind: EntityKind::Platform,
            domain: PlatformDomain::Air,
            country: Country::Italy_ITA_,
            category: 4,
            subcategory: 5,
            specific: 6,
            extra: 7,
        },
        alternative_entity_type: EntityType::default(),
        entity_linear_velocity: VectorF32 {
            first_vector_component: 1.0,
            second_vector_component: 2.0,
            third_vector_component: 3.0,
        },
        entity_location: Location {
            x_coordinate: 1.0,
            y_coordinate: 2.0,
            z_coordinate: 3.0,
        },
        entity_orientation: Orientation {
            psi: 1.0,
            theta: 2.0,
            phi: 3.0,
        },
        entity_appearance: dis_rs::entity_state::model::EntityAppearance::AirPlatform(
            AirPlatformAppearance::default(),
        ),
        dead_reckoning_parameters: DrParameters {
            algorithm: DeadReckoningAlgorithm::StaticNonmovingEntity,
            other_parameters: DrOtherParameters::LocalEulerAngles(DrEulerAngles {
                local_yaw: 1.0,
                local_pitch: 2.0,
                local_roll: 3.0,
            }),
            linear_acceleration: VectorF32::default(),
            angular_velocity: VectorF32::default(),
        },
        entity_marking: EntityMarking::new("Ferris", EntityMarkingCharacterSet::ASCII),
        entity_capabilities: EntityCapabilities::AirPlatformEntityCapabilities(
            AirPlatformCapabilities::default(),
        ),
        variable_parameters: alloc::vec![
            VariableParameter::Articulated(ArticulatedPart {
                change_indicator: 213,
                attachment_id: 0,
                type_metric: ArticulatedPartsTypeMetric::Azimuth,
                type_class: ArticulatedPartsTypeClass::PrimaryTurretNumber1,
                parameter_value: -0.305
            }),
            VariableParameter::Articulated(ArticulatedPart {
                change_indicator: 45,
                attachment_id: 0,
                type_metric: ArticulatedPartsTypeMetric::AzimuthRate,
                type_class: ArticulatedPartsTypeClass::PrimaryTurretNumber1,
                parameter_value: -0.058
            }),
            VariableParameter::Articulated(ArticulatedPart {
                change_indicator: 187,
                attachment_id: 1,
                type_metric: ArticulatedPartsTypeMetric::Elevation,
                type_class: ArticulatedPartsTypeClass::PrimaryGunNumber1,
                parameter_value: 0.267
            }),
            VariableParameter::Articulated(ArticulatedPart {
                change_indicator: 34,
                attachment_id: 1,
                type_metric: ArticulatedPartsTypeMetric::ElevationRate,
                type_class: ArticulatedPartsTypeClass::PrimaryGunNumber1,
                parameter_value: 0.384
            })
        ],
    });
    defmt::info!("pdu body constructed");

    // PDU
    defmt::info!("constructing pdu");
    let pdu = Pdu::finalize_from_parts(
        PduHeader::new(ProtocolVersion::IEEE1278_12012, 1, pdu_body.body_type()).with_pdu_status(
            PduStatus::default()
                .with_transferred_entity_indicator(TransferredEntityIndicator::NoDifference)
                .with_lvc_indicator(LvcIndicator::NoStatement)
                .with_coupled_extension_indicator(CoupledExtensionIndicator::NotCoupled),
        ),
        pdu_body,
        Timestamp::Absolute(TimeUnits::MAX),
    );
    defmt::info!("constructed pdu");

    // Serialize
    defmt::info!("serializing pdu");
    let mut buf = BytesMut::with_capacity(usize::from(pdu.pdu_length()));
    pdu.serialize(&mut buf).unwrap();
    defmt::info!("pdu serialized");

    // Deserialize
    defmt::info!("deserializing pdu");
    let pdus = dis_rs::parse(&buf).unwrap();
    defmt::info!("pdu deserialized");

    // Checks
    assert_eq!(pdus.len(), 1);
    assert_eq!(pdu, pdus[0]);

    semihosting::process::ExitCode::SUCCESS.exit_process();
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    defmt::error!("{}", info);
    semihosting::process::ExitCode::FAILURE.exit_process();
}
