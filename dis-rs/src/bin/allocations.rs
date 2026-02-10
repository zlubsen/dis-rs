use dis_rs::model::Pdu;
#[cfg(feature = "pcap-file")]
use pcap_file::pcap::PcapReader;
use std::fs::File;

const FILE_EXT_PCAP: &str = "pcap";
const FILE_EXT_XMSN: &str = "xmsn";

fn main() {
    #[cfg(feature = "hotpath")]
    let _guard = hotpath::init(
        "pdu_parser_allocations".to_string(),
        &[50, 95, 99],
        hotpath::Format::Table,
    );

    let file_name = std::env::args()
        .nth(1)
        .expect("Filename as first argument expected.");

    let bytes = match file_name.to_lowercase().split(".").last() {
        Some(FILE_EXT_PCAP) => {
            println!("Opening .{FILE_EXT_PCAP} file: {file_name}");
            read_pcap_file(&file_name)
        }
        Some(FILE_EXT_XMSN) => {
            println!("Opening .{FILE_EXT_XMSN} file: {file_name}");
            read_xmsn_file(&file_name)
        }
        Some(_) | None => {
            eprintln!("Unknown file format or no file specified: {}", file_name);
            eprintln!("Expecting '.{FILE_EXT_PCAP}' or '.{FILE_EXT_XMSN}' files.");
            return;
        }
    };
    println!("bytes len: {}", bytes.len());
    let pdus = parse_dis(&bytes);

    println!("Parsed {} PDUs.", pdus.len());

    #[cfg(feature = "hotpath")]
    drop(_guard);
}

fn read_pcap_file(file_name: &str) -> Vec<u8> {
    let file_in = File::open(file_name).expect("Error opening .{FILE_EXT_PCAP} file");
    let mut pcap_reader = PcapReader::new(file_in).unwrap();
    const NETWORK_STACK_HEADERS_LENGTH: usize = 42; // Ethernet/IP/UDP headers

    let mut bytes = Vec::new();
    while let Some(pkt) = pcap_reader.next_packet() {
        let pkt = pkt.expect("Error opening packet");
        let data = &pkt.data[NETWORK_STACK_HEADERS_LENGTH..];
        // let mut parsed = dis_rs::parse(data).expect("Expected a well-formed PDU");
        bytes.append(&mut data.to_vec());
    }
    bytes
}

fn read_xmsn_file(file_name: &str) -> Vec<u8> {
    std::fs::read(file_name).expect("Error opening and reading .{FILE_EXT_XMSN} file")
}

fn parse_dis(bytes: &Vec<u8>) -> Vec<Pdu> {
    dis_rs::parse(bytes.as_slice()).expect("Expected well formed PDUs.")
}
