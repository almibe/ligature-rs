# ligature

Ligature is a library for working with knowledge graphs on the JVM written in Scala.
This project provides the main interfaces used by Ligature as well as some helper functions and constants.
See related projects for implementations of these APIs.

## Ligature's Data Model

| Collection | Source | Edge | Destination | Context        |
| ---------- | ------ | ---- | ----------- | -------------- |
| string     | vertex | edge | vertex      | anonymous node |

### Vertices

A vertex in Ligature can be one of many kinds.
A node is a vertex with a literal as an id.
An anonymous node is a vertex with a generated id.
Currently, anonymous nodes are given a long for an id but `AnonymousNode(42L)` is different from `Node(LongLiteral(42L))`.
A vertex can also simply represent a literal value.
A literal is different than a node with a literal identifier.
For example `StringLiteral("Hello")` is different from `Node(StringLiteral("Hello"))`.

### Edges

An edge connects two vertices together or a single vertex to itself.
An edge simply consists of a label.
Edge labels in Ligature are *currently* defined as strings that start with an ASCII letter
or an underscore and don't contain any of the following characters:
 * whitespace (space, newline, tabs, carriage returns, etc)
 * " ' `
 * &lt; &gt;
 * ( )
 * { }
 * \
 * [ ]
 
If for some reason you need any of these characters in your identifier it is suggested that you use standard URL encoding.
Labels can be something that is meaningful like an IRI/URL, an id from an existing system, or just a name.
Edges with labels that start with `@@` are internal edges and can't be created by users as they aren't valid names.

### Triples

A triple is a set of a source vertex, an edge, and a destination vertex.
Below is an example statement using identifiers in Kotlin format.

```scala
tx.addTriple(Node(StringLiteral("Emily")), Edge("loves"), Node(StringLiteral("cats")))
```

### Contexts

TODO

### Literals

Literals in Ligature represent an immutable value.
Several types are currently supported with plans to add more.
Below is a table with the currently supported types.

| Name/Signature | Description | Range? |
| -------------- | ----------- | ------ |
| LangLiteral(val value: String, val langTag: String) | Similar to a plain literal in RDF.  A text String and a lang tag. | Yes |
| StringLiteral(val value: String) | A simple string type. | Yes |
| BooleanLiteral(val value: Boolean) | A boolean value. | No |
| LongLiteral(val value: Long) | A value based on Kotlin's u64. | Yes |
| DoubleLiteral(val value: Double) | A value based on Kotlin's f64 | Yes |

## Building
This project requires SBT to be installed.
I recommend using https://sdkman.io/ to manage SBT installs.
Once that is set up use `sbt test` to run tests `sbt publishM2` to install the artifact locally.

## Related Projects

| Name | Description | URL |
| ---- | ----------- | --- |
| ligature-keyvalue | A library for storing Ligature data in a key-value store and an in-memory implementation. | https://github.com/almibe/ligature-keyvalue |
| ligature-rocksdb | Implementation of Ligature that uses the RocksDB data store. | https://github.com/almibe/ligature-rocksdb |
| wander | A scripting language for working with Ligature. | https://github.com/almibe/wander |
| ligature-ontology | Ontology/OWL support for Ligature. | https://github.com/almibe/ligature-ontology |
| ligature-test-suite | A common test suite for Ligature implementations. | https://github.com/almibe/ligature-test-suite |
| ligature-foundationdb | Implementation of Ligature for the JVM that uses FoundationDB as its data store. | https://github.com/almibe/ligature-foundationdb |
| ligature-formats | Support for various RDF serializations with Ligature. | https://github.com/almibe/ligature-formats |
| ligature-sparql | SPARQL support for Ligature. | https://github.com/almibe/ligature-sparql |

## Ligature-Ex

Ligature-Ex is a version of Ligature that is written in Rust.
It currently isn't very active, but I plan on working on it after the main version Ligature is stable.

| Name | Description | URL |
| ---- | ----------- | --- |
| ligature-ex | A Rust implementation of Ligature | https://github.com/almibe/ligature-ex |
| ligature-ex-in-memory | In-memory implementation of the Ligature API in Rust using im | https://github.com/almibe/ligature-ex-in-memory |
| ligature-ex-test-suite | A common test suite for Ligature-Ex implementations. | https://github.com/almibe/ligature-ex-test-suite |
| ligature-ex-level | Implementation for Ligature-Ex that uses Level as its data store. | https://github.com/almibe/ligature-ex-level |
| ligature-ex-wander | Wander support for Ligature-Ex. | https://github.com/almibe/ligature-ex-wander |
| ligature-ex-ontology | Ontology support for Ligature-Ex. | https://github.com/almibe/ligature-ex-wander |
