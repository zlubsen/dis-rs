# Changelog for C-DIS Gateway

## 0.3.0 - 2024-xx-xx

Support for new PDUs, aligned with `cdis-assemble`:
- Electromagnetic Emission
- Designator
- Transmitter
- Signal
- Receiver

## 0.2.0 - 2024-07-15

Support for new PDUs, aligned with `cdis-assemble`:
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

## 0.1.1 - 2024-06-26

Fire PDU support.

## 0.1.0 - 2024-06-19

First implementation of the `cdis-gateway` crate (binary).

It implements an Encoder/Decoder-pair application for C-DIS (SISO-STD-023-2024).
A self-hosted site is available to monitor the gateway.

The gateway supports converting EntityState PDUs. 