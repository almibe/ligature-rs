# ligature-rs

This project is a set of libraries and applications for working with Ligature knowledge graphs written in Rust.
Ligature is heavily influenced by RDF and related standards, but attempts to be simpler and handle different use cases.
See [ligature.dev](https://ligature.dev) to learn more about Ligature.

## Status

This project is still very much under development and subject to change quite a bit in the short term while I'm
experimenting.

## Building
This project uses cargo for building.
See https://rustup.rs/ for instructions on installing the Rust toolchain.
See https://doc.rust-lang.org/cargo/ for documentation on cargo in general.

## Rust Projects in this Repository

This repo contains several projects.
Each project contains its own README with additional information when needed.

| Name                  | Description                                                             | Output |
| --------------------- | ----------------------------------------------------------------------- | ------ |
| lig                   | Serialization support for Ligature using the Lig format.                | lib    |
| ligature              | Basic structs and traits used by Ligature implementations.              | lib    |
| ligature-benchmark    | A simple benchmark for Ligature implementations.                        | app    |
| ligature-http         | An HTTP server for working with Ligature and Wander.                    | app    |
| ligature-in-memory    | An in-memory implementation of Ligature.                                | lib    |
| ligature-redb         | An implementation of Ligature that uses redb for storage.               | lib    |
| ligature-repl         | A REPL for using Ligature and Wander.                                   | app    |
| ligature-sqlite       | An implementation of Ligature that uses sqlite3 for storage.            | lib    |
| ligature-test-suite   | A test suite for Ligature implementations.                              | app    |
| ligature-wasm         | Compile Ligature for Wasm, provides in memory store and Wander support. | wasm   |
| wander                | The Wander scripting language.                                          | lib    |

## Running Integration Tests

Besides regular Rust tests, this project contains a testing application called `ligature-test-suite`.
This project can be ran like any other normal Rust application.

```
cd ligature-test-suite
cargo run
```

## Ligature HTTP

ligature-http is an application that exposes an HTTP endpoint to run Wander code.
It currently exists to aid with development and shouldn't be used for anything else.
