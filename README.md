# DIS for Rust

dis-lib is an implementation of the Distributed Interactive Simulation (DIS) protocol for Rust. It provides functions to build PDUs in applications, send them out via a network and parse received byte streams into PDUs.

Constructing PDUs is done via builder pattern constructors.

Given a buffer, the lib can return multiple PDUs in multiple DIS versions present in the buffer. 

## Features

Here is an overview of the DIS features supported by dis-lib. 'Read' means reading a PDU from a byte stream. 'Write' means constructing a PDU in a struct and serializing it to a buffer. 

| PDU / function | v6 read | v6 write | v7 read | v7 write |
| --- | --- | --- | --- | --- |
| PDU Header | V | V | | | 
| EntityState PDU | V | V | | |
| FirePdu |  |  |  |  |
| DetonationPdu |  |  |  |  |
| CollisionPdu |  |  |  |  |
| ServiceRequestPdu |  |  |  |  |
| ResupplyOfferPdu |  |  |  |  |
| ResupplyReceivedPdu |  |  |  |  |
| ResupplyCancelPdu |  |  |  |  |
| RepairCompletePdu |  |  |  |  |
| RepairResponsePdu |  |  |  |  |
| CreateEntityPdu |  |  |  |  |
| RemoveEntityPdu |  |  |  |  |
| StartResumePdu |  |  |  |  |
| StopFreezePdu |  |  |  |  |
| AcknowledgePdu |  |  |  |  |
| ActionRequestPdu |  |  |  |  |
| ActionResponsePdu |  |  |  |  |
| DataQueryPdu |  |  |  |  |
| SetDataPdu |  |  |  |  |
| DataPdu |  |  |  |  |
| EventReportPdu |  |  |  |  |
| CommentPdu |  |  |  |  |
| ElectromagneticEmissionPdu |  |  |  |  |
| DesignatorPdu |  |  |  |  |
| TransmitterPdu |  |  |  |  |
| SignalPdu |  |  |  |  |
| ReceiverPdu |  |  |  |  |
| AnnounceObjectPdu |  |  |  |  |
| DeleteObjectPdu |  |  |  |  |
| DescribeApplicationPdu |  |  |  |  |
| DescribeEventPdu |  |  |  |  |
| DescribeObjectPdu |  |  |  |  |
| RequestEventPdu |  |  |  |  |
| RequestObjectPdu |  |  |  |  |
| TimeSpacePositionIndicatorFIPdu |  |  |  |  |
| AppearanceFIPdu |  |  |  |  |
| ArticulatedPartsFIPdu |  |  |  |  |
| FireFIPdu |  |  |  |  |
| DetonationFIPdu |  |  |  |  |
| PointObjectStatePdu |  |  |  |  |
| LinearObjectStatePdu |  |  |  |  |
| ArealObjectStatePdu |  |  |  |  |
| EnvironmentPdu |  |  |  |  |
| TransferControlRequestPdu |  |  |  |  |
| TransferControlPdu |  |  |  |  |
| TransferControlAcknowledgePdu |  |  |  |  |
| IntercomControlPdu |  |  |  |  |
| IntercomSignalPdu |  |  |  |  |
| AggregatePdu |  |  |  |  |
| 'Other' PDU | V | V | | |
| Dead Reckoning Algos | | | | |

# Repositories

The library currently consists of two crates:
- dis-lib: main library containing PDU definitions and parsers, builders, etc
- dis-derive: lib containing derive macros used by dis_lib
