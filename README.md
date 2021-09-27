# L I G ∀ T U R Ǝ
Ligature is a library for working with knowledge graphs written in Rust.
Ligature is heavily influenced by RDF and related standards but attempts to be more general, more flexible,
and easier to use.

## Status
This project is still very much under development and subject to change quite a bit in the short term while I'm
experimenting.

## Specification

See https://github.com/almibe/ligature-specification to learn more about Ligature.

## Building
This project uses cargo for building.
See https://rustup.rs/ for instructions on installing the Rust toolchain.
See https://doc.rust-lang.org/cargo/ for documentation on cargo in general.

### Wasm

Parts of this project are intended to be used with either native Rust or Wasm.
To build for Wasm install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/).


## Projects in this Repository

This project uses a monorepo and contains several subprojects.
They are listed below.
The `Core?` column denotes whether a module is able to compiled to web assembly,
so it can be used with node or JVM projects (or anything that runs wasm).

| Name                  | Description                                                         | Core? |
| --------------------- | ------------------------------------------------------------------- | ----- |
| ligature              | Basic structs and traits used by Ligature implementations.          | yes   |
| lig                   | Serialization support for Ligature using the Lig format.            | yes   |
| wander                | The Wander scripting language.                                      | yes   |
| ligature-test-suite   | A test suite for Ligature implementations.                          | yes   |
| ligature-benchmark    | A simple benchmark for Ligature implementations.                    | yes   |
| ligature-in-memory    | An in-memory implementation of Ligature based on the `im` crate.    | yes   |
| ligature-sqlite       | An implementation of Ligature that uses sqlite3 for storage.        | no    |

## Related Projects

Note: work on these projects kind of on hold for now while I'm getting this repo into shape.

| Name                                                                       | Description                                                                       |
| -------------------------------------------------------------------------- | ----------------------------------------------------------------------------------|
| [ligature-js](https://github.com/almibe/ligature-js)                       | A repo with JavaScript code related to Ligature.                                  |
| [ligature-jvm](https://github.com/almibe/ligature-jvm)                     | An implementation of Ligature written in Scala for the JVM.                       |
