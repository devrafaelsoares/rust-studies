<div align="center">

# 🦀 Rust Studies

**A hands-on journey through the Rust programming language — from fundamentals to advanced systems programming.**

[![Rust](https://img.shields.io/badge/Rust-1.85+-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-blue?style=for-the-badge)](LICENSE)
[![Cargo Workspace](https://img.shields.io/badge/Cargo-Workspace-orange?style=for-the-badge&logo=rust)](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html)
[![Docker](https://img.shields.io/badge/Docker-Ready-2496ED?style=for-the-badge&logo=docker&logoColor=white)](docker-compose.yaml)

</div>

---

## 📖 About

This repository is a **Cargo Workspace** containing multiple crates, each focused on a different Rust concept. It serves as a structured learning path and reference for core Rust topics — from ownership and borrowing to async gRPC services.

## 🏗️ Project Structure

```
rust-studies/
├── Cargo.toml              # Workspace root
├── Dockerfile              # Dev container setup
├── docker-compose.yaml     # Docker orchestration
│
├── hello_cargo/            # 🚀 Getting started with Cargo
│   └── src/main.rs
│
├── primitive-types/        # 🔢 Scalar types, arrays & tuples
│   └── src/main.rs
│
├── rust-core/              # 🧠 Ownership, borrowing & cloning
│   └── src/main.rs
│
├── rust-loop/              # 🔁 Control flow: for, while & loop
│   └── src/main.rs
│
└── grpc-chat/              # 🌐 gRPC with Tonic (client & server)
    ├── proto/
    ├── data/
    └── src/
        ├── server/server.rs
        └── client/client.rs
```

## 📚 Modules

| Crate | Topic | Key Concepts |
|---|---|---|
| **hello_cargo** | Getting Started | Project setup, `println!` macro, Cargo basics |
| **primitive-types** | Data Types | Arrays, tuples, `std::mem`, stack allocation |
| **rust-core** | Ownership & Borrowing | `String`, references (`&`), `.clone()`, move semantics |
| **rust-loop** | Control Flow | `for` ranges, `while` conditions, infinite `loop` with `break` |
| **grpc-chat** | Async & Networking | Tonic, Protobuf, async/await, client-server architecture |

## 🚀 Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (1.85+)
- [Docker](https://docs.docker.com/get-docker/) *(optional, for containerized dev)*

### Running a specific crate

```bash
# Build the entire workspace
cargo build

# Run a specific crate
cargo run -p hello_cargo
cargo run -p primitive-types
cargo run -p rust-core
cargo run -p rust-loop

# Run gRPC server
cargo run -p grpc-rust --bin routeguide-server

# Run gRPC client (in another terminal)
cargo run -p grpc-rust --bin routeguide-client
```

### Using Docker

```bash
docker compose build
docker compose run --rm rust-dev-service bash
# Inside the container:
cargo run -p hello_cargo
```

### Running tests

```bash
# Run all tests across the workspace
cargo test

# Run tests for a specific crate
cargo test -p rust-core
```

## 🐳 Docker Setup

The project includes a `Dockerfile` and `docker-compose.yaml` for a consistent development environment:

- **Base image:** `rust:1.95.0-slim`
- **Tools included:** `nano`, `vim`, `curl`
- **Volume mount:** Project root → `/app`

## 📄 License

This project is licensed under the MIT License — see the [LICENSE](LICENSE) file for details.

---

<div align="center">

**Built with 🦀 by [Rafael Soares](https://github.com/devrafaelsoares)**

</div>
