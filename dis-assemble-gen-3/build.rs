const SISO_REF_DIR: &str = "./definitions/enumerations/";
const SISO_SCHEMA_DIR: &str = "./definitions/v8-schemas";

fn main() {
    let uid_index = dis_gen_siso_ref_010::execute(SISO_REF_DIR);

    dis_gen_ieee_1278_v8::execute(SISO_SCHEMA_DIR, uid_index);

    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=./definitions/enumerations/overrides.toml");
    println!("cargo::rerun-if-changed=./definitions/enumerations/SISO-REF-010.xml");
}
