# L I G ∀ T U R Ǝ
Ligature is a library for working with knowledge graphs written in Rust.
This project provides the main traits and structs used by Ligature as well as some helper functions.
See related projects for implementations of these APIs.
Ligature is heavily influenced by RDF and related standards but attempts to be more general and easier to use.

## Status
This project is still very much under development and subject to change quite a bit in the short term while I'm experimenting.

## Ligature's Data Model
```
Dataset { dataset_name: DatasetName, links: Link* }
DatasetName { name: String }
Link { source: Vertex, arrow: Arrow, target: Vertex, context: Node }
Vertx { 
    Node { id: u64 } |
    String { value: String } |
    Integer { value: i64 } |
    /* TODO list all literal types */ 
}
Arrow { name: String }
```

### Datasets
A dataset in Ligature is a named collection of statements.
Even though dataset names might seem like they nest (`test/test` looks like it is under `test`) this isn't the case.
A dataset is its own unique entity and stands alone from all other datasets.
Also, datasets are very different from named graphs in RDF.
For example with named graphs blank nodes are shared across graphs in a dataset, but in datasets blank nodes are unique to their dataset.
`TODO explain dataset names`

### Vertices
Vertices in Ligature can either be a Node or a Literal.

### Nodes
Nodes in Ligature are simply an object that we can make statements about.
Every Node is defined by an unique id.
The `newNode` method returns a new Node that is automatically generated.
The `newNode` method runs inside a transaction so it is guaranteed to be unique and at the time of creation.

### Literals
Literals in Ligature represent an immutable value.
Several types are currently supported with plans to add more.
Below is a table with the currently supported types.

| Name/Signature                                      | Description                                                       | Range? |
| --------------------------------------------------- | ----------------------------------------------------------------- | ------ |
| LangLiteral { value: String, langTag: String }      | Similar to a plain literal in RDF.  A text String and a lang tag. | Yes    |
| StringLiteral(String)                               | A simple string type.                                             | Yes    |
| BooleanLiteral(bool)                                | A boolean value.                                                  | No     |
| LongLiteral(i64)                                    | A value based on Rust's i64.                                      | Yes    |
| DoubleLiteral(f64)                                  | A value based on Rust's f64.                                      | Yes    |

### Arrows
Arrows in Ligature are a label that links two Nodes together.

### Links
A Link in Ligature is a sum type of { source: Node, arrow: Arrow, target: Node, context: Node }.

### Contexts
Contexts in Ligature are an unique Idenifier that represents a given Link.

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
| [ligature-test-suite](https://github.com/almibe/ligature-test-suite)   | A common test suite for Ligature implementations.                                      |
| [ligature-mock](https://github.com/almibe/ligature-mock)               | A painfully simple (yet working) implementation of Ligature.                           |
| [ligature-sled](https://github.com/almibe/ligature-sled)               | An implementation of Ligature based on the sled key-value store.                       |
| [ligature-key-value](https://github.com/almibe/ligature-key-value)     | A library for storing Ligature data in a key-value store using the `slonky` library.   |
| [ligature-benchmark](https://github.com/almibe/ligature-benchmark)     | An internal benchmark for Ligature.                                                    |
| [wander](https://github.com/almibe/wander)                             | A scripting language for working with Ligature.                                        |
| [ligature-schema](https://github.com/almibe/ligature-schema)           | RDFS and SHACL support for Ligature.                                                   |
| [iris](https://github.com/almibe/iris)                                 | IRI support for Scala 3.                                                               |
| [ligature-formats](https://github.com/almibe/ligature-formats)         | Support for various RDF serializations with Ligature.                                  |
| [ligature-sparql](https://github.com/almibe/ligature-sparql)           | SPARQL support for Ligature.                                                           |
