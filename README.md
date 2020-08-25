# ligature

Ligature is a library for working with knowledge graphs on the JVM written in Kotlin.
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

| Collection    | Subject           | Predicate     | Object            | Context           |
| ------------- | ----------------- | ------------- | ----------------- | ----------------- |
| named element | named element     | named element | named element     | anonymous element |
|               | anonymous element |               | anonymous element |                   |
|               |                   |               | literal           |                   |

### Entities

Ligature has three types of elements.
A named element is represented by an identifier given by the user
and an anonymous element is represented by a numeric identifier that is automatically generated.
Finally, a literal is one of several types of elements that represents a value of a specific type see below for a list
of current literal types.
Named element identifiers in Ligature are *currently* defined as strings that start with an ASCII letter
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
Below is an example statement using identifiers in Kotlin format.

```kotlin
tx.addStatement(NamedElement("Emily"), NamedElement("loves"), NamedElement("cats"))
```

Besides using named elements, the `newElement` method returns a unique Anonymous Element with an Identifier
that is automatically generated.
The `newElement` method runs inside a transaction so it is guaranteed to be unique and at the time of creation.
For example here is some pseudocode.

```kotlin
collection.write { tx =>
  val e = tx.newElement() // creates a new identifer, in this case let's say `42`
  tx.addStatement(e, a, NamedElement("company")) // should run fine
  tx.addStatement(e, NamedElement("name"), StringLiteral("Pear")) // should run fine
  tx.addStatement(AnonymousElement(newElement.identifer), NamedElement("name"), StringLiteral("Pear")) // will run fine since it's just another way of writing the above line
  tx.addStatement(AnonymousElement(24601), a, NamedElement("bird")) // will erorr out since that identifier hasn't been created yet
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
| LongLiteral(val value: Long) | A value based on Kotlin's Long. | Yes |
| DoubleLiteral(val value: Double) | A value based on Kotlin's Double. | Yes |

### Predicates

Predicates are just NamedElements in the predicate position of the triple.

### Context

Contexts are unique AnonymousElements that are created for every Statement.
They can be accessed from PersistedStatement objects.

## Building
This project requires gradle to be installed.
I recommend using https://sdkman.io/ to manage gradle installs on Linux.
Once that is set up use `gradle test` to run tests `gradle install` to install the artifact locally.

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
