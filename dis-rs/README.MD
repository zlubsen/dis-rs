# DIS for Rust

dis-rs is an implementation of the Distributed Interactive Simulation (DIS) protocol for Rust. It provides structures and functions to build PDUs in applications, send them out via a network and parse received byte streams into PDUs.

Constructing PDUs is done via builder pattern constructors.

Given a buffer with data from the network, the library can return multiple PDUs in multiple DIS versions present in the buffer.

The library supports both versions 6 and 7 of the standard.

## Features

Here is an overview of the DIS PDUs/features supported by dis-rs. 'Read' means reading a PDU from a byte stream. 'Write' means constructing a PDU in a struct and serializing it to a buffer. 

| PDU / function | v6 read | v6 write | v7 read | v7 write |
| --- |---------|----------|---------|----------|
| PDU Header | V       | V        | V       | V        | 
| EntityState | V       | V        | V       | V        |
| Fire | V       | V        | V       | V        |
| Detonation | V       | V        | V       | V        |
| Collision | V       | V        | V       | V        |
| ServiceRequestPdu |         |          |         |          |
| ResupplyOfferPdu |         |          |         |          |
| ResupplyReceivedPdu |         |          |         |          |
| ResupplyCancelPdu |         |          |         |          |
| RepairCompletePdu |         |          |         |          |
| RepairResponsePdu |         |          |         |          |
| CreateEntityPdu |         |          |         |          |
| RemoveEntityPdu |         |          |         |          |
| StartResumePdu | V       | V        | V       | V        |
| StopFreezePdu | V       | V        | V       | V        |
| AcknowledgePdu | V       | V        | V       | V        |
| ActionRequestPdu |         |          |         |          |
| ActionResponsePdu |         |          |         |          |
| DataQueryPdu |         |          |         |          |
| SetDataPdu |         |          |         |          |
| DataPdu |         |          |         |          |
| EventReportPdu |         |          |         |          |
| CommentPdu |         |          |         |          |
| ElectromagneticEmissionPdu | V       | V        | V       | V        |
| DesignatorPdu | V       | V        | V       | V        |
| TransmitterPdu | V       | V        | V       | V        |
| SignalPdu | V       | V        | V       | V        |
| ReceiverPdu | V       | V        | V       | V        |
| AnnounceObjectPdu |         |          |         |          |
| DeleteObjectPdu |         |          |         |          |
| DescribeApplicationPdu |         |          |         |          |
| DescribeEventPdu |         |          |         |          |
| DescribeObjectPdu |         |          |         |          |
| RequestEventPdu |         |          |         |          |
| RequestObjectPdu |         |          |         |          |
| TimeSpacePositionIndicatorFIPdu |         |          |         |          |
| AppearanceFIPdu |         |          |         |          |
| ArticulatedPartsFIPdu |         |          |         |          |
| FireFIPdu |         |          |         |          |
| DetonationFIPdu |         |          |         |          |
| PointObjectStatePdu |         |          |         |          |
| LinearObjectStatePdu |         |          |         |          |
| ArealObjectStatePdu |         |          |         |          |
| EnvironmentPdu |         |          |         |          |
| TransferControlRequestPdu |         |          |         |          |
| TransferControlPdu |         |          |         |          |
| TransferControlAcknowledgePdu |         |          |         |          |
| IntercomControlPdu |         |          |         |          |
| IntercomSignalPdu |         |          |         |          |
| AggregatePdu |         |          |         |          |
| 'Other' PDU | V       | V        | V       | V        |
| Dead Reckoning Algos |         |          |         |          |

### Enumerations
dis-rs uses the SISO-REF-010 reference to map the wire level encoding to actual names of enumerations and values in code.
E.g., one can use the enum `PduType::EntityState` in code instead of remembering that a `1` means that specific value.

The code for these enums is generated using a build script from the published SISO-REF-010.xml file. Which is currently a v30.

## Usage

### Constructing PDUs
PDUs are constructed using idiomatic `new()` or `default()` functions on the structs for `PduHeader` or `PduBody`s.
Some `PduBody` variants require the `new()` constructors to have arguments.
Further fields are set using `.with_field_name()` functions.

The main data structure is a `Pdu`, which consists of a `header` and a `body`. The body is a variant of `PduBody`, an enum that wraps a specific struct for that PDU type in a variant.
The specific body structs, e.g. an `EntityState`, can be wrapped / converted to a `PduBody` by call the `to_pdu_body()` function on the struct.
Further, the body can be merged with a `PduHeader` using the associated function `finalize_from_parts(header, body, timestamp)` of `Pdu`. This will give you a complete PDU.

### Parsing
The library exposes three functions to parse data into PDUs from a buffer: `parse()`, `parse_v6()` and `parse_v7()`.
Each function works the same, where the general `parse()` function returns all valid PDUs from the buffer and the others filter out v6 or v7 version PDUs.

### Serializing
To serialize a PDU to bytes (DIS wire format), simply call the `serialize()` function on a `Pdu`, providing the buffer as argument.

## Resources

- SISO: https://www.sisostds.org - Organisation maintaining the DIS standard and reference material.
- A lot of great background material on DIS can be found at OpenDIS: http://open-dis.org and https://github.com/open-dis.
- Wikipedia: https://en.wikipedia.org/wiki/Distributed_Interactive_Simulation.
- DIS Data Dictionary (version 6 only): http://faculty.nps.edu/brutzman/vrtp/mil/navy/nps/disEnumerations/JdbeHtmlFiles/pdu/.

Copyright (C) 2022 Zeeger Lubsen