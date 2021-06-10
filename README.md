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

## Projects in this Repository
| Name                  | Description                                                                            |
| --------------------- | -------------------------------------------------------------------------------------- |
| ligature              | Basic structs and traits used by Ligature implementations.                             |
| lig                   | Serialization support for Ligature using the Lig format.                               |
| ligature-test-suite   | A test suite for Ligature implementations.                                             |
| ligature-benchmark    | A simple benchmark for Ligature implementations.                                       |
| ligature-in-memory    | An in-memory implementation of Ligature based on the `im` crate.                       |
| ligature-sqlite       | An implementation of Ligature that uses sqlite3 for storage.                           |

## Related Projects
| Name                                                                       | Description                                                                       |
| -------------------------------------------------------------------------- | ----------------------------------------------------------------------------------|
| [ligature-specification](https://github.com/almibe/ligature-specification) | Ligature's specification.                                                         |
| [ligature-lab](https://github.com/almibe/ligature-lab)                     | A multi-user web front-end for experimenting with Ligature via `ligature-server`. |
| [ligature-jvm](https://github.com/almibe/ligature-jvm)                     | An implementation of Ligature written in Scala for the JVM.                       |
