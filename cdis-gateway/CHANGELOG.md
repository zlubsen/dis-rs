<!-- markdownlint-disable MD024 -->

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security

## [0.3.0] - 2024-11-27

### Added

- New PDUs (aligned with `cdis-assemble`):
  - `Electromagnetic Emission`
  - `Designator`
  - `Transmitter`
  - `Signal`
  - `Receiver`
  - `IFF`
- Proper implementation of `block_own_host` and uri `EndPointSpec` configuration options ([#36](https://github.com/zlubsen/dis-rs/issues/36) and [#37](https://github.com/zlubsen/dis-rs/issues/37))

### Changed

### Deprecated

### Removed

### Fixed

### Security

## [0.2.0] - 2024-07-15

### Added

- New PDUs (aligned with `cdis-assemble`):
  - `Collision`
  - `Detonation`
  - `CreateEntity`
  - `RemoveEntity`
  - `StartResume`
  - `StopFreeze`
  - `Acknowledge`
  - `ActionRequest`
  - `DataQuery`
  - `SetData`
  - `Data`
  - `EventReport`
  - `Comment`

## [0.1.1] - 2024-06-26

### Added

- `Fire` PDU

## [0.1.0] - 2024-06-19

### Added

- First implementation of the `cdis-gateway` crate (binary).
    It implements an Encoder/Decoder-pair application for C-DIS (SISO-STD-023-2024).
    A self-hosted site is available to monitor the gateway.
    The gateway supports converting `EntityState` PDUs.
