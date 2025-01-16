# DIS Infra Core

Primitives and facilities to create networking infrastructure for simulation applications.

## Concept

The basic concept of the infra-core is that we have `Nodes` that are connected to each other via channels.
Each node performs a single task, such as listening/writing to sockets, parsing packets, filtering data, etc.

