# L I G ∀ T U R Ǝ
Ligature is a library for working with knowledge graphs written in Rust.
Ligature is heavily influenced by RDF and related standards but attempts to be more general, more flexible,
and easier to use.

## Status
This project is still very much under development and subject to change quite a bit in the short term while I'm
experimenting.

## Specification

See [https://github.com/almibe/ligature-specification] to learn more about Ligature.

## Building
This project uses cargo for building.
See https://rustup.rs/ for instructions on installing the Rust toolchain.
See https://doc.rust-lang.org/cargo/ for documentation on cargo in general.

## Related Projects
| Name                                                                   | Description                                                                            |
| ---------------------------------------------------------------------- | -------------------------------------------------------------------------------------- |
| [ligature-server](https://github.com/almibe/ligature-server)           | An HTTP server for working with Ligature.                                              |
| [ligature-client](https://github.com/almibe/ligature-client)           | A JVM HTTP client for working with Ligature.                                           |
| [ligature-lab](https://github.com/almibe/ligature-lab)                 | A multi-user web front-end for experimenting with Ligature via `ligature-server`.      |
| [ligature-lab-desktop](https://github.com/almibe/ligature-lab-desktop) | A single-user desktop front-end for experimenting with Ligature via `ligature-server`. |
| [ligature-repl](https://github.com/almibe/ligature-repl)               | A REPL for working with Ligature.                                                      |
| [ligature-test-suite](https://github.com/almibe/ligature-test-suite)   | A common test suite for Ligature implementations.                                      |
| [ligature-mock](https://github.com/almibe/ligature-mock)               | A painfully simple (yet working) implementation of Ligature.                           |
| [ligature-kv](https://github.com/almibe/ligature-kv)                   | A common library to help with storing Ligature data in a key-value store.              |
| [ligature-lmdb](https://github.com/almibe/ligature-lmdb)               | An implementation of Ligature based on the LMDB key-value store.                       |
| [ligature-sled](https://github.com/almibe/ligature-sled)               | An implementation of Ligature based on the sled key-value store.                       |
| [ligature-benchmark](https://github.com/almibe/ligature-benchmark)     | An internal benchmark for Ligature.                                                    |
| [wander](https://github.com/almibe/wander)                             | A scripting language for working with Ligature.                                        |
| [ligature-schema](https://github.com/almibe/ligature-schema)           | RDFS and SHACL support for Ligature.                                                   |
| [ligature-formats](https://github.com/almibe/ligature-formats)         | Support for various RDF serializations with Ligature.                                  |
| [ligature-sparql](https://github.com/almibe/ligature-sparql)           | SPARQL support for Ligature.                                                           |
| [slonky](https://github.com/almibe/slonky)                             | An implementation of Ligature written in Scala for the JVM.                            |
