const SISO_REF_DIR: &str = "./definitions/enumerations/";

fn main() {
    let _uid_index = dis_gen_siso_ref_010::execute(SISO_REF_DIR);

    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=./definitions/enumerations/overrides.toml");
    println!("cargo::rerun-if-changed=./definitions/enumerations/SISO-REF-010.xml");
}
