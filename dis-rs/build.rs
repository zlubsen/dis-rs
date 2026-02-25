const SISO_REF_FILE: &str = "./definitions/enumerations/SISO-REF-010.xml";
const SISO_SCHEMA_DIR: &str = "./definitions/v8-schemas";

fn main() {
    let uid_index = dis_gen_siso_ref_010::execute(SISO_REF_FILE);

    #[cfg(feature = "v8")]
    dis_gen_ieee_1278_v8::execute(SISO_SCHEMA_DIR, &uid_index);
}
