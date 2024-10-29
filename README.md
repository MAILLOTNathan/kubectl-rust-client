# kubectl-rust-client

This is a Rust client for Kubernetes.

## Pre-requisites

- Rust
- Cargo
- Kubernetes cluster (only for testing)
- kubectl
- Docker
- GTK3

## Development section

All the development is done in the `src` directory. The `main.rs` file is the entry point of the application.

### Configuration files

The configuration files (for the user interface) are located in the `metadata` directory. The `metadata` directory contains the following directories:
- json: contains the JSON files for the configuration of the user interface.

You must not write paths to the configuration files in the code. Instead, you must modify only the `src/env/config.rs` file, by setting the `CONFIGURATION_PATH` constant to the path of the configuration file you want to use.

Example:
```rust
pub const CONFIGURATION_PATH: &str = "metadata/json/configuration.json";
```
