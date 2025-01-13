# Diagnostics over Internet Protocol (DoIP) Sockets

A Diagnostics over Internet Protocol (DoIP) implementation for TCP & UDP Sockets with helper functions.

## Features

- **DoIP Message Framing**: Implements framing mechanisms for reliable transmission and reception of DoIP messages.
- **TCP and UDP Support**: Provides seamless support for both transport protocols.
- **Asynchronous Design**: Built on [Tokio](https://tokio.rs/) for high-performance and non-blocking communication.
- **Error Handling**: Robust error handling for socket operations and DoIP message processing.
- **Extensible and Modular**: Designed to integrate with higher-level diagnostic tools or frameworks.

## Getting Started

### Prerequisites

- Rust programming language (latest stable version). Install Rust from [rust-lang.org](https://www.rust-lang.org/).
- Windows 10 or other compatible operating systems.

### Installation

Add the library as a dependency in your `Cargo.toml` file:

```toml
[dependencies]
doip-sockets = "0.2.0"
```

### Documentation

Comprehensive documentation is available [here](https://crates.io/crates/doip-sockets) (link to hosted docs).

## Development

### Building the Project

Clone the repository and build the project using Cargo:

```sh
git clone https://github.com/samp-reston/doip-sockets.git
cd doip-sockets
cargo build
```

### Running Tests

Run unit tests to ensure functionality:

```sh
cargo test
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Based on the ISO 13400 DoIP specification.
- Built with the Rust programming language.
- Thanks to the Tokio project for enabling high-performance asynchronous networking.

## Contact

For support, questions, or feature requests, please open an issue on the [GitHub repository](https://github.com/samp-reston/doip-sockets).
