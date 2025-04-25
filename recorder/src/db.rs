use crate::model::MarkerType;
use crate::{DbId, FrameId, RecorderError, RecordingMetaData};
use bytes::Bytes;
use chrono::{DateTime, Utc};
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
