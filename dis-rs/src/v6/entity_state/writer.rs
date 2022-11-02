use bytes::{BufMut, BytesMut};
use crate::common::Serialize;
use crate::v6::entity_state::model::EntityCapabilities;

impl Serialize for EntityCapabilities {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        let ammunition_supply = if self.ammunition_supply { 1u32 } else { 0u32 } << 31;
        let fuel_supply = if self.fuel_supply { 1u32 } else { 0u32 } << 30;
        let recovery = if self.recovery { 1u32 } else { 0u32 } << 29;
        let repair = if self.repair { 1u32 } else { 0u32 } << 28;
        let capabilities = ammunition_supply | fuel_supply | recovery | repair;
        buf.put_u32(capabilities);
        4
    }
}
