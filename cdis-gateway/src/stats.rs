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
    bytes_received_interval: u64,
    packets_received_interval: u64,
    bytes_sent_interval: u64,
    packets_sent_interval: u64,
    pub bytes_received_latest_aggregate: u64,
    pub packets_received_latest_aggregate: u64,
    pub bytes_sent_latest_aggregate: u64,
    pub packets_sent_latest_aggregate: u64,
}

impl SocketStats {
    fn receive(&mut self, bytes: u64, packets: u64) {
        self.bytes_received += bytes;
        self.packets_received += packets;
        self.bytes_received_interval += bytes;
        self.packets_received_interval += packets;
    }

    fn sent(&mut self, bytes: u64, packets: u64) {
        self.bytes_sent += bytes;
        self.packets_sent += packets;
        self.bytes_sent_interval += bytes;
        self.packets_sent_interval += packets;
    }

    fn aggregate(&mut self) {
        self.bytes_received_latest_aggregate = self.bytes_received_interval;
        self.bytes_received_interval = 0;

        self.packets_received_latest_aggregate = self.packets_received_interval;
        self.packets_received_interval = 0;

        self.bytes_sent_latest_aggregate = self.bytes_sent_interval;
        self.bytes_sent_interval = 0;

        self.packets_sent_latest_aggregate = self.packets_sent_interval;
        self.packets_sent_interval = 0;
    }
}

#[derive(Clone, Debug, Default)]
pub(crate) struct CodecStats {
    pub received_count: HashMap<PduType, (u64, u64)>, // tuple containing the count per PduType, and total of bytes for that type.
    pub codec_count: HashMap<PduType, (u64, u64)>, // tuple containing the count per PduType, and total of bytes for that type.
    pub rejected_count: u64,
    pub unimplemented_count: u64,
    pub unimplemented_bytes: u64, // amount of received bytes not encoded due to the PduType being unimplemented.
    pub compression_rate_total: f64,
}

impl CodecStats {
    fn aggregate(&mut self) {
        const TO_PERCENT: f64 = 100.0;
        // FIXME: this calculates the ratio between ALL received bytes and the encoded/decoded output bytes, no taking into account the rejected, non-codec'ed PDUs.
        let encoded_bytes = self.codec_count.values().map(|stat| stat.1 ).sum::<u64>() as f64;
        let received_bytes = self.received_count.values().map(|stat| stat.1 ).sum::<u64>() as f64;
        if received_bytes.is_normal() {
            self.compression_rate_total = encoded_bytes / received_bytes * TO_PERCENT;
        }
    }
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
                self.dis.receive(count as u64, 1);
            }
            Event::ReceivedBytesCDis(count) => {
                self.cdis.receive(count as u64, 1);
            }
            Event::ReceivedDis(pdu_type, size) => {
                self.encoder.received_count.entry(pdu_type)
                    .and_modify(|count| {
                        count.0 += 1;
                        count.1 += size;
                    } )
                    .or_insert((1, size));
            }
            Event::ReceivedCDis(pdu_type, size) => {
                self.decoder.received_count.entry(pdu_type)
                    .and_modify(|count| {
                        count.0 += 1;
                        count.1 += size;
                    } )
                    .or_insert((1, size));
            }
            Event::EncodedPdu(pdu_type, size) => {
                self.encoder.codec_count.entry(pdu_type)
                    .and_modify(|count| {
                        count.0 += 1;
                        count.1 += size;
                    } )
                    .or_insert((1, size));
            }
            Event::DecodedPdu(pdu_type, size) => {
                self.decoder.codec_count.entry(pdu_type)
                    .and_modify(|count| {
                        count.0 += 1;
                        count.1 += size;
                    } )
                    .or_insert((1, size));
            }
            Event::RejectedUnsupportedDisPdu(_pdu_type, _size) => {
                self.encoder.rejected_count += 1;
            }
            Event::RejectedUnsupportedCDisPdu(_pdu_type, _size) => {
                self.decoder.rejected_count += 1;
            }
            Event::SentDis(count) => {
                self.dis.sent(count as u64, 1);
            }
            Event::SentCDis(count) => {
                self.cdis.sent(count as u64, 1);
            }
            Event::UnimplementedEncodedPdu(_pdu_type, size) => {
                self.encoder.unimplemented_count += 1;
                self.encoder.unimplemented_bytes += size;
            }
            Event::UnimplementedDecodedPdu(_pdu_type, size) => {
                // fail; // TODO test stats of decoder
                // TODO - unimplemented encoded/decoded still get output to the other socket.
                self.decoder.unimplemented_count += 1;
                self.decoder.unimplemented_bytes += size;
            }
        }
    }
}

pub async fn run_stats(
    stat_tx: tokio::sync::broadcast::Sender<SseStat>,
    mut cmd_rx: tokio::sync::broadcast::Receiver<Command>,
    mut event_rx: tokio::sync::mpsc::Receiver<Event>) {
    let mut stats = GatewayStats::default();
    let mut emit_timer = tokio::time::interval(Duration::from_millis(500));
    let mut aggregate_timer = tokio::time::interval(Duration::from_secs(1));

    loop {
        select! {
            _ = aggregate_timer.tick() => {
                stats.dis.aggregate();
                stats.cdis.aggregate();
                stats.encoder.aggregate();
                stats.decoder.aggregate();
            }
            _ = emit_timer.tick() => {
                // TODO cleanup, proper unwrap handling - trace!("Stats task stopping due to failure of stats to site channel.");
                stat_tx.send(SseStat::DisSocket(stats.dis.clone())).unwrap();
                stat_tx.send(SseStat::CdisSocket(stats.cdis.clone())).unwrap();
                stat_tx.send(SseStat::Encoder(stats.encoder.clone())).unwrap();
                stat_tx.send(SseStat::Decoder(stats.decoder.clone())).unwrap();
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
