# L I G ∀ T U R Ǝ
Ligature is a library for working with knowledge graphs written in Rust.
This project provides the main traits and structs used by Ligature as well as some helper functions.
See related projects for implementations of these APIs.
Ligature is heavily influenced by RDF and related standards but attempts to be more general, more flexible, and easier to use.

## Status
This project is still very much under development and subject to change quite a bit in the short term while I'm experimenting.

## Ligature's Data Model
```
Dataset { dataset_name: DatasetName, statements: Statement* }
DatasetName { name: String }
Entity { id: u64 }
Attribute { name: String }
Value {
    Entity { value: Entity } |
    StringLiteral { value: String } |
    IntegerLiteral { value: i64 } |
    FloatLiteral { value: f64 } |
}
Statement { entity: Entity, attribute: Attribute, value: Value }
PersistedStatement { statement: Statement, context: Entity }
```

### Datasets
A dataset in Ligature is a named collection of statements.
Valid dataset names are currently groups of characters that include `_ a-z A-Z 0-9` that can't start with a number and that are separated by single `/`.
This naming convention is likely to change.
Even though dataset names might seem like they nest (`test/test` looks like it is under `test`) this isn't the case.
A dataset is its own unique entity and stands alone from all other datasets.
Also, datasets are very different from named graphs in RDF.
For example with named graphs blank nodes are shared across graphs in a dataset, but in datasets blank nodes are unique to their dataset.

### Entities
Entities in Ligature are simply an object that we can make statements about.
Every Entity is defined by an unique id.
The `new_entity` method returns a new Entity that is automatically generated.
The `new_entity` method runs inside a transaction so it is guaranteed to be unique and at the time of creation.

### Attributes
Attributes in Ligature are a label that relates two Entities together.

### Values
Values in Ligature can either be an Entity or a Literal.

### Literals
Literals in Ligature represent an immutable value.
Several types are currently supported with plans to add more.
Below is a table with the currently supported types.

| Name/Signature        | Description                  | Range? |
| --------------------- | -----------------------------| ------ |
| StringLiteral(String) | A simple string type.        | Yes    |
| IntegerLiteral(i64)   | A value based on Rust's i64. | Yes    |
| FloatLiteral(f64)     | A value based on Rust's f64. | Yes    |

### Statements
A Statement in Ligature is a product type of { entity: Entity, attribute: Attribute, value: Value, context: Entity }.

### Contexts
Contexts in Ligature are an unique Entity that represents a given Statement.
They allow you to make Statements about Statements.

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
