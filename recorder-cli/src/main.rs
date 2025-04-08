#[tokio::main]
async fn main() {
    let recorder = recorder::Recorder::new().await.unwrap();
    dbg!(recorder);
}
