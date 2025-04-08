use chrono::{DateTime, Utc};
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::SqlitePool;
use std::path::{Path, PathBuf};
use thiserror::Error;

pub type FrameId = usize;

#[derive(Default, Debug)]
pub enum State {
    #[default]
    Uninitialised,
    Initialised,
    Recording,
    Playing,
    Paused,
    Finished,
}

#[derive(Debug)]
pub enum Operation {
    CreateRecording(Box<Path>),
    LoadRecording(Box<Path>),
    AddStream,
    RemoveStream,
    Record,
    Play,
    Pause,
    Rewind,
    Seek(FrameId),
}

#[derive(Debug, Error)]
pub enum RecorderError {
    #[error("Failed to create database '{0}'.")]
    DatabaseCannotBeCreated(String),
    #[error("The database '{0}' does not exist.")]
    DatabaseDoesNotExist(String),
    #[error("Database error: {0}.")]
    DatabaseError(sqlx::Error),
}

#[derive(Default, Debug)]
pub struct Position {
    pub number_of_frames: FrameId,
    pub current_frame: FrameId,
}

#[derive(Debug)]
pub struct RecordingMetaData {
    pub schema_version: String,
    pub name: String,
    pub date_created: DateTime<Utc>,
}

#[derive(Debug)]
pub struct Recorder {
    pub filename: String,
    pub pool: SqlitePool,
    pub meta: RecordingMetaData,
    pub state: State,
    pub position: Position,
}

impl Recorder {
    pub async fn new() -> Result<Self, RecorderError> {
        let file_path = create_db_filename();
        Self::new_with_file(&file_path).await
    }

    pub async fn new_with_file(file_path: impl AsRef<Path>) -> Result<Self, RecorderError> {
        let filename = file_path.as_ref().to_string_lossy().to_string();
        let mut pool = open_database(file_path).await?;
        let meta = read_metadata(&mut pool).await?;
        Ok(Self {
            filename,
            pool,
            meta,
            state: State::Uninitialised,
            position: Position::default(),
        })
    }
}

fn create_db_filename() -> PathBuf {
    let timestamp: DateTime<Utc> = Utc::now();
    let mut path = PathBuf::new();
    path.push(format!("{}.db", timestamp.format("%Y%m%d-%H%M%S")));

    path
}

async fn open_database(file_path: impl AsRef<Path>) -> Result<SqlitePool, RecorderError> {
    let options = SqliteConnectOptions::new()
        .filename(file_path)
        .create_if_missing(true);

    let pool = SqlitePool::connect_with(options)
        .await
        .map_err(|err| RecorderError::DatabaseError(err))?;

    Ok(pool)
}

async fn create_tables(pool: &SqlitePool) -> Result<(), RecorderError> {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .unwrap_or_else(|err| RecorderError::DatabaseError(err.into()));

    Ok(())
}

async fn read_metadata(pool: &mut SqlitePool) -> Result<RecordingMetaData, RecorderError> {
    sqlx::query!("SELECT * FROM meta_data;")
        .fetch_one(pool)
        .await?;
    todo!()
}
