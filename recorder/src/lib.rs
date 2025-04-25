use crate::play::PlayCommand;
use crate::rec::RecCommand;
use chrono::{DateTime, Utc};
use dis_rs::model::Pdu;
use futures_util::future::JoinAll;
use futures_util::StreamExt;
use gateway_core::error::GatewayError;
use gateway_core::runtime::{preset_builder_from_spec_str, run_from_builder, Command, Event};
use sqlx::SqlitePool;
use std::path::Path;
use std::time::Duration;
use thiserror::Error;
use tokio::task::JoinHandle;

pub(crate) mod db;
pub mod model;
mod play;
mod rec;

pub type DbId = u64;
pub type FrameId = u64;

#[derive(Default, Debug, Copy, Clone)]
pub enum State {
    #[default]
    Uninitialised,
    Ready,
    Recording,
    Playing,
    // Paused,
    Finished,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
    CreateRecording(Box<Path>),
    LoadRecording(Box<Path>),
    CloseRecording,
    AddStream,
    RemoveStream,
    Record,
    Play,
    Stop,
    Pause,
    Rewind,
    Seek(FrameId),
    Quit,
}

#[derive(Debug, Clone)]
pub enum RecorderEvent {
    Info(RecorderInfo),
    Error(RecorderError),
}

#[derive(Debug, Copy, Clone)]
pub struct RecorderInfo {
    state: State,
    position: Position,
}

#[derive(Debug, Error, Clone)]
pub enum RecorderError {
    #[error("Failed to create database '{0}'.")]
    DatabaseCannotBeCreated(String),
    #[error("The database '{0}' does not exist.")]
    DatabaseDoesNotExist(String),
    #[error("Database error: {0}.")]
    DatabaseError(String),
    #[error("Gateway error: {0}.")]
    GatewayError(String),
}

#[derive(Default, Debug, Copy, Clone)]
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
            state: State::Ready,
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

    pub fn info(&self) -> RecorderInfo {
        let position = if let Some(recording) = &self.recording {
            recording.position
        } else {
            Position::default()
        };
        RecorderInfo {
            state: self.state,
            position,
        }
    }

    pub async fn run(
        &mut self,
    ) -> (
        tokio::sync::mpsc::Sender<Operation>,
        tokio::sync::mpsc::Receiver<RecorderInfo>,
    ) {
        const INFO_OUTPUT_INTERVAL_MS: u64 = 250;
        const CHANNEL_CAPACITY: usize = 20;
        let (cmd_tx, mut cmd_rx) = tokio::sync::mpsc::channel::<Operation>(CHANNEL_CAPACITY);
        let (recorder_info_tx, recorder_info_rx) =
            tokio::sync::mpsc::channel::<RecorderInfo>(CHANNEL_CAPACITY);

        let (rec_cmd_tx, rec_cmd_rx) = tokio::sync::mpsc::channel::<RecCommand>(CHANNEL_CAPACITY);
        let (play_cmd_tx, play_cmd_rx) =
            tokio::sync::mpsc::channel::<PlayCommand>(CHANNEL_CAPACITY);
        let (int_event_tx, mut int_event_rx) =
            tokio::sync::mpsc::channel::<RecorderEvent>(CHANNEL_CAPACITY);

        let mut info_interval =
            tokio::time::interval(Duration::from_millis(INFO_OUTPUT_INTERVAL_MS));

        // let handle = tokio::spawn(async move {
        //     let mut incoming_futures = Vec::new();
        //     self.streams
        //         .iter_mut()
        //         .for_each(|mut stream| incoming_futures.push(stream.output_rx.recv()));
        //     let incoming_futures = FuturesUnordered::from_iter(incoming_futures);
        //     let incoming_futures = incoming_futures
        //         .collect::<Vec<Result<Pdu, tokio::sync::broadcast::error::RecvError>>>();

        loop {
            tokio::select! {
                Some(op) = cmd_rx.recv() => {
                    self.handle_operation(op.clone());
                    if op == Operation::Quit { break };
                }
                _ = info_interval.tick() => {
                    if let Err(err) = recorder_info_tx.send(self.info()).await {
                        eprintln!("{err}");
                    };
                }
                Some(event) = int_event_rx.recv() => {
                    match event {
                        RecorderEvent::Info(info) => {
                            println!("{info:?}");
                        }
                        RecorderEvent::Error(err) => {
                            eprintln!("{err:?}");
                        }
                    }
                }
            };
        }

        (cmd_tx, recorder_info_rx)
    }

    // fn futures_infra(&mut self) -> Collect<FuturesUnordered<impl Future<Output>>, _> {
    //     let mut incoming_futures = Vec::new();
    //     self.streams
    //         .iter_mut()
    //         .for_each(|mut stream| incoming_futures.push(stream.output_rx.recv()));
    //     let incoming_futures = FuturesUnordered::from_iter(incoming_futures);
    //     // let _ = incoming_futures
    //     //     .collect::<Vec<Result<Pdu, tokio::sync::broadcast::error::RecvError>>>()
    //     //     .await;
    //     // let a = incoming_futures
    //     //     .collect::<Vec<Result<Pdu, tokio::sync::broadcast::error::RecvError>>>()
    // }

    pub fn handle_operation(&mut self, op: Operation) {
        match op {
            Operation::CreateRecording(_) => {}
            Operation::LoadRecording(_) => {}
            Operation::CloseRecording => {}
            Operation::AddStream => {}
            Operation::RemoveStream => {}
            Operation::Record => {}
            Operation::Play => {}
            Operation::Stop => {}
            Operation::Pause => {}
            Operation::Rewind => {}
            Operation::Seek(_) => {}
            Operation::Quit => self.shutdown(),
        }
    }

    pub fn shutdown(&self) {
        self.streams.iter().for_each(|stream| {
            let _ = stream.cmd_tx.send(Command::Quit).unwrap();
        });
    }
}
