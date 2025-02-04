<!-- markdownlint-disable MD024 -->

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Added

### Changed

- Updated dependencies

### Deprecated

### Removed

### Fixed

- All kinds of code style changes due to applying `rustfmt` and `clippy`

### Security

## [0.3.0] - 2024-11-27

### Added

- `Electromagnetic Emission` PDU
- `Designator` PDU
- `Transmitter` PDU
- `Signal` PDU
- `Receiver` PDU
- `IFF` PDU

### Changed

### Deprecated

### Removed

- `use_guise` configuration option, which was redundant. The need for encoding the alternative entity type in the
  `EntityState` PDU is determined based on the value of the field.

### Fixed

### Security

## [0.2.0] - 2024-07-15

### Added

- `Collision` PDU
- `Detonation` PDU
- `CreateEntity` PDU
- `RemoveEntity` PDU
- `StartResume` PDU
- `StopFreeze` PDU
- `Acknowledge` PDU
- `ActionRequest` PDU
- `DataQuery` PDU
- `SetData` PDU
- `Data` PDU
- `EventReport` PDU
- `Comment` PDU

### Security

- Unclear specification
    - `Detonation DescriptorRecord`: in C-DIS does not specify the format for an `Explosion Descriptor Record` (having
      enum explosive_material and `f32` explosive_force values), which is a valid field value in DIS v7
    - `Signal` PDU: Encoding Scheme record: the DIS variants other than EncodedAudio and RawBinaryData are not
      represented in C-DIS.
- Inconsistent specification
    - `Collision`: `13.5.b` states entity location units to be in Centimeters or Dekameters; Table 49 states the
      possible units to be Cm and Meters, which is in line with the similar field in the `Detonation` PDU
    - `StopFreeze`: bitfield frozen_behavior is specified as having 2-bit length in CDIS, but UID 68 is a 3-bit
      bitfield (Implementation decision: use 3 bits for the bitfield)
    - `EE`: page 90 j - Beam Status - Status is on if Flag = 1 or off if Flag = 0; the SISO-REF-010 lists a value of 0
      as ACTIVE, and 1 as DEACTIVATED, which is the opposite when considering the on-wire values
    - `BeamData` record: Table 17 lists the field Beam Sweep Sync to be SVINT13, but otherwise it is described as an
      unsigned 10-bit field (in text 11.3.e, table 62, table 68)
- Textual issue
    - `EE`: page 94 - Jamming Technique Flag - bit size not listed (should be 1)
    - `p104`: typo 'Cyrpto Key ID'
    - CDIS table 71 states 16 bits more padding before nr of IFF data records is part of the basic data field instead of
      layer 4 itself. Compare to dis v7 table B.41

## [0.1.1] - 2024-06-26

### Added

- `Fire` PDU

## [0.1.0] - 2024-06-19

### Added

- First implementation of the `cdis-assemble` crate.
  Models for CDIS specific types, records and basic structures such as `CdisPdu`, `CdisBody`.
  Facilities for parsing C-DIS bit-streams into C-DIS models.
  Facilities for writing C-DIS models into a bit-stream.
  Facilities for encoding and decoding DIS PDUs into and from C-DIS PDUs (and thereby all involved types, records).
  Additional data structs for handling state and settings used in an Encoder/Decoder.
