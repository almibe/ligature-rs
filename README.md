# L I G ∀ T U R Ǝ
Ligature is a library for working with knowledge graphs written in Rust.
This project provides the main interfaces used by Ligature as well as some helper functions and constants.
See related projects for implementations of these APIs.
Ligature is heavily influenced by RDF and related standards but attempts to be more general purpose and easier to use.
It attempts to do this by making minor changes while staying mostly compatible.

## Status
This project is still very much under development and subject to change quite a bit in the short term while I'm experimenting.

## Ligature's Data Model (a.k.a. RDF's Data Model)
| Dataset | Subject    | Predicate  | Object     | Graph      |
| ------- | ---------- | ---------- | ---------- | ---------- |
| dataset | iri        | iri        | iri        | iri        |
|         | blank node |            | blank node | blank node |
|         |            |            | literal    |            |

### Datasets
A dataset in Ligature is a named collection of statements.
Even though dataset names might seem like they nest (`test/test` looks like it is under `test`) this isn't the case.
A dataset is its own unique entity and stands alone from all other datasets.
Also, datasets are very different from named graphs.
For example with named graphs blank nodes are shared across graphs in a dataset, but in datasets blank nodes are unique to their dataset.
`TODO explain dataset names`

### IRI

`TODO`

### Blank Nodes
Besides using IRIs, the `newNode` method returns a unique Anonymous Node with an Identifier
that is automatically generated.
The `newNode` method runs inside a transaction so it is guaranteed to be unique and at the time of creation.
For example here is some pseudocode.

```scala
val ds = Dataset("dataset")
instance.write.use { tx =>
  val e: AnonymousNode = tx.newNode(ds) // creates a new identifer, in this case let's say `42`
  tx.addStatement(ds, Statement(e, a, LocalName("company"))) // should run fine
  tx.addStatement(ds, Statement(e, LocalName("name"), StringLiteral("Pear"))) // should run fine
  tx.addStatement(ds, Statement(AnonymousNode(newNode.identifer), LocalName("name"), StringLiteral("Pear"))) // will run fine since it's just another way of writing the above line
  tx.addStatement(ds, Statement(AnonymousNode(24601), a, LocalName("bird"))) // will erorr out since that identifier hasn't been created yet
}
```

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

## Building

`TODO update for cargo`

## Related Projects
| Name                                                                   | Description                                                                            |
| ---------------------------------------------------------------------- | -------------------------------------------------------------------------------------- |
| [ligature-server](https://github.com/almibe/ligature-server)           | An HTTP server for working with Ligature.                                              |
| [ligature-client](https://github.com/almibe/ligature-client)           | A JVM HTTP client for working with Ligature.                                           |
| [ligature-lab](https://github.com/almibe/ligature-lab)                 | A multi-user web front-end for experimenting with Ligature via `ligature-server`.      |
| [ligature-lab-desktop](https://github.com/almibe/ligature-lab-desktop) | A single-user desktop front-end for experimenting with Ligature via `ligature-server`. |
| [ligature-test-suite](https://github.com/almibe/ligature-test-suite)   | A common test suite for Ligature implementations.                                      |
| [ligature-mock](https://github.com/almibe/ligature-mock)               | A painfully simple (yet working) implementation of Ligature.                           |
| [ligature-xodus](https://github.com/almibe/ligature-xodus)             | An implementation of Ligature based on Xodus's EntityStore api.                        |
| [ligature-key-value](https://github.com/almibe/ligature-key-value)     | A library for storing Ligature data in a key-value store using the `slonky` library.   |
| [ligature-benchmark](https://github.com/almibe/ligature-benchmark)     | An internal benchmark for Ligature.                                                    |
| [wander](https://github.com/almibe/wander)                             | A scripting language for working with Ligature.                                        |
| [ligature-schema](https://github.com/almibe/ligature-schema)           | RDFS and SHACL support for Ligature.                                                   |
| [iris](https://github.com/almibe/iris)                                 | IRI support for Scala 3.                                                               |
| [ligature-formats](https://github.com/almibe/ligature-formats)         | Support for various RDF serializations with Ligature.                                  |
| [ligature-sparql](https://github.com/almibe/ligature-sparql)           | SPARQL support for Ligature.                                                           |
