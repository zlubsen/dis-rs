use std::collections::HashMap;
use std::time::Duration;
use tokio::select;
use tracing::log::trace;
use dis_rs::enumerations::PduType;
use crate::{Command, Event};

#[derive(Clone, Debug)]
pub(crate) enum SseStat {
    DisSocket(SocketStats),
    CdisSocket(SocketStats),
    Encoder(CodecStats),
    Decoder(CodecStats),
}

#[derive(Clone, Debug, Default)]
pub(crate) struct SocketStats {
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub packets_sent: u64,
    pub packets_received: u64,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct CodecStats {
    pub received_count: HashMap<PduType, i64>,
    pub codec_count: HashMap<PduType, i64>,
    pub rejected_count: u64,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct GatewayStats {
    pub dis: SocketStats,
    pub cdis: SocketStats,
    pub encoder: CodecStats,
    pub decoder: CodecStats,
}

impl GatewayStats {
    fn handle_event(&mut self, event: Event) {
        match event {
            Event::ReceivedBytesDis(count) => {
                self.dis.bytes_received += count as u64;
                self.dis.packets_received += 1;
            }
            Event::ReceivedBytesCDis(count) => {
                self.cdis.bytes_received += count as u64;
                self.cdis.packets_received += 1;
            }
            Event::ReceivedDis(pdu_type) => {
                self.encoder.received_count.entry(pdu_type)
                    .and_modify(|mut count| *count += 1 )
                    .or_insert(1);
            }
            Event::ReceivedCDis(pdu_type) => {
                self.decoder.received_count.entry(pdu_type)
                    .and_modify(|mut count| *count += 1 )
                    .or_insert(1);
            }
            Event::EncodedPdu(pdu_type) => {
                self.encoder.codec_count.entry(pdu_type)
                    .and_modify(|mut count| *count += 1 )
                    .or_insert(1);
            }
            Event::DecodedPdu(pdu_type) => {
                self.decoder.codec_count.entry(pdu_type)
                    .and_modify(|mut count| *count += 1 )
                    .or_insert(1);
            }
            Event::RejectedUnsupportedDisPdu(pdu_type) => {
                self.encoder.rejected_count += 1;
            }
            Event::RejectedUnsupportedCDisPdu(pdu_type) => {
                self.decoder.rejected_count += 1;
            }
            Event::SentDis(count) => {
                self.dis.bytes_sent += count as u64;
                self.dis.packets_sent += 1;
            }
            Event::SentCDis(count) => {
                self.cdis.bytes_sent += count as u64;
                self.cdis.packets_sent += 1;
            }
        }
    }
}

pub async fn run_stats(
    stat_tx: tokio::sync::broadcast::Sender<SseStat>,
    mut cmd_rx: tokio::sync::broadcast::Receiver<Command>,
    mut event_rx: tokio::sync::mpsc::Receiver<Event>) {
    let mut stats = GatewayStats::default();
    let mut timer = tokio::time::interval(Duration::from_millis(500));

    loop {
        select! {
            _ = timer.tick() => {
                // TODO cleanup, proper unwrap handling
                stat_tx.send(SseStat::DisSocket(stats.dis.clone())).unwrap();
                stat_tx.send(SseStat::CdisSocket(stats.cdis.clone())).unwrap();
                stat_tx.send(SseStat::Encoder(stats.encoder.clone())).unwrap();
                stat_tx.send(SseStat::Decoder(stats.decoder.clone())).unwrap();
                // if let Err(err) = stat_tx.send(stats.clone()).await {
                //     trace!("Stats task stopping due to failure of stats to site channel.");
                //     return;
                // }
            }
            event = event_rx.recv() => {
                if let Some(event) = event {
                    stats.handle_event(event);
                }
            }
            cmd = cmd_rx.recv() => {
                let cmd = cmd.unwrap_or_default();
                match cmd {
                    Command::NoOp => { }
                    Command::Quit => {
                        trace!("Stats task stopping due to receiving Command::Quit.");
                        return;
                    }
                }
            }
        }
    }
}
