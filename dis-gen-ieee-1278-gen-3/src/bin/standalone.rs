/// Expects `OUT_DIR` environment variable to be set.
/// Usually this is done by Cargo when executing a build script,
/// but for standalone testing the user must take care of setting the value manually.
fn main() {
    let siso_dir_path = std::env::args()
        .nth(1)
        .expect("Expected (1st) argument of the path to the SISO-REF-010 directory");
    let ieee_dir_path = std::env::args()
        .nth(2)
        .expect("Expected (2nd) argument of the path to the IEEE schema directory");
    let uid_lookup = dis_gen_siso_ref_010::execute(&siso_dir_path);
    dis_gen_ieee_1278_gen_3::execute(&ieee_dir_path, uid_lookup);
}
