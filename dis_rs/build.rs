use std::{env, fs};
use std::path::Path;
use std::str::FromStr;

use quick_xml::Reader;
use quick_xml::events::Event;

#[derive(Debug)]
struct Enum {
    uid: usize,
    name: String,
    size: usize,
    items: Vec<EnumItem>,
}

#[derive(Debug)]
struct EnumItem {
    description: String,
    value: usize,
}

fn main() {
    let uids: Vec<usize> = vec![
        //3, 4, 5, // protocol version, pdu type, pdu family
        // 6, // Force Id
        7, // Entity Kind
        //19, 20, 21
        // 29, // Country
        // 44, // DR algorithms
        45, // entity marking char set
        // 56, 57, 58, 59, // attached and articulated parts
        // 60, 61 // Munition Descriptor-Warhead, Fuse
        // 62, // Detonation result
    ];

    let out_dir = env::var_os("OUT_DIR").unwrap();
    // let enums_input_file = env::var("SISO_REF_010_FILE").unwrap();
    // println!("{}", enums_input_file);
    // read the xml file from an env var
    // let enums_file_path = format!("./enumerations/{}", enums_input_file.as_str());
    // println!("{}", enums_file_path);
    // let xml = std::fs::read_to_string(Path::new(enums_file_path.as_str())).unwrap();
    // let mut reader = Reader::from_str(&xml);
    // let mut reader = Reader::from_file(Path::new(enums_file_path.as_str())).unwrap();
    let mut reader = Reader::from_file(Path::new("./enumerations/SISO-REF-010.xml")).unwrap();
    reader.trim_text(true);

    let mut buf = Vec::new();

    let mut current_enum = Enum{
        uid: 0,
        name: "".to_string(),
        size: 0,
        items: vec![]
    };
    let mut current_item = EnumItem {
        description: "".to_string(),
        value: 0
    };

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"enum" => {
                        // extract 'uid' 'name' 'size' attributes; create a struct for this with a vec for enumrow elements
                        let uid = if let Ok(Some(attr_uid)) = e.try_get_attribute(b"uid") {
                            Some(usize::from_str(reader.decoder().decode(&*attr_uid.value).unwrap()).unwrap())
                        } else { None };
                        if !uids.contains(&uid.unwrap()) {
                            continue;
                        }

                        let name = if let Ok(Some(attr_name)) = e.try_get_attribute(b"name") {
                            Some(String::from_utf8(attr_name.value.to_vec()).unwrap())
                        } else { None };

                        let size = if let Ok(Some(attr_size)) = e.try_get_attribute(b"size") {
                            Some(usize::from_str(reader.decoder().decode(&*attr_size.value).unwrap()).unwrap())
                        } else { None };

                        if let (Some(uid), Some(name), Some(size)) = (uid, name, size) {
                            current_enum = Enum {
                                uid,
                                name,
                                size,
                                items: vec![]
                            }
                        };
                        println!("{:?}",current_enum);
                    },
                    b"enumrow" => {
                        // extract 'value' and 'description' attributes into an enumrow struct, add to parent enum struct;
                    },
                    _ => (),
                }
            }
            Ok(Event::Text(_e)) => (),//txt.push(e.unescape_and_decode(&reader).unwrap()),
            Ok(Event::Eof) => break, // exits the loop when reaching end of file
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (), // There are several other `Event`s we do not consider here
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

