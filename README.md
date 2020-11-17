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
| Dataset | Subject | Predicate | Object  | Context |
| ------- | ------- | --------- | --------| ------- |
| Dataset | Subject | Predicate | Subject | Subject |
|         |         |           | Literal |         |

### Datasets
A dataset in Ligature is a named collection of statements.
A dataset's name must be a valid URL path (not a full URL just the path part).
Currently, naming is even more restrictive and must contain only lower case ASCII characters, underscores, and forward-slashes.
This is likely to change to be more flexible but seems like a good starting point.

It's important to note that currently, Ligature doesn't support named graphs like quad-stores support, and datasets are very different from named graphs.
Even though dataset names might seem like they nest (`test/test` looks like it is under `test`) this isn't the case.
A dataset is its own unique entity and stands alone from all other datasets.
For example with named graphs blank nodes are shared across graphs in a dataset, but in Ligature Subjects are unique to their dataset.
If/When Ligature supports named graphs within datasets it will be the case that Subjects are shared across named graphs in a single dataset.

### Subjects
Subjects in Ligature are similar to blank nodes in RDF.
Ligature doesn't support IRIs as first class entities like RDF does.
Instead, Subjects are assigned a unique id by the system when they are created (currently ids are represented by Longs but they might be UUIDs in the future).

### Predicates
Unlike RDF Predicates in Ligature are their own entity and not just an IRI.
A Predicate has a user defied identifier in Ligature that is *currently* defined as strings that start with an ASCII letter
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

### Context
Contexts are unique Subjects that are created for every Statement.
They can be accessed from PersistedStatement objects.

### Example

Below is an example statement using identifiers in Scala format.
Notice that the `newSubject` method returns a unique Subject that is automatically generated.
The `newSubject` method runs inside a transaction, so it is guaranteed to be unique and at the time of creation.

```scala
val ds = Dataset("dataset")
instance.write.use { tx => //start a write transaction assuming we already have a LigatureInstance object
  tx.createDataset(ds) //make a new Dataset
  val subject: Subject = tx.newSubject(ds) //create a new Subject in the given Dataset
  tx.addStatement(ds, Statement(e, Predicate("name"), StringLiteral("Lycus")))
  //if you know an id you can directly instantiate a Subject, but you usually won't so this example is silly
  tx.addStatement(ds, Statement(Subject(newNode.identifer), Predicate("favoriteFruit"), StringLiteral("pear")))
}
```

## Building
This project requires SBT to be installed.
On Linux/Mac I recommend using https://sdkman.io/ to manage SBT installs.
Once that is set up use `sbt test` to run tests `sbt publishLocal` to install the artifact locally.

## Related Projects
| Name                                                                   | Description                                                                            |
| ---------------------------------------------------------------------- | -------------------------------------------------------------------------------------- |
| [ligature-server](https://github.com/almibe/ligature-server)           | An HTTP server for working with Ligature.                                              |
| [ligature-lab](https://github.com/almibe/ligature-lab)                 | A multi-user web front-end for experimenting with Ligature via `ligature-server`.      |
| [ligature-lab-desktop](https://github.com/almibe/ligature-lab-desktop) | A single-user desktop front-end for experimenting with Ligature via `ligature-server`. |
| [ligature-test-suite](https://github.com/almibe/ligature-test-suite)   | A common test suite for Ligature implementations.                                      |
| [ligature-mock](https://github.com/almibe/ligature-mock)               | A painfully simple (yet working) implementation of Ligature.                           |
| [ligature-xodus](https://github.com/almibe/ligature-xodus)             | An implementation of Ligature based on Xodus's EntityStore api.                        |
| [ligature-key-value](https://github.com/almibe/ligature-key-value)     | A library for storing Ligature data in a key-value store using the `slonky` library.   |
| [ligature-benchmark](https://github.com/almibe/ligature-benchmark)     | An internal benchmark for Ligature.                                                    |
| [wander](https://github.com/almibe/wander)                             | A scripting language for working with Ligature.                                        |
| [ligature-ontology](https://github.com/almibe/ligature-ontology)       | Ontology/OWL support for Ligature.                                                     |
| [ligature-formats](https://github.com/almibe/ligature-formats)         | Support for various RDF serializations with Ligature.                                  |
| [ligature-sparql](https://github.com/almibe/ligature-sparql)           | SPARQL support for Ligature.                                                           |
