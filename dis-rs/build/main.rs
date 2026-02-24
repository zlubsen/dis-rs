mod siso_1278_v8;
mod siso_ref_010;

fn main() {
    let uid_index = siso_ref_010::execute();

    #[cfg(feature = "v8")]
    siso_1278_v8::execute(&uid_index);
}
