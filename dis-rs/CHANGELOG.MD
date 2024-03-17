# Changelog for crate DIS-RS

## 0.8.0 - 2024-xx-yy

Changed the main serialization function for `PDU` to be fallible. It now returns `DisError::InsufficientBufferSize` when 
the provided buffer does not have enough capacity to serialize the entire PDU. This should prevent the library from panicking during serialization.
The `Serialize` trait is not changed, `PDU` now no longer implements this trait but provides the function with the changed signature (returning `Result<u16, DisError>`).
The impls for the `Serialize` and `SerializePdu` traits are not changed and could still panic, but are practically only to be called via `PDU::serialize()`, thus behind the capacity check in that method.

## 0.7.0 - 2024-03-12
Added Logistics PDUs (ServiceRequest, ResupplyOffer, ResupplyReceived, ResupplyCancel, RepairComplete, RepairResponse)

Added Simulation Management with Reliability PDUs

Added EntityManagement PDUs (IsPartOf, IsGroupOf, TransferOwnership, AggregateState)

Added SEES

Updated Enumerations to SISO-REF-010 v32

Added convenience trait implementations (FromStr, Display, TryFrom, Hash) for records EntityType, AggregateType.

Improved references to the specification in code.

Minor hygiene fixes.

## 0.6.0 - 2024-02-04
Refactored all PDUs to have more clean builders (in code / API).

Added Internal Consistency Tests for all PDUs (create a PDU, write to buffer, parse back in, compare).

## 0.6.0-beta-2 - 2024-02-02
Fix for Signal PDU processing (data length in bits vs bytes).

## 0.6.0-beta - 2024-02-01
Added IFF PDU.

Timestamps are now modeled according to the standard instead of just an u32 field. Use by means of casting a ```TimeStamp``` (which just wraps a u32 for compatibility) to a ```DisTimeStamp```.

Refactoring all PDUs to have 1) more neat builders, 2) internal consistency tests and 3) improving references to the standard document in the code.

## 0.5.2 - 2023-10-04
Fixed issues #1 and #2. Public export of Serialize trait, and malformed v7 PDU Header related to the PDU Status field.

## 0.5.1 - 2023-08-24
Sanitize EntityMarking values parsed from the PDU.

## 0.5.0 - 2023-04-01
Added support for ActionRequest, ActionResponse, DataQuery, SetData, Data, EventReport, Comment PDUs.

## 0.4.0 - 2023-02-18
Added support for StartResume, StopFreeze, Acknowledge, CollisionElastic, EntityStateUpdate, Attribute, Designator, Signal, Transmitter, Receiver PDUs.

## 0.3.2 - 2023-01-21
Added export for Electromagnetic Emissions and Other PDUs.

## 0.3.1 - 2023-01-18
Updated README.MD and added CHANGELOG.MD (this file).

## 0.3.0 - 2023-01-18
Added support for Electromagnetic Emissions PDU.

## 0.2.1 - 2022-11-07
Added Eq and Hash derives for generated enums and bitfields.

## 0.2.0 - 2022-11-02
Added support for Fire, Detonation, Collision PDUs.
Major refactoring of internals.