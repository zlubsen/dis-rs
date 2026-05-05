# dis-rs

`dis-rs` is an implementation of the `Distributed Interactive Simulation` (`DIS`) protocol for Rust.
It provides structures and functions to build `PDU`s in applications, send them out via a network and parse received
byte streams into PDUs.

Constructing `PDU`s is done via builder pattern constructors.

Given a buffer with data from the network, the library can return multiple `PDU`s in multiple `DIS` versions present in
the buffer.

The library supports versions `6` and `7` of the standard, and features experimental support for version `8` based on
_draft 5_.

**Generation 2** (`v6`/`v7`): As a rule of thumb, the lib is modeled mostly towards supporting `v7` (in terms of how the
data is modeled), and
provides compatibility with how thing were in v6 mostly transparent for the user (i.e., an incoming `v6` `PDU` is parsed
and then stored in a `v7` model and put back on the wire based on the version specified in the header).

**Generation 3**: Like the protocol itself, the generation 3 library is not backwards compatible with generation 2 in
terms of modeling the PDUs and records. While organized and modeled highly similar, the structs and functions are
completely separate.

This crate re-exports the separate implementations of the crates `dis-assemble-gen-2` and `dis-assemble-gen-3`.

Originally this crate contained the main implementation for versions 6 and 7. With the introduction of v8, generations 2
and 3 are implemented separately and re-exported in the `dis-rs` crate for maintaining compatibility, and introducing
organizational consistency.

## Usage

See the bundled crates' README.md files for usage information:

- [dis-assemble-gen-2](https://github.com/zlubsen/dis-rs/blob/main/dis-assemble-gen-2/README.md)
- [dis-assemble-gen-3](https://github.com/zlubsen/dis-rs/blob/main/dis-assemble-gen-3/README.md)

## Crate feature flags

The crate offers optional feature flags:

- "serde": Adds support for `serde` to the models. See the example `serde-json` for details.
- "gen3": Adds support for v8/generation 3 of the protocol.
