use chrono::{DateTime, Utc};
use dis_rs::model::Pdu;
use futures_util::future::JoinAll;
use gateway_core::error::GatewayError;
use gateway_core::runtime::{preset_builder_from_spec_str, run_from_builder, Command, Event};
use sqlx::SqlitePool;
use std::path::Path;
use thiserror::Error;
use tokio::task::JoinHandle;

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
    #[error("Gateway error: {0}.")]
    GatewayError(GatewayError),
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
    pub recording: Option<Recording>,
    pub streams: Vec<DisStream>,
    pub state: State,
}

#[derive(Debug)]
pub struct Recording {
    pub filename: String,
    pub pool: SqlitePool,
    pub meta: RecordingMetaData,
    pub position: Position,
}

#[derive(Debug)]
pub struct DisStream {
    pub input_tx: tokio::sync::broadcast::Sender<Pdu>,
    pub output_rx: tokio::sync::broadcast::Receiver<Pdu>,
    pub cmd_tx: tokio::sync::broadcast::Sender<Command>,
    pub event_rx: tokio::sync::broadcast::Receiver<Event>,
    pub join_handle: JoinAll<JoinHandle<()>>,
}

impl Recorder {
    pub async fn new() -> Result<Self, RecorderError> {
        Ok(Self {
            recording: None,
            streams: vec![],
            state: State::Uninitialised,
        })
    }

    pub async fn new_with_default_name() -> Result<Self, RecorderError> {
        let file_path = db::create_db_filename();
        Self::new_with_file(&file_path).await
    }

    pub async fn new_with_file(file_path: impl AsRef<Path>) -> Result<Self, RecorderError> {
        let filename = file_path.as_ref().to_string_lossy().to_string();

        let mut pool = db::open_database(file_path).await?;
        let meta = db::query_metadata(&mut pool).await?;
        Ok(Self {
            recording: Some(Recording {
                filename,
                pool,
                meta,
                position: Position::default(),
            }),
            streams: vec![],
            state: State::Initialised,
        })
    }

    pub async fn add_dis_stream(&mut self, gateway_spec: &str) -> Result<(), GatewayError> {
        let (infra_runtime_builder, cmd_tx, event_rx, input_tx, output_rx) =
            preset_builder_from_spec_str::<Pdu, Pdu>(gateway_spec)?;
        let join_handle = run_from_builder(infra_runtime_builder).await?;
        let stream = DisStream {
            input_tx: input_tx.expect("Expected an input channel for the gateway."),
            output_rx: output_rx.expect("Expected an output channel for the gateway."),
            cmd_tx,
            event_rx,
            join_handle,
        };
        self.streams.push(stream);
        Ok(())
    }

    pub fn shutdown(&mut self) {
        self.streams.iter().for_each(|stream| {
            let _ = stream.cmd_tx.send(Command::Quit).unwrap();
        });
        // if let Some(recording) = &self.recording {
        //     recording.pool.close().await;
        // }
    }
}
