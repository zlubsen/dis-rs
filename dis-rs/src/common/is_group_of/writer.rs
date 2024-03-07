use bytes::{BufMut, BytesMut};
use crate::is_group_of::model::{GEDRecord1, GEDRecord2, GEDRecord3, GEDRecord4, GEDRecord5, GEDRecord6, GEDRecord7, GEDRecord8, GEDRecord9, GroupEntityDescription, GroupReferencePoint, IsGroupOf};
use crate::{Serialize, SerializePdu, SupportedVersion};

impl SerializePdu for IsGroupOf {
    fn serialize_pdu(&self, _version: SupportedVersion, buf: &mut BytesMut) -> u16 {
        let group_id_bytes= self.group_id.serialize(buf);
        buf.put_u8(self.grouped_entity_category.into());
        buf.put_u8(self.descriptions.len() as u8);
        buf.put_u32(0u32);
        let ref_point_bytes = self.group_reference_point.serialize(buf);
        let descriptions_bytes = self.descriptions.iter()
            .map(|ged_record| ged_record.serialize(buf) )
            .sum::<u16>();

        group_id_bytes + ref_point_bytes + descriptions_bytes + 6
    }
}

impl Serialize for GroupReferencePoint {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        buf.put_f64(self.latitude);
        buf.put_f64(self.longitude);

        8
    }
}

impl Serialize for GroupEntityDescription {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        match self {
            GroupEntityDescription::Undefined => { 0 }
            GroupEntityDescription::BasicGroundCombatVehicle(ged_record) => { ged_record.serialize(buf) }
            GroupEntityDescription::EnhancedGroundCombatVehicle(ged_record) => { ged_record.serialize(buf) }
            GroupEntityDescription::BasicGroundCombatSoldier(ged_record) => { ged_record.serialize(buf) }
            GroupEntityDescription::EnhancedGroundCombatSoldier(ged_record) => { ged_record.serialize(buf) }
            GroupEntityDescription::BasicRotorWingAircraft(ged_record) => { ged_record.serialize(buf) }
            GroupEntityDescription::EnhancedRotorWingAircraft(ged_record) => { ged_record.serialize(buf) }
            GroupEntityDescription::BasicFixedWingAircraft(ged_record) => { ged_record.serialize(buf) }
            GroupEntityDescription::EnhancedFixedWingAircraft(ged_record) => { ged_record.serialize(buf) }
            GroupEntityDescription::GroundLogisticsVehicle(ged_record) => { ged_record.serialize(buf) }
        }
    }
}

impl Serialize for GEDRecord1 {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        todo!()
    }
}

impl Serialize for GEDRecord2 {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        todo!()
    }
}

impl Serialize for GEDRecord3 {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        todo!()
    }
}

impl Serialize for GEDRecord4 {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        todo!()
    }
}

impl Serialize for GEDRecord5 {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        todo!()
    }
}

impl Serialize for GEDRecord6 {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        todo!()
    }
}

impl Serialize for GEDRecord7 {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        todo!()
    }
}

impl Serialize for GEDRecord8 {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        todo!()
    }
}

impl Serialize for GEDRecord9 {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        todo!()
    }
}