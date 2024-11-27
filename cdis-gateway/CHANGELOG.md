<!-- markdownlint-disable MD024 -->

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Added

- New PDUs (aligned with `cdis-assemble`):
  - `Electromagnetic Emission`
  - `Designator`
  - `Transmitter`
  - `Signal`
  - `Receiver`
  - `IFF`

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
