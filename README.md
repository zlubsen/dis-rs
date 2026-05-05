# IEEE 1278.1 Distributed Interactive Simulation for Rust

[![release](https://github.com/zlubsen/dis-rs/actions/workflows/release.yml/badge.svg)](https://github.com/zlubsen/dis-rs/actions/workflows/release.yml)
[![codecov](https://codecov.io/github/zlubsen/dis-rs/graph/badge.svg?token=W40X6L5A0D)](https://codecov.io/github/zlubsen/dis-rs)

`dis-rs` is a suite of libraries and applications centered around the Distributed Interactive Simulation protocol (IEEE
1278.1 - DIS), implemented in the Rust programming language and for use in Rust-based simulation applications.

This repository hosts a number of subprojects, each located in a specific directory:

- [dis-rs](https://github.com/zlubsen/dis-rs/tree/master/dis-rs)\
  Crate re-exporting _dis-assemble-gen-2_ and (feature gated) _dis-assemble-gen-3_. Originally contained the main DIS
  implementation for versions 6 and 7, which are now moved to _dis-assemble-gen-2_.
- [dis-assemble-gen-2](https://github.com/zlubsen/dis-rs/tree/master/dis-assemble-gen-2)\
  Foundational implementation of DIS (IEEE-1278.1) **Generation 2** (pre-version 8), offering models, parsing,
  serialization and enumerations for DIS
  PDUs.
- [dis-assemble-gen-3](https://github.com/zlubsen/dis-rs/tree/master/dis-assemble-gen-3)\
  Foundational implementation of DIS (IEEE-1278.1) **Generation 3** (version 8), offering models, parsing, serialization
  and enumerations for DIS
  PDUs.
- [cdis-assemble](https://github.com/zlubsen/dis-rs/tree/master/cdis-assemble)\
  Foundational implementation of C-DIS (SISO-STD-023-2024), offering models, parsing, serialization and conversion (to
  and from DIS) for C-DIS PDUs.
- [cdis-gateway](https://github.com/zlubsen/dis-rs/tree/master/cdis-gateway)\
  CLI / Web-UI application for encoding/decoding DIS to/from C-DIS.
- [gateway-core](https://github.com/zlubsen/dis-rs/tree/master/gateway-core)\
  Core library of traits and components to build network gateways handling (typically simulation) data.
- [dis-gen-ieee-1278-gen3](https://github.com/zlubsen/dis-rs/tree/master/dis-gen-ieee-1278-gen-3)\
  Generator for all DIS Gen 3 models, parsers, and serializers. The output is the main component of
  _dis-assemble-gen-3_.
- [dis-gen-siso-ref-010](https://github.com/zlubsen/dis-rs/tree/master/dis-gen-siso-ref-010)\
  Generator for DIS SISO-REF-010 enumerations code. Used for both _dis-assemble-gen-2_ and _dis-assemble-gen-3_.
- [dis-gen-utils](https://github.com/zlubsen/dis-rs/tree/master/dis-gen-utils)\
  Utils for code generators (_dis-gen-ieee-1278-gen3_ and _dis-gen-siso-ref-010_)

See `README.md` of each project for more details.

## Goal and vision

The goal of `dis-rs` is to provide the libraries and tools to use the DIS protocol in Rust-based applications
Currently, the basic implementation of the most relevant DIS PDUs is present, making the library usable for simulation
applications.
Additionally, a gateway implementing C-DIS is available

We intend to develop this project into a suite of tools that make it easy to integrate DIS into applications, having:

- A library that provides features beyond basic construction and usage of PDUs (reading/writing from a network using UDP
  sockets, heartbeats and dead reckoning logic, etc.)
- Application(s) to monitor and manage DIS network traffic (statistics, network load, filtering, routing)
- Application(s) to monitor DIS simulation data/state (map, view, interactions)

## Supported versions and maturity

`dis-assemble-gen-2` implements versions 6 and 7 of the DIS protocol. Version 7 is considered leading, meaning that v6
PDUs are
mapped to v7 models transparently to the user (regardless of the wire-format version). Some lesser used PDUs are not
implemented.
This implementation is relatively mature.

`dis-assemble-gen-3` implements version 8 of the DIS protocol. Currently, it is based on **draft 5** of the future
standard.
Most of the code is generated from the DIS v8 schema definitions, and the main structure and functions in the
implementation are based on the Gen 2 implementation.
In this sense the library is relatively mature, but being one of the earlier implementations it is not battle-tested (
e.g., used in exercises and against different implementations). Until the v8 standard is final and used in real-world
applications, this library is considered experimental.

`cdis-assemble` implements version 1 of the C-DIS protocol.
We're not aware of other (openly available) C-DIS implementations or practical real-life usage of the standard for that
matter, so operational maturity is to be considered experimental.

## Resources

- SISO: <https://www.sisostds.org> - Organization maintaining the DIS and C-DIS standards and reference material
- A lot of great background material on DIS can be found at OpenDIS: <http://open-dis.org>
  and <https://github.com/open-dis>
- Wikipedia: <https://wikipedia.org/wiki/Distributed_Interactive_Simulation>
- DIS Data Dictionary (version 6
  only): <http://faculty.nps.edu/brutzman/vrtp/mil/navy/nps/disEnumerations/JdbeHtmlFiles/pdu>

## License

This project is licensed under the [MIT](https://opensource.org/licenses/MIT) License. \
See [LICENSE](./LICENSE) file for details.
