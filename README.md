# DIS for Rust

[![release](https://github.com/zlubsen/dis-rs/actions/workflows/release.yml/badge.svg)](https://github.com/zlubsen/dis-rs/actions/workflows/release.yml)
[![codecov](https://codecov.io/github/zlubsen/dis-rs/graph/badge.svg?token=W40X6L5A0D)](https://codecov.io/github/zlubsen/dis-rs)

`dis-rs` is a suite of libraries and applications centered around the Distributed Interactive Simulation protocol (IEEE
1278.1), implemented in the Rust programming language and for use in Rust-based simulation applications.

This repository hosts a number of subprojects, each located in a specific directory:

- [dis-rs](https://github.com/zlubsen/dis-rs/tree/master/dis-rs)\
  Foundational implementation of DIS (IEEE-1278.1, offering models, parsing, serialization and enumerations for DIS
  PDUs.
- [cdis-assemble](https://github.com/zlubsen/dis-rs/tree/master/cdis-assemble)\
  Foundational implementation of C-DIS (SISO-STD-023-2024), offering models, parsing, serialization and conversion (to
  and from DIS) for C-DIS PDUs.
- [cdis-gateway](https://github.com/zlubsen/dis-rs/tree/master/cdis-gateway)\
  CLI application for encoding/decoding DIS to/from C-DIS.
- [gateway-core](https://github.com/zlubsen/dis-rs/tree/master/gateway-core)\
  Core library of traits and components to build network gateways handling (typically simulation) data.

See `README.md` of each project for more details.

## Goal and vision

The goal of `dis-rs` is to provide the libraries and tools to use the DIS protocol in Rust-based applications
Currently, the basic implementation of the most relevant DIS PDUs is present, making the library usable for simulation
applications
Additionally, a gateway implementing C-DIS is available

We intend to develop this project into a suite of tools that make it easy to integrate DIS into applications, having:

- A library that provides features beyond basic construction and usage of PDUs (reading/writing from a network using UDP
  sockets, heartbeats and dead reckoning logic, etc.)
- Application(s) to monitor and manage DIS network traffic (statistics, network load, filtering, routing)
- Application(s) to monitor DIS simulation data/state (map, view, interactions)

## Supported versions and maturity

`dis-rs` focuses on versions 6 and 7 of the DIS protocol. Version 7 is considered leading, meaning that v6 PDUs are
mapped to v7 models transparently to the user (irregardless of the wire-format version).

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
