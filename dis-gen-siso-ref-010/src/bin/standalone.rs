/// Expects `OUT_DIR` environment variable to be set.
/// Usually this is done by Cargo when executing a build script,
/// but for standalone testing the user must take care of setting the value manually.
fn main() {
    let file_path = std::env::args()
        .nth(1)
        .expect("Expected argument of the path to the SISO-REF-010.xml file");
    let (uid_lookup, _pdu_types, _er_types) = dis_gen_siso_ref_010::execute(&file_path);

    println!("Nr of items: {}", uid_lookup.len());
}
