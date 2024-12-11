# dis-rs

`dis-rs` is an implementation of the `Distributed Interactive Simulation` (`DIS`) protocol for Rust.
It provides structures and functions to build `PDU`s in applications, send them out via a network and parse received
byte streams into PDUs.

Constructing `PDU`s is done via builder pattern constructors.

Given a buffer with data from the network, the library can return multiple `PDU`s in multiple `DIS` versions present in
the buffer.

The library supports both versions `6` and `7` of the standard.
As a rule of thumb, the lib is modeled mostly towards supporting `v7` (in terms of how the data is modelled), and
provides compatibility with how thing were in v6 mostly transparent for the user (i.e., an incoming `v6` `PDU` is parsed
and then stored in a `v7` model and put back on the wire based on the version specified in the header).

## Features

Here is an overview of the `DIS` `PDU`s/features supported by `dis-rs`:

| PDU                             |      Support       |
|:--------------------------------|:------------------:|
| Other                           | :heavy_check_mark: |
| EntityState                     | :heavy_check_mark: |
| Fire                            | :heavy_check_mark: |
| Detonation                      | :heavy_check_mark: |
| Collision                       | :heavy_check_mark: |
| ServiceRequest                  | :heavy_check_mark: |
| ResupplyOffer                   | :heavy_check_mark: |
| ResupplyReceived                | :heavy_check_mark: |
| ResupplyCancel                  | :heavy_check_mark: |
| RepairComplete                  | :heavy_check_mark: |
| RepairResponse                  | :heavy_check_mark: |
| CreateEntity                    | :heavy_check_mark: |
| RemoveEntity                    | :heavy_check_mark: |
| StartResume                     | :heavy_check_mark: |
| StopFreeze                      | :heavy_check_mark: |
| Acknowledge                     | :heavy_check_mark: |
| ActionRequest                   | :heavy_check_mark: |
| ActionResponse                  | :heavy_check_mark: |
| DataQuery                       | :heavy_check_mark: |
| SetData                         | :heavy_check_mark: |
| Data                            | :heavy_check_mark: |
| EventReport                     | :heavy_check_mark: |
| Comment                         | :heavy_check_mark: |
| ElectromagneticEmission         | :heavy_check_mark: |
| Designator                      | :heavy_check_mark: |
| Transmitter                     | :heavy_check_mark: |
| Signal                          | :heavy_check_mark: |
| Receiver                        | :heavy_check_mark: |
| IFF                             | :heavy_check_mark: |
| UnderwaterAcoustic              | :heavy_check_mark: |
| SupplementalEmissionEntityState | :heavy_check_mark: |
| IntercomSignal                  |        :x:         |
| IntercomControl                 |        :x:         |
| AggregateState                  | :heavy_check_mark: |
| IsGroupOf                       | :heavy_check_mark: |
| TransferOwnership               | :heavy_check_mark: |
| IsPartOf                        | :heavy_check_mark: |
| MinefieldState                  |        :x:         |
| MinefieldQuery                  |        :x:         |
| MinefieldData                   |        :x:         |
| MinefieldResponseNACK           |        :x:         |
| EnvironmentalProcess            |        :x:         |
| GriddedData                     |        :x:         |
| PointObjectState                |        :x:         |
| LinearObjectState               |        :x:         |
| ArealObjectState                |        :x:         |
| TSPI                            |        :x:         |
| Appearance                      |        :x:         |
| ArticulatedParts                |        :x:         |
| LEFire                          |        :x:         |
| LEDetonation                    |        :x:         |
| CreateEntityR                   | :heavy_check_mark: |
| RemoveEntityR                   | :heavy_check_mark: |
| StartResumeR                    | :heavy_check_mark: |
| StopFreezeR                     | :heavy_check_mark: |
| AcknowledgeR                    | :heavy_check_mark: |
| ActionRequestR                  | :heavy_check_mark: |
| ActionResponseR                 | :heavy_check_mark: |
| DataQueryR                      | :heavy_check_mark: |
| SetDataR                        | :heavy_check_mark: |
| DataR                           | :heavy_check_mark: |
| EventReportR                    | :heavy_check_mark: |
| CommentR                        | :heavy_check_mark: |
| RecordR                         | :heavy_check_mark: |
| SetRecordR                      | :heavy_check_mark: |
| RecordQueryR                    | :heavy_check_mark: |
| CollisionElastic                | :heavy_check_mark: |
| EntityStateUpdate               | :heavy_check_mark: |
| DirectedEnergyFire              |        :x:         |
| EntityDamageStatus              |        :x:         |
| InformationOperationsAction     |        :x:         |
| InformationOperationsReport     |        :x:         |
| Attribute                       | :heavy_check_mark: |

### Enumerations

`dis-rs` uses the `SISO-REF-010` reference to map the wire level encoding to actual names of enumerations and values in
code.
E.g., one can use the enum `PduType::EntityState` in code instead of remembering that a `1` means that specific value.

The code for these enums is generated using a build script from the
published [SISO-REF-010.xml](./enumerations/SISO-REF-010.xml) file.

## Usage

### Constructing PDUs

`PDU`s are constructed using a `default()` or `builder()` associated functions on the structs for `PduHeader` or
`PduBody`s.
Using `SomePdu::builder()` constructs a Builder for the PDU.
The Builders start with the default values for PDU fields, and you have to set fields using `.with_field_name()`
functions.
The `build()` function turns the Builder into the typed body of the PDU you are building (e.g., `Signal`).
There are no internal validation or checks whether you construct a PDU with valid combinations of fields.

The main data structure is a `Pdu`, which consists of a `PduHeader` and a `PduBody`. The body is a variant of `PduBody`,
an enum that wraps a specific struct for that PDU type in a variant (such as `Pdu::Signal(Signal)`).
The specific body structs, e.g. an `EntityState`, can be wrapped / converted to a `PduBody` by call the
`into_pdu_body()` function on the struct.
Further, the body can be merged with a `PduHeader` using the associated function
`Pdu::finalize_from_parts(header, body, timestamp)`. This will give you a complete PDU.

### Parsing

The library exposes three functions to parse binary data (the DIS wire format) into PDUs from a buffer: `parse()`,
`parse_v6_pdus()` and `parse_v7_pdus()`.
Each function works the same, where the general `parse()` function returns all valid PDUs from the buffer and the others
filter out v6 or v7 version PDUs.

### Serializing

To serialize a `PDU` to bytes (the `DIS` wire format), simply call the `serialize()` function on a `Pdu`, providing the
buffer as argument.

## Crate feature flags

The crate offers one optional feature:

- "serde": Adds support for `serde` to the models. See the example `serde-json` for details.
