# C-DIS (SISO-STD-023-2024) in Rust

cdis-assemble is a foundational implementation of the Compressed-Distributed Interactive Simulation (C-DIS) protocol for Rust. It provides data structures and functions to build C-DIS PDUs, send them out via a network and parse received bit streams into C-DIS PDUs.

Its structure follows the same approach as the sibling library `dis-rs`, which implements the same base functionality for the DIS (Distributed Interactive Simulation - IEEE 1278.1) protocol in Rust.
Conversion (encoding/decoding) of DIS to C-DIS and vice versa is also part is this library.

This library largely builds on the `dis-rs` crate for things like DIS records that do not have a compressed variant, enumerations, etc.

As C-DIS is primarily intended to be used in an Encoder/Decoder pair to compress regular DIS over low-bandwidth connections, it is unlikely one needs to integrate this library into a simulation application.
See `cdis-gateway` for a functional gateway application for encoding/decoding C-DIS. As a consequence, this library offer fewer quality-of-life features compared to `dis-rs`, such as builders for data structures.

## PDU support

Here is an overview of the C-DIS PDUs supported by cdis-assemble. Any unimplemented (currently unsupported) PDUs will be treated as Other/Unsupported.

| PDU                     | Support |
|-------------------------|---------|
| Other / Unsupported     | V       |
| EntityState             | V       |
| Fire                    | V       |
| Detonation              | V       |
| Collision               | V       |
| CreateEntity            | V       |
| RemoveEntity            | V       |
| StartResume             | V       |
| StopFreeze              | V       |
| Acknowledge             | V       |
| ActionRequest           | V       |
| ActionResponse          |         |
| DataQuery               |         |
| SetData                 |         |
| Data                    |         |
| EventReport             |         |
| Comment                 |         |
| ElectromagneticEmission |         |
| Designator              |         |
| Transmitter             |         |
| Signal                  |         |
| Receiver                |         |
| IFF                     |         |

## Usage

### Constructing PDUs
The main data structure is a `CdisPdu`, which consists of a `CdisHeader` and a `CdisBody`. The body is a variant of `CdisBody`, an enum that wraps a specific struct for that PDU type in a variant (such as `CdisBody::EntityState(EntityState)`).
The specific body structs, e.g. an `EntityState`, can be wrapped / converted to a `CdisBody` by calling the `into_cdis_body()` function on the struct.
Further, the body can be merged with a `PduHeader` using the associated function `CdisPdu::finalize_from_parts(header, body, timestamp)`. This will give you a complete PDU.

### Parsing
The library exposes a function to parse binary data (the C-DIS bit-oriented wire format) into PDUs from a buffer: `parse(...)`.
Given a buffer of bytes (`&[u8]`) it will attempt to parse C-DIS PDUs, and return all found in the buffer.

### Serializing
To serialize a PDU to bits (the C-DIS bit-oriented wire format), call the `serialize()` function on a `CdisPdu`, providing a `BitBuffer` buffer as argument.
A BitBuffer can be created using the `create_bit_buffer()` function, which will allocate a appropriate buffer with enough space for the typical ethernet Maximum Transmission Unit (MTU, the maximum size of a PDU that can be communicated in a
single network layer transaction - 1500 bytes).

## Encoding / Decoding
The conversion of DIS to C-DIS and vice versa is done using the `CdisPdu::encode()` (associated function of the struct) and `CdisPdu::decode()` (method of an instance) functions.

The conversion requires, aside from the to-be converted PDU, two things:
- a state to convert 'stateful' PDUs (such as EntityState), tracking optional fields and timeouts. Either an `EncoderState` or `DecoderState`, containing specialised data structures for specific stateful PDUs. 
- a `CodecOptions` struct, providing the parameters for the conversion (such as Full/Partial update mode, heartbeat and timeout settings, ...).

See also `tests/codec_tests.rs` and tests in `src/codec.rs` for examples.

## Resources

- SISO: https://www.sisostandards.org - Organisation maintaining the DIS and C-DIS standards and reference material.
- A lot of great background material on DIS can be found at OpenDIS: http://open-dis.org and https://github.com/open-dis.
- Wikipedia: https://en.wikipedia.org/wiki/Distributed_Interactive_Simulation.
- DIS Data Dictionary (version 6 only): http://faculty.nps.edu/brutzman/vrtp/mil/navy/nps/disEnumerations/JdbeHtmlFiles/pdu/.

Copyright (C) 2024 Zeeger Lubsen