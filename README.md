# ligature
Ligature is a library for working with knowledge graphs on the JVM written in Scala.
This project provides the main interfaces used by Ligature as well as some helper functions and constants.
See related projects for implementations of these APIs.
Ligature is heavily influenced by RDF and related standards but attempts to be more general purpose and easier to use.

## RDF's Data Model
| Subject    | Predicate  | Object     | Graph      |
| ---------- | ---------- | ---------- | ---------- |
| iri        | iri        | iri        | iri        |
| blank node |            | blank node | blank node |
|            |            | literal    |            |

## Ligature's Data Model
| Collection | Subject       | Predicate | Object        | Context       |
| ---------- | ------------- | --------- | ------------- | ------------- |
| NamedNode  | NamedNode     | NamedNode | NamedNode     | AnonymousNode |
|            | AnonymousNode |           | AnonymousNode |               |
|            |               |           | Literal       |               |

### Nodes
Ligature has two types of nodes.
An NamedNode is represented by an identifier given by the user
and an AnonymousNode is represented by a numeric identifier that is automatically generated.
Finally, a literal is one of several types of nodes that represents a value of a specific type see below for a list
of current literal types.
Named node identifiers in Ligature are *currently* defined as strings that start with an ASCII letter
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
tx.addStatement(NamedNode("collection"), Statement(NamedNode("Emily"), NamedNode("loves"), NamedNode("cats")))
```

Besides using named nodes, the `newNode` method returns a unique Anonymous Node with an Identifier
that is automatically generated.
The `newNode` method runs inside a transaction so it is guaranteed to be unique and at the time of creation.
For example here is some pseudocode.

```scala
val col = NamedNode("collection")
collection.write.use { tx =>
  val e: AnonymousNode = tx.newNode() // creates a new identifer, in this case let's say `42`
  tx.addStatement(col, Statement(e, a, NamedNode("company"))) // should run fine
  tx.addStatement(col, Statement(e, NamedNode("name"), StringLiteral("Pear"))) // should run fine
  tx.addStatement(col, Statement(AnonymousNode(newNode.identifer), NamedNode("name"), StringLiteral("Pear"))) // will run fine since it's just another way of writing the above line
  tx.addStatement(col, Statement(AnonymousNode(24601), a, NamedNode("bird"))) // will erorr out since that identifier hasn't been created yet
}
```

### Literals
Literals in Ligature represent an immutable value.
Several types are currently supported with plans to add more.
Below is a table with the currently supported types.

| Name/Signature | Description | Range? |
| -------------- | ----------- | ------ |
| LangLiteral(val value: String, val langTag: String) | Similar to a plain literal in RDF.  A text String and a lang tag. | Yes |
| StringLiteral(val value: String) | A simple string type. | Yes |
| BooleanLiteral(val value: Boolean) | A boolean value. | No |
| LongLiteral(val value: Long) | A value based on Scala's Long. | Yes |
| DoubleLiteral(val value: Double) | A value based on Scala's Double. | Yes |

### Predicates
Predicates are just NamedNodes in the predicate position of the triple.

### Context
Contexts are unique AnonymousNodes that are created for every Statement.
They can be accessed from PersistedStatement objects.

## Building
This project requires SBT to be installed.
On Linux/Mac I recommend using https://sdkman.io/ to manage SBT installs.
Once that is set up use `sbt test` to run tests `sbt publishLocal` to install the artifact locally.

## Related Projects
| Name                                                                 | Description                                                                          |
| -------------------------------------------------------------------- | ------------------------------------------------------------------------------------ |
| [ligature-test-suite](https://github.com/almibe/ligature-test-suite) | A common test suite for Ligature implementations.                                    |
| [ligature-key-value](https://github.com/almibe/ligature-key-value)   | A library for storing Ligature data in a key-value store using the `slonky` library. |
| [ligature-benchmark](https://github.com/almibe/ligature-benchmark)   | An internal benchmark for Ligature.                                                  |
| [wander](https://github.com/almibe/wander)                           | A scripting language for working with Ligature.                                      |
| [ligature-ontology](https://github.com/almibe/ligature-ontology)     | Ontology/OWL support for Ligature.                                                   |
| [ligature-formats](https://github.com/almibe/ligature-formats)       | Support for various RDF serializations with Ligature.                                |
| [ligature-sparql](https://github.com/almibe/ligature-sparql)         | SPARQL support for Ligature.                                                         |

## Ligature-Ex
Ligature-Ex is a version of Ligature that is written in Rust.
It currently isn't very active, but I plan on working on it after the main version Ligature is stable.

| Name                                                                       | Description                                                       |
| -------------------------------------------------------------------------- | ----------------------------------------------------------------- |
| [ligature-ex](https://github.com/almibe/ligature-ex)                       | A Rust implementation of Ligature API.                            |
| [ligature-ex-in-memory](https://github.com/almibe/ligature-ex-in-memory)   | In-memory implementation of the Ligature API in Rust using im.    |
| [ligature-ex-test-suite](https://github.com/almibe/ligature-ex-test-suite) | A common test suite for Ligature-Ex implementations.              |
| [ligature-ex-level](https://github.com/almibe/ligature-ex-level)           | Implementation for Ligature-Ex that uses Level as its data store. |
| [ligature-ex-wander](https://github.com/almibe/ligature-ex-wander)         | Wander support for Ligature-Ex.                                   |
| [ligature-ex-ontology](https://github.com/almibe/ligature-ex-wander)       | Ontology support for Ligature-Ex.                                 |
