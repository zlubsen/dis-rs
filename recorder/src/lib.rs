use chrono::{DateTime, Utc};
use sqlx::SqlitePool;
use std::path::Path;
use thiserror::Error;

pub(crate) mod db;
pub mod model;

pub type DbId = u64;
pub type FrameId = u64;

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
    pub frame_time_ms: i64,
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
        let file_path = db::create_db_filename();
        Self::new_with_file(&file_path).await
    }

    pub async fn new_with_file(file_path: impl AsRef<Path>) -> Result<Self, RecorderError> {
        let filename = file_path.as_ref().to_string_lossy().to_string();

        let mut pool = db::open_database(file_path).await?;
        let meta = db::query_metadata(&mut pool).await?;
        Ok(Self {
            filename,
            pool,
            meta,
            state: State::Uninitialised,
            position: Position::default(),
        })
    }
}
