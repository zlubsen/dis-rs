const SISO_REF_DIR: &str = "./definitions/enumerations/";
const SISO_SCHEMA_DIR: &str = "./definitions/v8-schemas";

fn main() {
    const SISO_REF_010_DEV_DEP_SRC: &str = "../dis-gen-siso-ref-010/src";
    const IEEE_1278_GEN_3_DEV_DEP_SRC: &str = "../dis-gen-ieee-1278-gen-3/src";

    set_rerun_if_changed_for_path(SISO_REF_010_DEV_DEP_SRC);
    set_rerun_if_changed_for_path(IEEE_1278_GEN_3_DEV_DEP_SRC);

    let (uid_index, pdu_types_index, er_types_index) = dis_gen_siso_ref_010::execute(SISO_REF_DIR);

    dis_gen_ieee_1278_gen_3::execute(SISO_SCHEMA_DIR, uid_index, pdu_types_index, er_types_index);

    // println!("cargo::rerun-if-changed=build.rs");
    // println!("cargo::rerun-if-changed=./definitions/enumerations/overrides.toml");
    // println!("cargo::rerun-if-changed=./definitions/enumerations/SISO-REF-010.xml");
}

fn set_rerun_if_changed_for_path(path: &str) {
    // Read the directory and emit a flag for every file
    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.flatten() {
            if entry.path().is_file() {
                println!("cargo:rerun-if-changed={}", entry.path().display());
            }
        }
    }
}
