# Changelog for crate CDIS-ASSEMBLE

## 0.2.0 - 2024-xx-xx

Support for new PDUs:
- Collision
- Detonation
- CreateEntity
- RemoveEntity
- StartResume
- StopFreeze
- Acknowledge
- ActionRequest
- SetData
- Data
- EventReport

### Open issues with the standard document:
- Detonation: DescriptorRecord in C-DIS does not specify the format for an Explosion Descriptor record (having enum explosive_material and f32 explosive_force values), which is a valid field value in DIS v7.
- Collision: 13.5.b states entity location units to be in Centimeters or Dekameters; Table 49 states the possible units to be Cm and Meters, which is in line with the similar field in the Detonation PDU.
- StopFreeze: field frozen_behavior is specified as having 2-bit length, but UID 68 is a 3-bit bitfield.

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