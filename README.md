# Ligature Rust

This project is a set of libraries for working with Ligature knowledge graphs written in Rust.
Ligature is heavily influenced by RDF and related standards but attempts to be simpler and handle different use cases.

## Status

This project is still very much under development and subject to change quite a bit in the short term while I'm
experimenting.
Specifically this project doesn't currently implement all the parts of Ligature.
See [ligature-kt](https://github.com/almibe/ligature-kt) for a more complete implementation written in Kotlin.

## Documentation

See [ligature.dev](https://ligature.dev) or [the documentation repo](https://github.com/almibe/ligature-documentation) to learn more about Ligature.

## Building
This project uses cargo for building.
See https://rustup.rs/ for instructions on installing the Rust toolchain.
See https://doc.rust-lang.org/cargo/ for documentation on cargo in general.

### Wasm

Parts of this project are intended to be used with either native Rust or Wasm.
Currently only ligature-playground produces Wasm output.
To build for Wasm install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/).

## Projects in this Repository

This project uses a monorepo and contains several subprojects.
They are listed below.
The `Core?` column denotes whether a module is intended to be used by both native code and WebAssembly.

| Name                  | Description                                                         | Core? | Output |
| --------------------- | ------------------------------------------------------------------- | ----- | ------ |
| ligature              | Basic structs and traits used by Ligature implementations.          | yes   | lib    |
| lig                   | Serialization support for Ligature using the Lig format.            | yes   | lib    |
| wander                | The Wander scripting language.                                      | yes   | lib    |
| ligature-test-suite   | A test suite for Ligature implementations.                          | yes   | lib    |
| ligature-benchmark    | A simple benchmark for Ligature implementations.                    | yes   | lib    |
| ligature-in-memory    | An in-memory implementation of Ligature based on the `im` crate.    | yes   | lib    |
| ligature-sqlite       | An implementation of Ligature that uses sqlite3 for storage.        | no    | lib    |
| ligature-repl         | A REPL for using Ligature and Wander.                               | no    | app    |
| ligature-playground   | A Wasm artifact for using Ligature in the browser primarily.        | no    | wasm   |
