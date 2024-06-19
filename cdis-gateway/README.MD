# C-DIS Gateway

The C-DIS Gateway implements a functional Encoder/Decoder pair for the C-DIS protocol (SISO-STD-023-2024).
The gateway reads regular DIS PDUs from an UDP network socket, converts the PDU to C-DIS and sends it out over the network via another UDP socket.

## Basic Usage

The compiled binary can be executed from the CLI. The application takes one (positional) argument, the configuration file.

```bash
cdis-gateway <PATH_TO_FILE>
```

And similarly using `cargo run -- <PATH_TO_FILE>`.

### Configuration
The gateway is configured using a config file. The main points to configure are the network sockets (interface, port, unicast/broadcast/multicast modes)

See `config/sample_config.toml` for details and explanation of the various configuration items.  

### Web UI
The gateway hosts a basic web interface to monitor the functioning of the gateway. The port at which the application is available can be configured in the config file, it defaults to port 8080.

Copyright (C) 2024 Zeeger Lubsen