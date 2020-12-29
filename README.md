# ligature
Ligature is a library for working with knowledge graphs written in Rust.
This project provides the main interfaces used by Ligature as well as some helper functions and constants.
See related projects for implementations of these APIs.
Ligature is heavily influenced by RDF and related standards but attempts to be more general purpose and easier to use.
It attempts to do this by making minor changes while staying mostly compatible.

## Status
This project is still very much under development and subject to change quite a bit in the short term while I'm experimenting.

## RDF's Data Model
| Subject    | Predicate  | Object     | Graph      |
| ---------- | ---------- | ---------- | ---------- |
| iri        | iri        | iri        | iri        |
| blank node |            | blank node | blank node |
|            |            | literal    |            |

## Ligature's Data Model
| Dataset    | Subject    | Predicate  | Object     | Graph      |
| ---------- | ---------- | ---------- | ---------- | ---------- |
| local name | iri        | iri        | iri        | iri        |
|            | blank node | local name | blank node | blank node |
|            | local name |            | literal    | local name |
|            |            |            | local name |            |

### Datasets
A dataset in Ligature is a named collection of statements.
Even though dataset names might seem like they nest (`test/test` looks like it is under `test`) this isn't the case.
A dataset is its own unique entity and stands alone from all other datasets.
Also, datasets are very different from named graphs.
For example with named graphs blank nodes are shared across graphs in a dataset, but in datasets blank nodes are unique to their dataset.

### Local Names
An Local Name is represented by an identifier given by the user that isn't an IRI and local to the dataset.
Local Name identifiers in Ligature are *currently* defined as strings that start with an ASCII letter
or an underscore and don't contain any of the following characters:
 * whitespace (space, newline, tabs, carriage returns, etc)
 * " ' `
 * &lt; &gt;
 * ( )
 * { }
 * \
 * [ ]

If for some reason you need any of these characters in your identifier it is suggested that you use standard URL encoding.
Note that identifiers that start with underscores are reserved for internal use and end users cannot create them themselves.

Identifiers can be something that is meaningful like an IRI/URL, an id from an existing system, or a common name for the domain.
Below is an example statement using identifiers in Scala format.

```scala
tx.addStatement(LocalName("dataset"), Statement(LocalName("Emily"), LocalName("loves"), LocalName("cats")))
```

Besides using named nodes, the `newNode` method returns a unique Anonymous Node with an Identifier
that is automatically generated.
The `newNode` method runs inside a transaction so it is guaranteed to be unique and at the time of creation.
For example here is some pseudocode.

```scala
val ds = LocalName("dataset")
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
| LangLiteral(val value: String, val langTag: String) | Similar to a plain literal in RDF.  A text String and a lang tag. | Yes    |
| StringLiteral(val value: String)                    | A simple string type.                                             | Yes    |
| BooleanLiteral(val value: Boolean)                  | A boolean value.                                                  | No     |
| LongLiteral(val value: Long)                        | A value based on Scala's Long.                                    | Yes    |
| DoubleLiteral(val value: Double)                    | A value based on Scala's Double.                                  | Yes    |

## Building
This project requires SBT to be installed.
On Linux/Mac I recommend using https://sdkman.io/ to manage SBT installs.
Once that is set up use `sbt test` to run tests `sbt publishLocal` to install the artifact locally.

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
| [ligature-schema](https://github.com/almibe/ligature-schema)           | RDFS and SHACL support for Ligature.                                                     |
| [iris](https://github.com/almibe/iris)                                 | IRI support for Scala 3.                                                               |
| [ligature-formats](https://github.com/almibe/ligature-formats)         | Support for various RDF serializations with Ligature.                                  |
| [ligature-sparql](https://github.com/almibe/ligature-sparql)           | SPARQL support for Ligature.                                                           |
