# DIS Infra Core

Primitives and facilities to create networking infrastructure for simulation applications.

## Concept

The basic concept of the infra-core is that there are `Nodes` that are connected to each other via `Channels`, to
exchange data.
Each node performs a single task, such as listening/writing to sockets, parsing packets, filtering data, etc.
Each node has an incoming and an outgoing channel through which typed data can be received and send.

Several basic nodes for working with network socket (UDP, TCP) and DIS (parse, serialize) are built-in. Custom nodes can
be added as 'plugin modules'.

Gateways are created by specifying the nodes and channel connections in a `TOML`-based format.

## Usage

See the repo-level examples for usage and explanation of the fundamentals.

- `examples/basic-usage-gateway` - Explanation of the fundamentals of building a gateway.
- `examples/custom-gateway-node` - Explanation of developing and using a custom node.
- `examples/tcp_server_gateway` - Demonstration of using a TCP Server node.
- `examples/udp_dis_read_and_write` - Demonstration of a gateway which listens for DIS traffic (UDP broadcast), parses
  the PDU, and sends the PDU back out on the network.
