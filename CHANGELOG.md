<!-- markdownlint-disable MD024 -->

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Added

### Changed

- `DisError` now uses `thiserror` crate that transitively implements `std::error::Error`
- Overall repository improvements

### Deprecated

### Removed

### Fixed

### Security

## [0.8.0] - 2024-04-03

### Added

- `UnderwaterAcoustics` PDU

### Changed

- Export `BodyInfo` and `Interaction` traits
- Changed the main serialization function for `PDU` to be fallible.
  It now returns `DisError::InsufficientBufferSize` when the provided buffer does not have enough capacity to serialize the entire `PDU`.
  This should prevent the library from panicking during serialization.
  `Serialize` trait is not changed, `PDU` now no longer implements this trait but provides the function with the changed signature (returning `Result<u16, DisError>`).
  `impl`s for the `Serialize` and `SerializePdu` traits are not changed and could still panic, but are practically only to be called via `PDU::serialize()`, thus behind the capacity check in that method

## [0.7.0] - 2024-03-12

### Added

- Logistics PDUs:
  - `ServiceRequest` PDU
  - `ResupplyOffer` PDU
  - `ResupplyReceived` PDU
  - `ResupplyCancel` PDU
  - `RepairComplete` PDU
  - `RepairResponse` PDU
- Simulation management with reliability PDUs
- `SEES` PDU
- EntityManagement PDUs:
  - `IsPartOf` PDU
  - `IsGroupOf` PDU
  - `TransferOwnership` PDU
  - `AggregateState` PDU
- Convenience trait implementations (`FromStr`, `Display`, `TryFrom`, `Hash`) for records `EntityType` and `AggregateType`
- Improved references to the specification in code

### Changed

- Updated Enumerations to `SISO-REF-010 v32`

### Fixed

- Minor hygiene fixes

## [0.6.0] - 2024-02-04

### Added

- `IFF` PDU
- Internal consistency tests for all PDUs:
    1. Create a PDU
    2. Write to buffer
    3. Parse back in
    4. Compare

### Changed

- Refactoring all PDUs to have:
    1. More cleaner builders
    2. Internal consistency tests
    3. Improving references to the standard document in the code
- Timestamps are now modeled according to the standard instead of just an `u32` field.
  Use by means of casting a `TimeStamp` (which just wraps a `u32` for compatibility) to a `DisTimeStamp`

### Fixed

- `Signal` PDU processing (data length in bits vs bytes)

## [0.5.2] - 2023-10-04

### Added

- Public export of `Serialize` trait

### Fixed

- Issue [#1](https://github.com/zlubsen/dis-rs/issues/1)
- Issue [#2](https://github.com/zlubsen/dis-rs/issues/2)
- Malformed v7 PDU header related to the PDU Status field

## [0.5.1] - 2023-08-24

### Fixed

- Sanitize `EntityMarking` values parsed from the PDU

## [0.5.0] - 2023-04-01

### Added

- `ActionRequest` PDU
- `ActionResponse` PDU
- `DataQuery` PDU
- `SetData` PDU
- `Data` PDU
- `EventReport` PDU
- `Comment` PDU

## [0.4.0] - 2023-02-18

### Added

- `StartResume` PDU
- `StopFreeze` PDU
- `Acknowledge` PDU
- `CollisionElastic` PDU
- `EntityStateUpdate` PDU
- `Attribute` PDU
- `Designator` PDU
- `Signal` PDU
- `Transmitter` PDU
- `Receiver` PDU

## [0.3.2] - 2023-01-21

### Added

- Export for `ElectromagneticEmission` and `Other` PDUs

## [0.3.1] - 2023-01-18

### Added

- `CHANGELOG.md` file

### Changed

- `README.md` file

## [0.3.0] - 2023-01-18

### Added

- `ElectromagneticEmission` PDU

## [0.2.1] - 2022-11-07

### Added

- `Eq` and `Hash` derives for generated enums and bitfields

## [0.2.0] - 2022-11-02

### Added

- `Fire` PDU
- `Detonation` PDU
- `Collision` PDU

### Changed

- Major refactoring of internals
