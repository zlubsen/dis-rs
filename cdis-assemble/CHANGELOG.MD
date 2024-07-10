# Changelog for crate CDIS-ASSEMBLE

## 0.2.0 - 2024-xx-xx

Support for new PDUs:
- Collision
- Detonation
- CreateEntity
- RemoveEntity
- StartResume
- StopFreeze

## 0.1.1 - 2024-06-26

Fire PDU support.

## 0.1.0 - 2024-06-19

First implementation of the `cdis-assemble` crate.

It provides:
- Models for CDIS specific types, records and basic structures such as `CdisPdu`, `CdisBody`.
- Facilities for parsing C-DIS bit-streams into C-DIS models.
- Facilities for writing C-DIS models into a bit-stream.
- Facilities for encoding and decoding DIS PDUs into and from C-DIS PDUs (and thereby all involved types, records).
- Additional data structs for handling state and settings used in an Encoder/Decoder.