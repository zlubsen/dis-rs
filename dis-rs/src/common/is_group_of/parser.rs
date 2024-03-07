use nom::IResult;
use nom::multi::count;
use nom::number::complete::{be_f64, be_u16, be_u32, be_u8};
use crate::common::parser::entity_id;
use crate::entity_state::parser::entity_appearance;
use crate::enumerations::{EntityKind, IsGroupOfGroupedEntityCategory, PlatformDomain};
use crate::is_group_of::model::{GEDEntityLocation, GEDEntityOrientation, GEDRecord1, GEDRecord2, GEDRecord3, GEDRecord4, GEDRecord5, GEDRecord6, GEDRecord7, GEDRecord8, GEDRecord9, GroupEntityDescription, GroupReferencePoint, IsGroupOf};
use crate::model::{EntityType, PduBody};

pub(crate) fn is_group_of_body(input: &[u8]) -> IResult<&[u8], PduBody> {
    let (input, group_id) = entity_id(input)?;
    let (input, category) = be_u8(input)?;
    let category = IsGroupOfGroupedEntityCategory::from(category);
    let (input, number_of_entities) = be_u8(input)?;
    let (input, _padding) = be_u32(input)?;
    let (input, ref_point) = group_reference_point(input)?;
    let (input, descriptions) = count(
        group_entity_description(&category), number_of_entities.into())(input)?;

    Ok((input, IsGroupOf::builder()
        .with_group_id(group_id)
        .with_grouped_entity_category(category)
        .with_group_reference_point(ref_point)
        .with_descriptions(descriptions)
        .build()
        .into_pdu_body()))
}

fn group_reference_point(input: &[u8]) -> IResult<&[u8], GroupReferencePoint> {
    let (input, lat) = be_f64(input)?;
    let (input, lon) = be_f64(input)?;

    Ok((input, GroupReferencePoint::default()
        .with_latitude(lat)
        .with_longitude(lon)))
}

fn group_entity_description(category: &IsGroupOfGroupedEntityCategory) -> impl Fn(&[u8]) -> IResult<&[u8], GroupEntityDescription> + '_ {
    move |input: &[u8]| {
        let (input, ged) = match category {
            IsGroupOfGroupedEntityCategory::Undefined =>
                { (input, GroupEntityDescription::Undefined) }
            IsGroupOfGroupedEntityCategory::BasicGroundCombatVehicle =>
                { ged_record_1(input)? }
            IsGroupOfGroupedEntityCategory::EnhancedGroundCombatVehicle =>
                { ged_record_2(input)? }
            IsGroupOfGroupedEntityCategory::BasicGroundCombatSoldier =>
                { ged_record_3(input)? }
            IsGroupOfGroupedEntityCategory::EnhancedGroundCombatSoldier =>
                { ged_record_4(input)? }
            IsGroupOfGroupedEntityCategory::BasicRotorWingAircraft =>
                { ged_record_5(input)? }
            IsGroupOfGroupedEntityCategory::EnhancedRotorWingAircraft =>
                { ged_record_6(input)? }
            IsGroupOfGroupedEntityCategory::BasicFixedWingAircraft =>
                { ged_record_7(input)? }
            IsGroupOfGroupedEntityCategory::EnhancedFixedWingAircraft =>
                { ged_record_8(input)? }
            IsGroupOfGroupedEntityCategory::GroundLogisticsVehicle =>
                { ged_record_9(input)? }
            IsGroupOfGroupedEntityCategory::Unspecified(_) =>
                { (input, GroupEntityDescription::Undefined) }
        };

        Ok((input, ged))
    }
}

fn ged_record_1(input: &[u8]) -> IResult<&[u8], GroupEntityDescription> {
    let (input, entity_id) = be_u16(input)?;
    let (input, location) = ged_entity_location(input)?;
    let (input, appearance) = entity_appearance(
        EntityType::default()
            .with_kind(EntityKind::Platform)
            .with_domain(PlatformDomain::Land))(input)?;
    let (input, orientation) = ged_entity_orientation(input)?;
    let (input, speed) = be_u8(input)?;
    let (input, turret_azimuth) = be_u8(input)?;
    let (input, gun_elevation) = be_u8(input)?;
    let (input, turret_slew_rate) = be_u8(input)?;
    let (input, gun_elevation_rate) = be_u8(input)?;

    Ok((input, GroupEntityDescription::BasicGroundCombatVehicle(
        GEDRecord1 {
            entity_id,
            location,
            appearance,
            orientation,
            speed,
            turret_azimuth,
            gun_elevation,
            turret_slew_rate,
            gun_elevation_rate,
        }
    )))
}

fn ged_record_2(input: &[u8]) -> IResult<&[u8], GroupEntityDescription> {
    let (input, ged_record) = ged_record_1(input)?;
    let ged_record = if let GroupEntityDescription::BasicGroundCombatVehicle(ged_record) = ged_record { ged_record } else { GEDRecord1::default() };
    let (input, fuel_status) = be_u8(input)?;
    let (input, ground_maintenance_status) = be_u8(input)?;
    let (input, primary_ammunition) = be_u8(input)?;
    let (input, secondary_ammunition) = be_u8(input)?;

    Ok((input, GroupEntityDescription::EnhancedGroundCombatVehicle(
        GEDRecord2 {
            basic_ground_combat_vehicle: ged_record,
            fuel_status,
            ground_maintenance_status,
            primary_ammunition,
            secondary_ammunition,
        }
    )))
}

fn ged_record_3(input: &[u8]) -> IResult<&[u8], GroupEntityDescription> {
    let (input, entity_id) = be_u16(input)?;
    let (input, location) = ged_entity_location(input)?;
    let (input, appearance) = entity_appearance(
        EntityType::default()
            .with_kind(EntityKind::Platform)
            .with_domain(PlatformDomain::Land))(input)?;
    let (input, orientation) = ged_entity_orientation(input)?;
    let (input, speed) = be_u8(input)?;
    let (input, head_azimuth) = be_u8(input)?;
    let (input, head_elevation) = be_u8(input)?;
    let (input, head_scan_rate) = be_u8(input)?;
    let (input, head_elevation_rate) = be_u8(input)?;

    Ok((input, GroupEntityDescription::BasicGroundCombatSoldier(
        GEDRecord3 {
            entity_id,
            location,
            appearance,
            orientation,
            speed,
            head_azimuth,
            head_elevation,
            head_scan_rate,
            head_elevation_rate,
        }
    )))
}

fn ged_record_4(input: &[u8]) -> IResult<&[u8], GroupEntityDescription> {
    let (input, ged_record) = ged_record_3(input)?;
    let ged_record = if let GroupEntityDescription::BasicGroundCombatSoldier(ged_record) = ged_record { ged_record } else { GEDRecord3::default() };
    let (input, water_status) = be_u8(input)?;
    let (input, reset_status) = be_u8(input)?;
    let (input, primary_ammunition) = be_u8(input)?;
    let (input, secondary_ammunition) = be_u8(input)?;

    Ok((input, GroupEntityDescription::EnhancedGroundCombatSoldier(
        GEDRecord4 {
            basic_ground_combat_soldier: ged_record,
            water_status,
            reset_status,
            primary_ammunition,
            secondary_ammunition,
        }
    )))
}

fn ged_record_5(input: &[u8]) -> IResult<&[u8], GroupEntityDescription> {
    let (input, entity_id) = be_u16(input)?;
    let (input, location) = ged_entity_location(input)?;
    let (input, appearance) = entity_appearance(
        EntityType::default()
            .with_kind(EntityKind::Platform)
            .with_domain(PlatformDomain::Air))(input)?;
    let (input, orientation) = ged_entity_orientation(input)?;
    let (input, fuel_status) = be_u8(input)?;
    let (input, movement_horizontal_deviation) = be_u8(input)?;
    let (input, movement_vertical_deviation) = be_u8(input)?;
    let (input, movement_speed) = be_u16(input)?;
    let (input, turret_azimuth) = be_u8(input)?;
    let (input, gun_elevation) = be_u8(input)?;
    let (input, turret_scan_rate) = be_u8(input)?;
    let (input, gun_elevation_rate) = be_u8(input)?;

    Ok((input, GroupEntityDescription::BasicRotorWingAircraft(
        GEDRecord5 {
            entity_id,
            location,
            appearance,
            orientation,
            fuel_status,
            movement_horizontal_deviation,
            movement_vertical_deviation,
            movement_speed,
            turret_azimuth,
            gun_elevation,
            turret_scan_rate,
            gun_elevation_rate,
        }
    )))
}

fn ged_record_6(input: &[u8]) -> IResult<&[u8], GroupEntityDescription> {
    let (input, ged_record) = ged_record_5(input)?;
    let ged_record = if let GroupEntityDescription::BasicRotorWingAircraft(ged_record) = ged_record { ged_record } else { GEDRecord5::default() };
    let (input, supplemental_fuel_status) = be_u8(input)?;
    let (input, air_maintenance_status) = be_u8(input)?;
    let (input, primary_ammunition) = be_u8(input)?;
    let (input, secondary_ammunition) = be_u8(input)?;

    Ok((input, GroupEntityDescription::EnhancedRotorWingAircraft(
        GEDRecord6 {
            basic_rotor_wing_aircraft: ged_record,
            supplemental_fuel_status,
            air_maintenance_status,
            primary_ammunition,
            secondary_ammunition,
        }
    )))
}

fn ged_record_7(input: &[u8]) -> IResult<&[u8], GroupEntityDescription> {
    let (input, entity_id) = be_u16(input)?;
    let (input, location) = ged_entity_location(input)?;
    let (input, appearance) = entity_appearance(
        EntityType::default()
            .with_kind(EntityKind::Platform)
            .with_domain(PlatformDomain::Air))(input)?;
    let (input, orientation) = ged_entity_orientation(input)?;
    let (input, fuel_status) = be_u8(input)?;
    let (input, movement_horizontal_deviation) = be_u8(input)?;
    let (input, movement_vertical_deviation) = be_u8(input)?;
    let (input, movement_speed) = be_u16(input)?;

    Ok((input, GroupEntityDescription::BasicFixedWingAircraft(
        GEDRecord7 {
            entity_id,
            location,
            appearance,
            orientation,
            fuel_status,
            movement_horizontal_deviation,
            movement_vertical_deviation,
            movement_speed,
        }
    )))
}

fn ged_record_8(input: &[u8]) -> IResult<&[u8], GroupEntityDescription> {
    let (input, ged_record) = ged_record_7(input)?;
    let ged_record = if let GroupEntityDescription::BasicFixedWingAircraft(ged_record) = ged_record { ged_record } else { GEDRecord7::default() };
    let (input, supplemental_fuel_status) = be_u8(input)?;
    let (input, air_maintenance_status) = be_u8(input)?;
    let (input, primary_ammunition) = be_u8(input)?;
    let (input, secondary_ammunition) = be_u8(input)?;

    Ok((input, GroupEntityDescription::EnhancedFixedWingAircraft(
        GEDRecord8 {
            basic_fixed_wing_aircraft: ged_record,
            supplemental_fuel_status,
            air_maintenance_status,
            primary_ammunition,
            secondary_ammunition,
        }
    )))
}

fn ged_record_9(input: &[u8]) -> IResult<&[u8], GroupEntityDescription> {
    let (input, entity_id) = be_u16(input)?;
    let (input, location) = ged_entity_location(input)?;
    let (input, appearance) = entity_appearance(
        EntityType::default()
            .with_kind(EntityKind::Platform)
            .with_domain(PlatformDomain::Land))(input)?;
    let (input, orientation) = ged_entity_orientation(input)?;
    let (input, speed) = be_u16(input)?;

    Ok((input, GroupEntityDescription::GroundLogisticsVehicle(
        GEDRecord9 {
            entity_id,
            location,
            appearance,
            orientation,
            speed,
        }
    )))
}

fn ged_entity_location(input: &[u8]) -> IResult<&[u8], GEDEntityLocation> {
    let (input, x_offset) = be_u16(input)?;
    let (input, y_offset) = be_u16(input)?;
    let (input, z_offset) = be_u16(input)?;

    Ok((input, GEDEntityLocation {
        x_offset,
        y_offset,
        z_offset,
    }))
}

fn ged_entity_orientation(input: &[u8]) -> IResult<&[u8], GEDEntityOrientation> {
    let (input, psi) = be_u8(input)?;
    let (input, theta) = be_u8(input)?;
    let (input, phi) = be_u8(input)?;

    Ok((input, GEDEntityOrientation {
        psi,
        theta,
        phi,
    } ))
}
