# DIS for Rust

dis-rs is a suite of libraries and applications centered around the Distributed Interactive Simulation protocol (IEEE 
1278.1), implemented in the Rust programming language and for use in Rust-based simulation applications.

This repository hosts a number of sub-projects, each located in a directory in this repository:
- **dis-rs**: Foundational implementation of DIS, offering models, parsing, serialisation and enumerations for DIS PDUs.
- **cdis-assemble**: Foundational implementation of C-DIS (SISO-STD-023-2024), offering models, parsing, serialisation and conversion (to and from DIS) for C-DIS PDUs.
- **cdis-gateway**: CLI application for encoding/decoding DIS to/from C-DIS.

See the README.MD files for each project for specific details.

## Goal and vision
The goal of dis-rs is to provide the libraries and tools to use the DIS protocol in Rust-based applications.
Currently, the basic implementation of the most relevant DIS PDUs is present, making the library usable for simulation applications.
Additionally, a gateway implementing C-DIS is available. 

We intend to develop this project into a suite of tools that make it easy to integrate DIS into applications, having:
- a library that provides features beyond basic construction and usage of PDUs (reading/writing from a network using UDP sockets, heartbeats and dead reckoning logic, etc.).
- application(s) to monitor and manage DIS network traffic (statistics, network load, filtering, routing)
- application(s) to monitor DIS simulation data / state (map view, interactions)

## Supported versions and maturity
dis-rs focuses on versions 6 and 7 of the DIS protocol. Version 7 is considered leading, meaning that v6 PDUs are mapped to v7 models transparently to the user (irregardless of the wire-format version).

## Resources

- SISO: https://www.sisostds.org - Organisation maintaining the DIS and C-DIS standards and reference material.
- A lot of great background material on DIS can be found at OpenDIS: http://open-dis.org and https://github.com/open-dis.
- Wikipedia: https://en.wikipedia.org/wiki/Distributed_Interactive_Simulation.
- DIS Data Dictionary (version 6 only): http://faculty.nps.edu/brutzman/vrtp/mil/navy/nps/disEnumerations/JdbeHtmlFiles/pdu/.

Copyright (C) 2024 Zeeger Lubsen