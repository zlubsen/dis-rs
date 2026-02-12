mod enumerations;
mod pdus;

fn main() {
    let uid_index = enumerations::execute();

    #[cfg(feature = "v8")]
    pdus::execute(&uid_index);
}
