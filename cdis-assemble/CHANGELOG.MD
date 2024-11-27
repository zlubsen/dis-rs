# Changelog for crate CDIS-ASSEMBLE

## 0.3.0 - 2024-11-27

Support for new PDUs:

- Electromagnetic Emission
- Designator
- Transmitter
- Signal
- Receiver
- IFF

Removed the 'use_guise' configuration option, which was redundant. The need for encoding the alternative entity type in
the EntityState PDU is determined based on the value of the field.

## 0.2.0 - 2024-07-15

Support for new PDUs:

- Collision
- Detonation
- CreateEntity
- RemoveEntity
- StartResume
- StopFreeze
- Acknowledge
- ActionRequest
- DataQuery
- SetData
- Data
- EventReport
- Comment

### Open issues with the standard document:

- [Unclear specification] Detonation: DescriptorRecord in C-DIS does not specify the format for an Explosion Descriptor
  record (having enum explosive_material and f32 explosive_force values), which is a valid field value in DIS v7.
- [Inconsistent specification] Collision: 13.5.b states entity location units to be in Centimeters or Dekameters; Table
  49 states the possible units to be Cm and Meters, which is in line with the similar field in the Detonation PDU.
- [Inconsistent specification] StopFreeze: bitfield frozen_behavior is specified as having 2-bit length in CDIS, but UID
  68 is a 3-bit bitfield (Implementation decision: use 3 bits for the bitfield).
- [Textual issue] EE: page 94 - Jamming Technique Flag - bit size not listed (should be 1).
- [Inconsistent specification] EE: page 90 j) - Beam Status - Status is on if Flag = 1 or off if Flag = 0; the
  SISO-REF-010 lists a value of 0 as ACTIVE, and 1 as DEACTIVATED, which is the opposite when considering the on-wire
  values.
- [Inconsistent specification] BeamData record: Table 17 lists the field Beam Sweep Sync to be SVINT13, but otherwise it
  is described as an unsigned 10-bit field (in text 11.3.e, table 62, table 68).
- [Unclear specification] Signal PDU - Encoding Scheme record: the DIS variants other than EncodedAudio and
  RawBinaryData are not represented in C-DIS.
- [Textual issue] p104: typo 'Cyrpto Key ID'
- [Textual issue] CDIS table 71 states 16 bits more padding before nr of IFF data records is part of the basic data
  field instead of layer 4 itself. Compare to dis v7 table B.41.

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