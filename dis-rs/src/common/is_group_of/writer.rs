use crate::is_group_of::model::{
    GEDEntityLocation, GEDEntityOrientation, GEDRecord1, GEDRecord2, GEDRecord3, GEDRecord4,
    GEDRecord5, GEDRecord6, GEDRecord7, GEDRecord8, GEDRecord9, GroupEntityDescription,
    GroupReferencePoint, IsGroupOf,
};
use crate::{Serialize, SerializePdu, SupportedVersion};
use bytes::{BufMut, BytesMut};

impl SerializePdu for IsGroupOf {
    fn serialize_pdu(&self, _version: SupportedVersion, buf: &mut BytesMut) -> u16 {
        let group_id_bytes = self.group_id.serialize(buf);
        buf.put_u8(self.grouped_entity_category.into());
        buf.put_u8(self.descriptions.len() as u8);
        buf.put_u32(0u32);
        let ref_point_bytes = self.group_reference_point.serialize(buf);
        let descriptions_bytes = self
            .descriptions
            .iter()
            .map(|ged_record| ged_record.serialize(buf))
            .sum::<u16>();

        group_id_bytes + ref_point_bytes + descriptions_bytes + 6
    }
}

impl Serialize for GroupReferencePoint {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        buf.put_f64(self.latitude);
        buf.put_f64(self.longitude);

        16
    }
}

impl Serialize for GroupEntityDescription {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        match self {
            GroupEntityDescription::Undefined => 0,
            GroupEntityDescription::BasicGroundCombatVehicle(ged_record) => {
                ged_record.serialize(buf)
            }
            GroupEntityDescription::EnhancedGroundCombatVehicle(ged_record) => {
                ged_record.serialize(buf)
            }
            GroupEntityDescription::BasicGroundCombatSoldier(ged_record) => {
                ged_record.serialize(buf)
            }
            GroupEntityDescription::EnhancedGroundCombatSoldier(ged_record) => {
                ged_record.serialize(buf)
            }
            GroupEntityDescription::BasicRotorWingAircraft(ged_record) => ged_record.serialize(buf),
            GroupEntityDescription::EnhancedRotorWingAircraft(ged_record) => {
                ged_record.serialize(buf)
            }
            GroupEntityDescription::BasicFixedWingAircraft(ged_record) => ged_record.serialize(buf),
            GroupEntityDescription::EnhancedFixedWingAircraft(ged_record) => {
                ged_record.serialize(buf)
            }
            GroupEntityDescription::GroundLogisticsVehicle(ged_record) => ged_record.serialize(buf),
        }
    }
}

impl Serialize for GEDEntityLocation {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        buf.put_u16(self.x_offset);
        buf.put_u16(self.y_offset);
        buf.put_u16(self.z_offset);

        self.record_length()
    }
}

impl Serialize for GEDEntityOrientation {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        buf.put_u8(self.psi);
        buf.put_u8(self.theta);
        buf.put_u8(self.phi);

        self.record_length()
    }
}

impl Serialize for GEDRecord1 {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        buf.put_u16(self.entity_id);
        self.location.serialize(buf);
        self.appearance.serialize(buf);
        self.orientation.serialize(buf);
        buf.put_u8(self.speed);
        buf.put_u8(self.turret_azimuth);
        buf.put_u8(self.gun_elevation);
        buf.put_u8(self.turret_slew_rate);
        buf.put_u8(self.gun_elevation_rate);

        self.record_length()
    }
}

impl Serialize for GEDRecord2 {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        self.basic_ground_combat_vehicle.serialize(buf);
        buf.put_u8(self.fuel_status);
        buf.put_u8(self.ground_maintenance_status);
        buf.put_u8(self.primary_ammunition);
        buf.put_u8(self.secondary_ammunition);

        self.record_length()
    }
}

impl Serialize for GEDRecord3 {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        buf.put_u16(self.entity_id);
        self.location.serialize(buf);
        self.appearance.serialize(buf);
        self.orientation.serialize(buf);
        buf.put_u8(self.speed);
        buf.put_u8(self.head_azimuth);
        buf.put_u8(self.head_elevation);
        buf.put_u8(self.head_scan_rate);
        buf.put_u8(self.head_elevation_rate);

        self.record_length()
    }
}

impl Serialize for GEDRecord4 {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        self.basic_ground_combat_soldier.serialize(buf);
        buf.put_u8(self.water_status);
        buf.put_u8(self.reset_status);
        buf.put_u8(self.primary_ammunition);
        buf.put_u8(self.secondary_ammunition);

        self.record_length()
    }
}

impl Serialize for GEDRecord5 {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        buf.put_u16(self.entity_id);
        self.location.serialize(buf);
        self.appearance.serialize(buf);
        self.orientation.serialize(buf);
        buf.put_u8(self.fuel_status);
        buf.put_u8(self.movement_horizontal_deviation);
        buf.put_u8(self.movement_vertical_deviation);
        buf.put_u16(self.movement_speed);
        buf.put_u8(self.turret_azimuth);
        buf.put_u8(self.gun_elevation);
        buf.put_u8(self.turret_scan_rate);
        buf.put_u8(self.gun_elevation_rate);

        self.record_length()
    }
}

impl Serialize for GEDRecord6 {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        self.basic_rotor_wing_aircraft.serialize(buf);
        buf.put_u8(self.supplemental_fuel_status);
        buf.put_u8(self.air_maintenance_status);
        buf.put_u8(self.primary_ammunition);
        buf.put_u8(self.secondary_ammunition);

        self.record_length()
    }
}

impl Serialize for GEDRecord7 {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        buf.put_u16(self.entity_id);
        self.location.serialize(buf);
        self.appearance.serialize(buf);
        self.orientation.serialize(buf);
        buf.put_u8(self.fuel_status);
        buf.put_u8(self.movement_horizontal_deviation);
        buf.put_u8(self.movement_vertical_deviation);
        buf.put_u16(self.movement_speed);

        self.record_length()
    }
}

impl Serialize for GEDRecord8 {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        self.basic_fixed_wing_aircraft.serialize(buf);
        buf.put_u8(self.supplemental_fuel_status);
        buf.put_u8(self.air_maintenance_status);
        buf.put_u8(self.primary_ammunition);
        buf.put_u8(self.secondary_ammunition);

        self.record_length()
    }
}

impl Serialize for GEDRecord9 {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        buf.put_u16(self.entity_id);
        self.location.serialize(buf);
        self.appearance.serialize(buf);
        self.orientation.serialize(buf);
        buf.put_u16(self.speed);

        self.record_length()
    }
}
