# Bioreactor Hub

This project provides a reference implementation of the bioreactor hub. This includes the server side of the `hub-protocol` REST API for managing bioreactor devices, and the server side of the `device-protocol` low-level API for communicating with individual bioreactors.  

> The low-level protocol is not yet implemented.

The project is using Rust. To compile it, install [Rust](https://rust-lang.org) and execute:

```bash
cargo run
```

This should start a HTTP server using a test configuration defined in `hub.json`. To log in, use password `let me in`. To use a custom configuration file, use `cargo run -- path/to/config.json`.

### Configuration

Currently, the configuration file supports:
 - `name`: A human readable name of this hub server.
 - `description`: An optional description with more details about the hub.
 - `server_password`: At least 16 character long password used for token generation.
 - `user_password`: A list of user passwords that can be used for logging into the hub.

In the future, additional configuration values will be available (e.g. `port`), including configuration of test devices (virtual bioreactors) and configuration of actual low-level connections to physical devices.