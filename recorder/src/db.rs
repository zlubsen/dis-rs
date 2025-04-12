use crate::{RecorderError, RecordingMetaData};
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
        .map_err(|err| RecorderError::DatabaseError(err))?;

    if new_db {
        run_migrations(&pool).await?;
        insert_initial_metadata(&pool).await?;
    }

    Ok(pool)
}

/// Insert the initial metadata into a database.
async fn insert_initial_metadata(pool: &SqlitePool) -> Result<(), RecorderError> {
    let timestamp = Utc::now().timestamp();
    let _query_result = sqlx::query(
        "INSERT INTO metadata (hostname, time_started_utc, schema_version) VALUES ($1, $2, $3);",
    )
    .bind(gethostname().to_string_lossy())
    .bind(timestamp)
    .bind(env!("CARGO_PKG_VERSION"))
    .execute(pool)
    .await
    .map_err(|err| RecorderError::DatabaseError(err))?;
    Ok(())
}

/// Run the migrations for a database, creating all tables and relations in the schema.
pub(crate) async fn run_migrations(pool: &SqlitePool) -> Result<(), RecorderError> {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .map_err(|err| RecorderError::DatabaseError(err.into()))?;

    Ok(())
}

/// Fetch the metadata from the database, returned as a `RecordingMetaData` struct.
pub(crate) async fn query_metadata(pool: &SqlitePool) -> Result<RecordingMetaData, RecorderError> {
    let row = sqlx::query("SELECT * FROM metadata LIMIT 1")
        .map(|row: SqliteRow| {
            let id: u64 = row.get("id");
            let hostname: &str = row.get("hostname");
            let time_started_utc: i64 = row.get("time_started_utc");
            let frame_time_ms: i64 = row.get("frame_time_ms");
            let schema_version: &str = row.get("schema_version");

            let date_created =
                chrono::DateTime::<Utc>::from_timestamp(time_started_utc, 0).unwrap();

            RecordingMetaData {
                schema_version: schema_version.to_string(),
                name: hostname.to_string(),
                date_created,
                frame_time_ms,
            }
        })
        .fetch_one(pool)
        .await
        .map_err(|err| RecorderError::DatabaseError(err))?;

    Ok(row)
}
