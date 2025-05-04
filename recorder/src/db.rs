use crate::model::MarkerType;
use crate::{DbId, FrameId, RecorderError, RecordingMetaData};
use bytes::{Bytes, BytesMut};
use chrono::{DateTime, Utc};
use dis_rs::entity_state::model::EntityState;
use dis_rs::model::PduHeader;
use dis_rs::{serialize_pdu_status, Serialize};
use gethostname::gethostname;
use sqlx::sqlite::{SqliteConnectOptions, SqliteRow};
use sqlx::{Row, SqlitePool};
use std::path::{Path, PathBuf};

/// Generates a filename for a new database, consisting of the date and time of creation.
/// Formatted as `YYYYMMDD-HHMMSS.db`
pub(crate) fn create_db_filename() -> PathBuf {
    let timestamp: DateTime<Utc> = Utc::now();
    let mut path = PathBuf::new();
    path.push(format!("{}.db", timestamp.format("%Y%m%d-%H%M%S")));

    path
}

/// Opens a connection to a database at the specified path, returning a connection pool.
/// When a file at the specified path does not exist it will be created, along with all the tables.
pub(crate) async fn open_database(
    file_path: impl AsRef<Path>,
) -> Result<SqlitePool, RecorderError> {
    let new_db = !file_path.as_ref().exists();
    let options = SqliteConnectOptions::new()
        .filename(file_path)
        .create_if_missing(true);

    let pool = SqlitePool::connect_with(options)
        .await
        .map_err(|err| RecorderError::DatabaseError(err.to_string()))?;

    if new_db {
        run_migrations(&pool).await?;
        insert_initial_metadata(&pool).await?;
    }

    Ok(pool)
}

/// Insert the initial metadata into a database.
async fn insert_initial_metadata(pool: &SqlitePool) -> Result<(), RecorderError> {
    let timestamp = Utc::now().timestamp_millis();
    let _query_result = sqlx::query(
        "INSERT INTO metadata (hostname, time_created_utc_ms, schema_version) VALUES ($1, $2, $3);",
    )
    .bind(gethostname().to_string_lossy())
    .bind(timestamp)
    .bind(env!("CARGO_PKG_VERSION"))
    .execute(pool)
    .await
    .map_err(|err| RecorderError::DatabaseError(err.to_string()))?;
    Ok(())
}

/// Run the migrations for a database, creating all tables and relations in the schema.
pub(crate) async fn run_migrations(pool: &SqlitePool) -> Result<(), RecorderError> {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .map_err(|err| RecorderError::DatabaseError(err.to_string()))?;

    Ok(())
}

/// Fetch the metadata from the database, returned as a `RecordingMetaData` struct.
pub(crate) async fn query_metadata(pool: &SqlitePool) -> Result<RecordingMetaData, RecorderError> {
    let row = sqlx::query("SELECT * FROM metadata LIMIT 1")
        .map(|row: SqliteRow| {
            let _id: u64 = row.get("id");
            let hostname: &str = row.get("hostname");
            let time_started_utc: i64 = row.get("time_created_utc_ms");
            let frame_time_ms: i64 = row.get("frame_duration_ms");
            let schema_version: &str = row.get("schema_version");

            let date_created =
                chrono::DateTime::<Utc>::from_timestamp_millis(time_started_utc).unwrap();

            RecordingMetaData {
                schema_version: schema_version.to_string(),
                name: hostname.to_string(),
                date_created,
                frame_time_ms,
            }
        })
        .fetch_one(pool)
        .await
        .map_err(|err| RecorderError::DatabaseError(err.to_string()))?;

    Ok(row)
}

/// Fetch all `MarkerType`s from the database, returned as a `Vec<MarkerType>`.
pub(crate) async fn query_marker_types(
    pool: &SqlitePool,
) -> Result<Vec<MarkerType>, RecorderError> {
    let rows = sqlx::query("SELECT * FROM marker_types")
        .map(|row: SqliteRow| {
            let id: u64 = row.get("id");
            let name: &str = row.get("name");

            MarkerType {
                id,
                name: name.to_string(),
            }
        })
        .fetch_all(pool)
        .await
        .map_err(|err| RecorderError::DatabaseError(err.to_string()))?;

    Ok(rows)
}

pub(crate) async fn insert_stream(
    pool: &SqlitePool,
    name: &str,
    protocol: &str,
) -> Result<DbId, RecorderError> {
    let query_result = sqlx::query("INSERT INTO streams (name, protocol) VALUES ($1, $2);")
        .bind(name)
        .bind(protocol)
        .execute(pool)
        .await
        .map_err(|err| RecorderError::DatabaseError(err.to_string()))?;
    Ok(query_result.last_insert_rowid() as DbId)
}

// TODO create frames for a given period; or create frames using triggers; only for frames that have actual packets?

pub(crate) async fn insert_packet(
    pool: &SqlitePool,
    stream_id: i64,
    frame_id: FrameId,
    time_received: i64,
    time_since_start_ms: i64,
    bytes: &Bytes,
) -> Result<DbId, RecorderError> {
    let query_result = sqlx::query(
        r"
            INSERT INTO packets (
                 stream_id,
                 frame_id,
                 time_received,
                 time_since_start_ms,
                 bytes)
            VALUES (
                $1,
                $2,
                $3,
                $4,
                $5,
            );
        ",
    )
    .bind(stream_id)
    .bind(frame_id as i64)
    .bind(time_received)
    .bind(time_since_start_ms)
    .bind(&bytes[..])
    .execute(pool)
    .await
    .map_err(|err| RecorderError::DatabaseError(err.to_string()))?;

    Ok(query_result.last_insert_rowid() as DbId)
}

pub(crate) async fn insert_pdu_entity_state(
    pool: &SqlitePool,
    stream_id: i64,
    frame_id: FrameId,
    time_received: i64,
    time_since_start_ms: i64,
    header: &PduHeader,
    body: &EntityState,
) -> Result<DbId, RecorderError> {
    let dr_params = {
        let mut dr_params = BytesMut::with_capacity(40);
        dr_params.resize(40, 0);
        body.dead_reckoning_parameters.serialize(&mut dr_params);
        dr_params.freeze()
    };
    let var_params = if body.variable_parameters.is_empty() {
        None
    } else {
        let mut var_params = BytesMut::with_capacity(16 * body.variable_parameters.len());
        var_params.resize(16 * body.variable_parameters.len(), 0);
        body.variable_parameters.iter().for_each(|param| {
            param.serialize(&mut var_params);
        });
        Some(var_params.to_vec())
    };

    let query_result = sqlx::query(
        r"
            START TRANSACTION;

            INSERT INTO pdu_header (
                protocol_version,
                exercise_id,
                pdu_type,
                protocol_family,
                time_stamp,
                pdu_length,
                pdu_status
            )
            VALUES (
                $1,
                $2,
                $3,
                $4,
                $5,
                $6,
                $7,
            )

            INSERT INTO pdu_entity_state (
                header_id,
                entity_id_site,
                entity_id_application,
                entity_id_entity,
                force_id,
                entity_type_kind,
                entity_type_domain,
                entity_type_country,
                entity_type_category,
                entity_type_subcategory,
                entity_type_specific,
                entity_type_extra,
                alt_entity_type_kind,
                alt_entity_type_domain,
                alt_entity_type_country,
                alt_entity_type_category,
                alt_entity_type_subcategory,
                alt_entity_type_specific,
                alt_entity_type_extra,
                entity_linear_velocity_first,
                entity_linear_velocity_second,
                entity_linear_velocity_third,
                entity_location_x,
                entity_location_y,
                entity_location_z,
                entity_orientation_psi,
                entity_orientation_theta,
                entity_orientation_phi,
                entity_appearance,
                dead_reckoning_parameters,
                entity_marking,
                entity_capabilities,
                variable_parameters
            )
            VALUES (
                SELECT last_insert_rowid(),
                $8,
                $9,
                $10,
                $11,
                $12,
                $13,
                $14,
                $15,
                $16,
                $17,
                $18,
                $19,
                $20,
                $21,
                $22,
                $23,
                $24,
                $25,
                $26,
                $27,
                $28,
                $29,
                $30,
                $31,
                $32,
                $33,
                $34,
                $35,
                $36,
                $37,
                $38,
                $39,
                $40,
            );

            COMMIT;
        ",
    )
    // bindings for header
    .bind(u8::from(header.protocol_version))
    .bind(u8::from(header.exercise_id))
    .bind(u8::from(header.pdu_type))
    .bind(u8::from(header.protocol_family))
    .bind(header.time_stamp)
    .bind(header.pdu_length)
    .bind(if let Some(status) = &header.pdu_status {
        serialize_pdu_status(status, &header.pdu_type)
    } else {
        0u8
    })
    // bindings for entity state body
    .bind(body.entity_id.simulation_address.site_id)
    .bind(body.entity_id.simulation_address.application_id)
    .bind(body.entity_id.entity_id)
    .bind(u8::from(body.force_id))
    .bind(u8::from(body.entity_type.kind))
    .bind(u8::from(body.entity_type.domain))
    .bind(u16::from(body.entity_type.country))
    .bind(u8::from(body.entity_type.category))
    .bind(u8::from(body.entity_type.subcategory))
    .bind(u8::from(body.entity_type.specific))
    .bind(u8::from(body.entity_type.extra))
    .bind(u8::from(body.alternative_entity_type.kind))
    .bind(u8::from(body.alternative_entity_type.domain))
    .bind(u16::from(body.alternative_entity_type.country))
    .bind(u8::from(body.alternative_entity_type.category))
    .bind(u8::from(body.alternative_entity_type.subcategory))
    .bind(u8::from(body.alternative_entity_type.specific))
    .bind(u8::from(body.alternative_entity_type.extra))
    .bind(body.entity_linear_velocity.first_vector_component)
    .bind(body.entity_linear_velocity.second_vector_component)
    .bind(body.entity_linear_velocity.third_vector_component)
    .bind(body.entity_location.x_coordinate)
    .bind(body.entity_location.y_coordinate)
    .bind(body.entity_location.z_coordinate)
    .bind(body.entity_orientation.psi)
    .bind(body.entity_orientation.theta)
    .bind(body.entity_orientation.phi)
    .bind(u32::from(&body.entity_appearance))
    .bind(dr_params.as_ref())
    .bind(&body.entity_marking.marking_string)
    .bind(u32::from(body.entity_capabilities))
    .bind(var_params)
    .execute(pool)
    .await
    .map_err(|err| RecorderError::DatabaseError(err.to_string()))?;

    Ok(query_result.last_insert_rowid() as DbId)
}
