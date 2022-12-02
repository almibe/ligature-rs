# ligature-rs

This project is a set of libraries and applications for working with Ligature knowledge graphs written in Rust.
Ligature is heavily influenced by RDF and related standards, but attempts to be simpler and handle different use cases.
See [ligature.dev](https://ligature.dev) to learn more about Ligature.

## Status

This project is still very much under development and subject to change quite a bit in the short term while I'm
experimenting.
Specifically this project doesn't currently implement all the parts of Ligature.
See [ligature-kt](https://github.com/almibe/ligature-kt) for a more complete implementation written in Kotlin.

## Building
This project uses cargo for building.
See https://rustup.rs/ for instructions on installing the Rust toolchain.
See https://doc.rust-lang.org/cargo/ for documentation on cargo in general.

### Grain

This project contains a folder `grain` that contains Grain code for working with Ligature.
This integration is still experimental and details will be added as things are worked out.
See https://grain-lang.org for information on Grain.

## Rust Projects in this Repository

This project uses a monorepo and contains several subprojects.
Each project contains its own README with additional information when needed.

| Name                  | Description                                                         | Output |
| --------------------- | ------------------------------------------------------------------- | ------ |
| ligature              | Basic structs and traits used by Ligature implementations.          | lib    |
| lig                   | Serialization support for Ligature using the Lig format.            | lib    |
| wander                | The Wander scripting language.                                      | lib    |
| ligature-test-suite   | A test suite for Ligature implementations.                          | lib    |
| ligature-benchmark    | A simple benchmark for Ligature implementations.                    | lib    |
| ligature-in-memory    | An in-memory implementation of Ligature.                            | lib    |
| ligature-sqlite       | An implementation of Ligature that uses sqlite3 for storage.        | lib    |
| ligature-repl         | A REPL for using Ligature and Wander.                               | app    |
