use std::{env, fs};
use std::path::Path;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let enums_input_file = env::var("SISO_REF_010_FILE").unwrap();
    println!("{}", enums_input_file);
    // read the xml file from an env var
    let enums_file_path = format!("./enumerations/{}", enums_input_file.as_str());
    println!("{}", enums_file_path);
    let xml = std::fs::read_to_string(Path::new(enums_file_path.as_str())).unwrap();
    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);

    let mut buf = Vec::new();

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {

            }
            Ok(Event::Text(e)) => txt.push(e.unescape_and_decode(&reader).unwrap()),
            Ok(Event::Eof) => break, // exits the loop when reaching end of file
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            // _ => (), // There are several other `Event`s we do not consider here
        }
    }

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("enumerations.rs");

    fs::write(
        &dest_path,
        "pub fn message() -> &'static str {
            \"Hello, World!\"
        }

        pub enum Test {
            One,
            Two,
            Three,
        }
        "
    ).unwrap();

    // find all enumerations that we want to generate

    // generate all enums

    // save to file OUT_DIR/enumerations.rs

    // println!("cargo:rerun-if-changed={enums_file_path}");
}