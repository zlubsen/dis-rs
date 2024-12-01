# cdis-gateway

The `C-DIS Gateway` implements a functional Encoder/Decoder pair for the C-DIS protocol (SISO-STD-023-2024).
The gateway reads regular DIS PDUs from an UDP network socket, converts the PDU to C-DIS and sends it out over the
network via another UDP socket.

## Basic Usage

The compiled binary can be executed from the CLI.
The application takes one (positional) argument, the configuration file.

```sh
cdis-gateway <PATH_TO_FILE>
```

And similarly using `cargo run -- <PATH_TO_FILE>`.

### Configuration

The gateway is configured via a configuration file.
The main point of the configuration is for network sockets (interface, port, unicast/broadcast/multicast modes)

See [config/sample_config.toml](./config/sample_config.toml) for details and explanation of the various configuration
items.

### Logging

The log level of the gateway defaults to `INFO`, outputting the initial settings and not much more. This can be
overwritten by setting the `RUST_LOG` environment variable.

### Web UI

The gateway hosts a basic web interface to monitor the functioning of the gateway.
The port at which the application is available can be configured in the config file, it defaults to port `8080`.
