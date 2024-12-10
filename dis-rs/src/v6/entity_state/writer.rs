use crate::common::Serialize;
use crate::v6::entity_state::model::EntityCapabilities;
use bytes::{BufMut, BytesMut};

impl Serialize for EntityCapabilities {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        let ammunition_supply = u32::from(self.ammunition_supply) << 31;
        let fuel_supply = u32::from(self.fuel_supply) << 30;
        let recovery = u32::from(self.recovery) << 29;
        let repair = u32::from(self.repair) << 28;
        let capabilities = ammunition_supply | fuel_supply | recovery | repair;
        buf.put_u32(capabilities);
        4
    }
}
