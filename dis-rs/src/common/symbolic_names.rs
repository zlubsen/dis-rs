// Defines Symbolic names from the standard (v7 - tables 25)

use crate::EntityId;

pub const ALL_AGGS: u32 = 0xFFFF;
pub const ALL_APPLIC: u32 = 0xFFFF;
pub const ALL_BEAMS: u16 = 0xFF;
pub const ALL_EMITTERS: u16 = 0xFF;
pub const ALL_ENTITIES: u32 = 0xFFFF;
pub const ALL_OBJECTS: u32 = 0xFFFF;
pub const ALL_SITES: u32 = 0xFFFF;
pub const EP_NO_SEQUENCE: u32 = 0xFFFF;
pub const IO_UNTIL_FURTHER_NOTICE: u32 = 65_535;
pub const MAX_PDU_SIZE_BITS: u32 = 65_536;
pub const MAX_PDU_SIZE_OCTETS: u32 = 8192;
pub const MULTIPLES_PRESENT: u32 = 0;
pub const NO_AGG: u16 = 0;
pub const NO_APPLIC: u16 = 0;
pub const NO_BEAM: u16 = 0;
pub const NO_CATEGORY: u16 = 0;
pub const NO_EMITTER: u16 = 0;
pub const NO_ENTITY: u16 = 0;
pub const NO_FIRE_MISSION: u16 = 0;
pub const NO_KIND: u16 = 0;
pub const NO_OBJECT: u16 = 0;
pub const NO_PATTERN: f32 = 0.0;
pub const NO_REF_NUMBER: u16 = 0;
pub const NO_SITE: u16 = 0;
pub const NO_SPECIFIC: u8 = 0;
pub const NO_SUBCAT: u8 = 0;
pub const NO_VALUE: u16 = 0;
pub const POWER_ENGINE_OFF: f32 = -100.0;
pub const POWER_IDLE: f32 = 0.0;
pub const POWER_MAX_AFTERBURNER: f32 = 100.0;
pub const POWER_MILITARY: f32 = 50.0;
pub const POWER_MIN_AFTERBURNER: f32 = 51.0;
pub const RQST_ASSIGN_ID: u32 = 0xFFFE;

#[allow(non_snake_case)]
pub struct Variables {
    pub AGG_RESPONSE_DFLT: f32, //Default: 10 s
    pub COLLISION_ELASTIC_TIMEOUT: f32, //Default: 5 s
    pub COLLISION_THRSH: f32, //Default: 0.1 m/s
    pub DE_AREA_AIMING_THRSH: f32, //Default: 10°
    pub DE_ENERGY_THRSH: f32, //Default: 1.0%
    pub DE_PRECISION_AIMING_THRSH: f32, //Default: 0.5 m
    pub DRA_ORIENT_THRSH: f32, //Default: 3°
    pub DRA_POS_THRSH: f32, //Default: 1 m
    pub EE_AD_PULRAT_THRSH: f32, //Default: 0.017 rad/s
    pub EE_AD_PULACC_THRSH: f32, //Default: 0.017 rad/s2
    pub EE_AZ_THRSH: f32, //Default: 1°
    pub EE_EL_THRSH: f32, //Default: 1°
    pub EE_ERP_THRSH: f32, //Default: 1.0 dBm
    pub EE_FREQ_THRSH: f32, //Default: 1 Hz
    pub EE_FRNG_THRSH: f32, //Default: 1 Hz
    pub EE_FT_VEL_THRSH: f32, //Default: 1.0 m/s
    pub EE_FT_ACC_THRSH: f32, //Default: 1.0 m/s2
    pub EE_FT_MWD_THRSH: f32, //Default: 10000 m
    pub EE_FT_KT_THRSH: f32, //Default: 10 s
    pub EE_FT_ESP_THRSH: f32, //Default: 10 m
    pub EE_HIGH_DENSITY_THRSH: f32, //Default: 10 entities/beam
    pub EE_PRF_THRSH: f32, //Default: 1 Hz
    pub EE_PW_THRSH: f32, //Default: 1 μs
    pub EP_DIMENSION_THRSH: f32, //Default: 1 m
    pub EP_STATE_THRSH: f32, //User defined Default: ±10%
    pub GD_GEOMETRY_CHANGE: f32, //User defined Default: ±10%
    pub GD_STATE_CHANGE: f32, //User defined Default: ±10%
    pub HBT_DAMAGE_TIMEOUT_MPLIER: f32, //Default: 2.4 (see NOTE 3)
    pub HBT_ESPDU_KIND_CULTURAL_FEATURE: f32, //Default: 5 s Tolerance: ±10%
    pub HBT_ESPDU_KIND_ENVIRONMENTAL: f32, //Default: 5 s Tolerance: ±10%
    pub HBT_ESPDU_KIND_EXPENDABLE: f32, //Default: 5 s Tolerance: ±10%
    pub HBT_ESPDU_KIND_LIFE_FORM: f32, //Default: 5 s Tolerance: ±10%
    pub HBT_ESPDU_KIND_MUNITION: f32, //Default: 5 s Tolerance: ±10%
    pub HBT_ESPDU_KIND_RADIO: f32, //Default: 5 s Tolerance: ±10%
    pub HBT_ESPDU_KIND_SENSOR: f32, //Default: 5 s Tolerance: ±10%
    pub HBT_ESPDU_KIND_EMITTER: f32, //Default: 5 s Tolerance: ±10%
    pub HBT_ESPDU_KIND_SUPPLY: f32, //Default: 5 s Tolerance: ±10%
    pub HBT_ESPDU_PLATFORM_AIR: f32, //Default: 5 s Tolerance: ±10%
    pub HBT_ESPDU_PLATFORM_LAND: f32, //Default: 5 s Tolerance: ±10%
    pub HBT_ESPDU_PLATFORM_SPACE: f32, //Default: 5 s Tolerance: ±10%
    pub HBT_ESPDU_PLATFORM_SUBSURFACE: f32, //Default: 5 s Tolerance: ±10%
    pub HBT_ESPDU_PLATFORM_SURFACE: f32, //Default: 5 s Tolerance: ±10%
    pub HBT_PDU_AGGREGATE_STATE: f32, //Default: 30 s Tolerance: ±10%
    pub HBT_PDU_APPEARANCE: f32, //Default: 60 s Tolerance: ±10%
    pub HBT_PDU_DE_FIRE: f32, //Default: 0.5s Tolerance: ±10%
    pub HBT_PDU_DESIGNATOR: f32, //Default: 5 s Tolerance: ±10%
    pub HBT_PDU_EE: f32, //Default: 5 s Tolerance: ±10%
    pub HBT_PDU_ENTITY_DAMAGE: f32, //Default: 10 s Tolerance: ±10%
    pub HBT_PDU_ENVIRONMENTAL_PROCESS: f32, //Default: 15 s Tolerance: ±10%
    pub HBT_PDU_GRIDDED_DATA: f32, //Default: 15 min Tolerance: ±10%
    pub HBT_PDU_IFF: f32, //Default: 10 s Tolerance: ±10%
    pub HBT_PDU_ISGROUPOF: f32, //Default: 5 s Tolerance: ±10%
    pub HBT_PDU_MINEFIELD_DATA: f32, //Default: 5 s Tolerance: ±10%
    pub HBT_PDU_MINEFIELD_STATE: f32, //Default: 5 s Tolerance: ±10%
    pub HBT_PDU_RECEIVER: f32, //Default: 5 s Tolerance: ±10%
    pub HBT_PDU_SEES: f32, //Default: 3 min Tolerance: ±10%
    pub HBT_PDU_TRANSMITTER: f32, //Default: 2 s Tolerance: ±10%
    pub HBT_PDU_TSPI: f32, //Default: 30 s Tolerance: ±10%
    pub HBT_PDU_UA: f32, //Default: 3 min Tolerance: ±10%
    pub HBT_STATIONARY: f32, //Default: 1 min Tolerance: ±10%
    pub HBT_TIMEOUT_MPLIER: f32, //Default: 2.4 (see NOTE 2)
    pub HQ_TOD_DIFF_THRSH: f32, //Default: 20 ms
    pub IFF_AZ_THRSH: f32, //Default: 3°
    pub IFF_CHG_LATENCY: f32, //Default: 2 s
    pub IFF_EL_THRSH: f32, //Default: 3°
    pub IFF_IP_REPLY_TIMER: f32, //Default: 30 s
    pub IFF_PDU_FINAL: f32, //Default: 10 s
    pub IFF_PDU_RESUME: f32, //Default: 10 s
    pub MINEFIELD_CHANGE: f32, //Default: 2.5 s
    pub MINEFIELD_RESPONSE_TIMER: f32, //Default: 1 s
    pub NON_SYNC_THRSH: f32, //Default: 1 min
    pub REPAR_REC_T1: f32, //Default: 5 s
    pub REPAR_SUP_T1: f32, //Default: 12 s
    pub REPAR_SUP_T2: f32, //Default: 12 s
    pub RESUP_REC_T1: f32, //Default: 5 s
    pub RESUP_REC_T2: f32, //Default: 55 s
    pub RESUP_SUP_T1: f32, //Default: 1 min
    pub SEES_NDA_THRSH: f32, //Default: ±2° in the axis of deflection
    pub SEES_PS_THRSH: f32, //Default: ±10% of the maximum value of the Power Setting
    pub SEES_RPM_THRSH: f32, //Default: ±5% of the maximum engine speed in RPM
    pub SMALLEST_MTU_OCTETS: f32, //Default: 1400 octets for Internet Protocol Version 4 networks [NOTE 1]
    pub SM_REL_RETRY_CNT: f32, //Default: 3
    pub SM_REL_RETRY_DELAY: f32, //Default: 2 s
    pub TIMESTAMP_AHEAD: f32, //Default: 5 s
    pub TIMESTAMP_BEHIND: f32, //Default: 5 s
    pub TI_TIMER1_DFLT: f32, //Default: 2 s
    pub TI_TIMER2_DFLT: f32, //Default: 12 s
    pub TO_AUTO_RESPONSE_TIMER: f32, //Default: 5 s
    pub TO_MAN_RESPONSE_TIMER: f32, //Default: 120 s
    pub TR_TIMER1_DFLT: f32, //Default: 5 s
    pub TR_TIMER2_DFLT: f32, //Default: 60 s
    pub TRANS_ORIENT_THRSH: f32, //Default: 180°
    pub TRANS_POS_THRSH: f32, //Default: 500 m
    pub UA_ORIENT_THRSH: f32, //Default: 2°
    pub UA_POS_THRSH: f32, //Default: 10 m
    pub UA_SRPM_ROC_THRSH: f32, //Default: ±10% of maximum rate of change
    pub UA_SRPM_THRSH: f32, //Default: ±5% of maximum shaft rate in RPM
    pub D_SPOT_NO_ENTITY: EntityId,
    pub ENTITY_ID_UNKNOWN: EntityId,
    pub NO_SPECIFIC_ENTITY: EntityId,
    pub TARGET_ID_UNKNOWN: EntityId,
}

impl Variables {
    pub fn new() -> Self {
        Self {
            AGG_RESPONSE_DFLT: 10.0, // s
            COLLISION_ELASTIC_TIMEOUT: 5.0, // s
            COLLISION_THRSH: 0.1, // m/s
            DE_AREA_AIMING_THRSH: 10.0, // °
            DE_ENERGY_THRSH: 1.0, // %
            DE_PRECISION_AIMING_THRSH: 0.5, // m
            DRA_ORIENT_THRSH: 3.0, // °
            DRA_POS_THRSH: 1.0, // m
            EE_AD_PULRAT_THRSH: 0.017, // rad/s
            EE_AD_PULACC_THRSH: 0.017, // rad/s2
            EE_AZ_THRSH: 1.0, // °
            EE_EL_THRSH: 1.0, // °
            EE_ERP_THRSH: 1.0, // dBm
            EE_FREQ_THRSH: 1.0, // Hz
            EE_FRNG_THRSH: 1.0, // Hz
            EE_FT_VEL_THRSH: 1.0, // m/s
            EE_FT_ACC_THRSH: 1.0, // m/s2
            EE_FT_MWD_THRSH: 10000.0, // m
            EE_FT_KT_THRSH: 10.0, // s
            EE_FT_ESP_THRSH: 10.0, // m
            EE_HIGH_DENSITY_THRSH: 10.0, // entities/beam
            EE_PRF_THRSH: 1.0, // Hz
            EE_PW_THRSH: 1.0, // μs
            EP_DIMENSION_THRSH: 1.0, // m
            EP_STATE_THRSH: 0.0, //User defined Default: ±10%
            GD_GEOMETRY_CHANGE: 0.0, //User defined Default: ±10%
            GD_STATE_CHANGE: 0.0, //User defined Default: ±10%
            HBT_DAMAGE_TIMEOUT_MPLIER: 2.4, // (see NOTE 3)
            HBT_ESPDU_KIND_CULTURAL_FEATURE: 5.0, // s Tolerance: ±10%
            HBT_ESPDU_KIND_ENVIRONMENTAL: 5.0, // s Tolerance: ±10%
            HBT_ESPDU_KIND_EXPENDABLE: 5.0, // s Tolerance: ±10%
            HBT_ESPDU_KIND_LIFE_FORM: 5.0, // s Tolerance: ±10%
            HBT_ESPDU_KIND_MUNITION: 5.0, // s Tolerance: ±10%
            HBT_ESPDU_KIND_RADIO: 5.0, // s Tolerance: ±10%
            HBT_ESPDU_KIND_SENSOR: 5.0, // s Tolerance: ±10%
            HBT_ESPDU_KIND_EMITTER: 5.0, // s Tolerance: ±10%
            HBT_ESPDU_KIND_SUPPLY: 5.0, // s Tolerance: ±10%
            HBT_ESPDU_PLATFORM_AIR: 5.0, // s Tolerance: ±10%
            HBT_ESPDU_PLATFORM_LAND: 5.0, // s Tolerance: ±10%
            HBT_ESPDU_PLATFORM_SPACE: 5.0, // s Tolerance: ±10%
            HBT_ESPDU_PLATFORM_SUBSURFACE: 5.0, // s Tolerance: ±10%
            HBT_ESPDU_PLATFORM_SURFACE: 5.0, // s Tolerance: ±10%
            HBT_PDU_AGGREGATE_STATE: 30.0, // s Tolerance: ±10%
            HBT_PDU_APPEARANCE: 60.0, // s Tolerance: ±10%
            HBT_PDU_DE_FIRE: 0.5, // s Tolerance: ±10%
            HBT_PDU_DESIGNATOR: 5.0, // s Tolerance: ±10%
            HBT_PDU_EE: 5.0, // s Tolerance: ±10%
            HBT_PDU_ENTITY_DAMAGE: 10.0, // s Tolerance: ±10%
            HBT_PDU_ENVIRONMENTAL_PROCESS: 15.0, // s Tolerance: ±10%
            HBT_PDU_GRIDDED_DATA: 15.0, // min Tolerance: ±10%
            HBT_PDU_IFF: 10.0, // s Tolerance: ±10%
            HBT_PDU_ISGROUPOF: 5.0, // s Tolerance: ±10%
            HBT_PDU_MINEFIELD_DATA: 5.0, // s Tolerance: ±10%
            HBT_PDU_MINEFIELD_STATE: 5.0, // s Tolerance: ±10%
            HBT_PDU_RECEIVER: 5.0, // s Tolerance: ±10%
            HBT_PDU_SEES: 3.0, // min Tolerance: ±10%
            HBT_PDU_TRANSMITTER: 2.0, // s Tolerance: ±10%
            HBT_PDU_TSPI: 30.0, // s Tolerance: ±10%
            HBT_PDU_UA: 3.0, // min Tolerance: ±10%
            HBT_STATIONARY: 1.0, // min Tolerance: ±10%
            HBT_TIMEOUT_MPLIER: 2.4, // (see NOTE 2)
            HQ_TOD_DIFF_THRSH: 20.0, // ms
            IFF_AZ_THRSH: 3.0, // °
            IFF_CHG_LATENCY: 2.0, // s
            IFF_EL_THRSH: 3.0, // °
            IFF_IP_REPLY_TIMER: 30.0, // s
            IFF_PDU_FINAL: 10.0, // s
            IFF_PDU_RESUME: 10.0, // s
            MINEFIELD_CHANGE: 2.5, // s
            MINEFIELD_RESPONSE_TIMER: 1.0, // s
            NON_SYNC_THRSH: 1.0, // min
            REPAR_REC_T1: 5.0, // s
            REPAR_SUP_T1: 12.0, // s
            REPAR_SUP_T2: 12.0, // s
            RESUP_REC_T1: 5.0, // s
            RESUP_REC_T2: 55.0, // s
            RESUP_SUP_T1: 1.0, // min
            SEES_NDA_THRSH: 2.0, // ° in the axis of deflection
            SEES_PS_THRSH: 10.0, // ±% of the maximum value of the Power Setting
            SEES_RPM_THRSH: 5.0, // ±% of the maximum engine speed in RPM
            SMALLEST_MTU_OCTETS: 1400.0, // octets for Internet Protocol Version 4 networks [NOTE 1]
            SM_REL_RETRY_CNT: 3.0, //
            SM_REL_RETRY_DELAY: 2.0, // s
            TIMESTAMP_AHEAD: 5.0, // s
            TIMESTAMP_BEHIND: 5.0, // s
            TI_TIMER1_DFLT: 2.0, // s
            TI_TIMER2_DFLT: 12.0, // s
            TO_AUTO_RESPONSE_TIMER: 5.0, // s
            TO_MAN_RESPONSE_TIMER: 120.0, // s
            TR_TIMER1_DFLT: 5.0, // s
            TR_TIMER2_DFLT: 60.0, // s
            TRANS_ORIENT_THRSH: 180.0, // °
            TRANS_POS_THRSH: 500.0, // m
            UA_ORIENT_THRSH: 2.0, // °
            UA_POS_THRSH: 10.0, // m
            UA_SRPM_ROC_THRSH: 10.0, // ±% of maximum rate of change
            UA_SRPM_THRSH: 5.0, // ±% of maximum shaft rate in RPM;
            D_SPOT_NO_ENTITY: EntityId::new(NO_SITE,  NO_APPLIC, NO_ENTITY),
            ENTITY_ID_UNKNOWN: EntityId::new(NO_SITE,  NO_APPLIC, NO_ENTITY),
            NO_SPECIFIC_ENTITY: EntityId::new(NO_SITE,  NO_APPLIC, NO_ENTITY),
            TARGET_ID_UNKNOWN: EntityId::new(NO_SITE,  NO_APPLIC, NO_ENTITY),
        }
    }
}

