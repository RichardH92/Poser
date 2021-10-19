# Poser

Poser is a service to track particles in 3d space. It is designed to be used as the persistence layer of an online multiplayer game.

## Features
* In-memory
* Strongly consistent
* Efficent bounds queries
* gRPC API

### Benchmarks

## Architecture


## Project Structure

## API

## Dev Environment
### Required Installs ###
1. [Rust](https://www.rust-lang.org/tools/install)

### Building Poser
```
cargo build
```

### Running Tests
```
cargo test
```

### Running Benchmarks
```
cargo bench
```

### Recommended Tools
| Tool | Description |
| -----| ------------|
| [BloomRPC](https://github.com/uw-labs/bloomrpc) | gRPC client |

