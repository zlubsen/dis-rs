mod enumerations;
mod pdus;

fn main() {
    enumerations::generate();

    #[cfg(feature = "v8")]
    pdus::generate();
}
