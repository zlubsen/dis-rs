use crate::{DbId, FrameId};
use chrono::Utc;

pub struct MarkerType {
    pub id: DbId,
    pub name: String,
}

pub struct Marker {
    pub id: DbId,
    pub marker_type: MarkerType,
    pub label: String,
}

pub struct Stream {
    pub id: DbId,
    pub name: String,
    pub protocol: String,
}

pub struct Frame {
    pub id: DbId,
    pub time_from: u64,
    pub time_to: u64,
}

pub struct Packet {
    pub id: DbId,
    pub stream_id: Stream,
    pub frame_id: FrameId,
    pub time_received: chrono::DateTime<Utc>,
    pub time_since_start_ms: u64,
    pub bytes: Vec<u8>,
}
