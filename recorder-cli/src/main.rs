use inquire::{required, CustomUserError, Text};
use recorder::Operation;
use std::path::Path;

#[tokio::main]
async fn main() {
    let dev_db_path = Path::new("./recorder_dev.db");

    if dev_db_path.exists() {
        println!("Cleaned old dev_db");
        std::fs::remove_file(dev_db_path).unwrap()
    }

    let mut recorder = recorder::Recorder::new_with_file(dev_db_path)
        .await
        .unwrap();

    recorder
        .add_dis_stream(dis_data_specification())
        .await
        .expect("failed to add a stream to the recorder");

    let (cmd_tx, mut info_rx) = recorder.run().await;
    // let (recorder_handle, cmd_tx, mut info_rx) = recorder.run().await;

    let info_handle = tokio::spawn(async move {
        loop {
            if let Some(info) = info_rx.recv().await {
                eprintln!("\r{info:?}");
            }
        }
    });

    loop {
        let command = user_input();
        if process_command(&command, &cmd_tx).await {
            break;
        }
    }

    // let _ = recorder_handle.await.unwrap();
    info_handle.abort();
}

#[derive(Debug)]
enum CliCommand {
    Nop,
    Record,
    Play,
    Pause,
    Rewind,
    Seek(u64),
    Quit,
}

fn user_input() -> CliCommand {
    let input = Text::new(":> ")
        // .with_autocomplete(&suggester)
        .with_validator(required!())
        .prompt()
        .unwrap();
    let mut input = input.split_ascii_whitespace();

    let command = match input.next().unwrap() {
        "record" => CliCommand::Record,
        "play" => CliCommand::Play,
        "pause" => CliCommand::Pause,
        "rewind" => CliCommand::Rewind,
        "seek" => CliCommand::Seek(
            input
                .next()
                .expect("Requires extra argument.")
                .parse::<u64>()
                .expect("Failed to parse seek argument - must be an integer."),
        ),
        "quit" => CliCommand::Quit,
        _ => CliCommand::Nop,
    };
    command
}

fn suggester(val: &str) -> Result<Vec<String>, CustomUserError> {
    let suggestions = ["Record", "Play", "Pause", "Rewind", "Seek", "Quit"];

    let val_lower = val.to_lowercase();

    Ok(suggestions
        .iter()
        .filter(|s| s.to_lowercase().contains(&val_lower))
        .map(|s| String::from(*s))
        .collect())
}

async fn process_command(
    cmd: &CliCommand,
    recorder_cmd: &tokio::sync::mpsc::Sender<Operation>,
) -> bool {
    match cmd {
        CliCommand::Nop => false,
        CliCommand::Record => false,
        CliCommand::Play => false,
        CliCommand::Pause => false,
        CliCommand::Rewind => false,
        CliCommand::Seek(_) => false,
        CliCommand::Quit => {
            recorder_cmd.send(Operation::Quit).await.unwrap();
            println!("Shutdown");
            true
        }
    }
}

fn dis_data_specification() -> &'static str {
    r#"
        [[ nodes ]]
        type = "udp"
        name = "UDP socket"
        uri = "127.0.0.1:3000"
        interface = "127.0.0.1:3000"
        mode = "broadcast"
        ttl = 1
        block_own_socket = false

        [[ nodes ]]
        type = "dis_receiver"
        name = "DIS parser"
        exercise_id = 1
        allow_dis_versions = [6, 7]

        [[ nodes ]]
        type = "dis_sender"
        name = "DIS serialiser"
        exercise_id = 1
        allow_dis_versions = [6, 7]

        [[ channels ]]
        from = "UDP socket"
        to = "DIS parser"

        [[ channels ]]
        from = "DIS serialiser"
        to = "UDP socket"
    "#
}
