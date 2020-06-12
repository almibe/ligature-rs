# ligature

Ligature is a Knowledge Base library for the JVM written in Kotlin.
This project provides the main interfaces used by Ligature as well as some helper functions and constants.
See relate projects for implementations of these APIs.
Ligature is heavily influenced by RDF and related standards but attempts to be more general purpose and easier to use.

## RDF's Data Model

| Subject    | Predicate  | Object     | Graph?     |
| ---------- | ---------- | ---------- | ---------- |
| iri        | iri        | iri        | iri        |
| blank node |            | blank node | blank node |
|            |            | literal    |            |

## Ligature's Data Model

| Subject          | Predicate    | Object           | Context       |
| ---------------- | ------------ | ---------------- | ------------- |
| named entity     | named entity | named entity     | named entity  |
| anonymous entity |              | anonymous entity |               |
|                  |              | literal          |               |
#### Entities

Ligature has two types of entities.
A named entity is represented by an identifier given by the user and an anonymous entity is represented by a numeric identifier that is automatically generated.
Named entity identifiers in Ligature are *currently* defined as strings that start with an ASCII letter or an underscore and don't contain any of the following characters:
 * whitespace (space, newline, tabs, carriage returns, etc)
 * " ' `
 * &lt; &gt;
 * ( )
 * { }
 * \
 * [ ]

If for some reason you need any of these characters in your identifier it is suggested that you use standard URL encoding.
Note that identifiers with underscores are reserved for internal use and end users cannot create them themselves.

Identifiers can be something that is meaningful like an IRI/URL, an id from an existing system, a name, or it can be an incrementing id via the `newEntity` method.
Below is an example statement using identifiers in Kotlin format.

`tx.addStatement(Entity("Emily"), Entity("loves"), Entity("cats"), default)`

The `default` argument passed is imported as a value from `dev.ligature.default`.
It is equal to `Entity("_")` and represents the default graph in Ligature.

Besides using named entities, the `newEntity` method returns a unique Anonymous Entity with an Identifier that is automatically generated.
The `newEntity` method runs inside a transaction so it is guaranteed to be unique and to not already exist in the Dataset at the time of creation.
For example here is some pseudo code.

```kotlin
val tx = collection.writeTx()
val newEntity = tx.newEntity() // creates a new identifer, in this case let's say `42`
tx.addStatement(x, a, NamedEntity("company")) // should run fine
tx.addStatement(newEntity, NamedEntity("name"), StringLiteral("Pear")) // should run fine
tx.addStatement(AnonymousEntity(newEntity.identifer), NamedEntity("name"), StringLiteral("Pear")) // will run fine since it's just another way of writing the above line
tx.addStatement(AnonyMousEntity(24601), a, NamedEntity("bird")) // will erorr out since that identifier hasn't been created yet
```

#### Literals

Literals in Ligature represent an immutable value.
Several types are currently supported with plans to add more.
Below is a table with the currently supported types.

| Name/Signature | Description | Range? | Collection? |
| -------------- | ----------- | ------ | ----------- |
| LangLiteral(val value: String, val langTag: String) | Similar to a plain literal in RDF.  A text String and a lang tag. | Yes | No |
| StringLiteral(val value: String) | A simple string type. | Yes | No |
| BooleanLiteral(val value: Boolean) | A boolean value. | No | No |
| LongLiteral(val value: Long) | A value based on Kotlin's u64. | Yes | No |
| DoubleLiteral(val value: Double) | A value based on Kotlin's f64 | Yes | No |

#### Predicates

Predicates are very similar to Entities in that they represented by a single Identifier, but they are only used in the Predicate position of a Statement or Rule.

## Building
Ligature requires Gradle to be installed.
See https://gradle.org for installation instructions.
Once that is set up use `gradle test` to run tests `gradle install` to install the artifact locally.

## Related Projects

| Name | Description | URL |
| ---- | ----------- | --- |
| ligature-in-memory | In-memory implementation of the Ligature API in Kotlin using Vavr | https://github.com/almibe/ligature-in-memory |
| ligature-rocksdb | Implementation of Ligature that uses the RocksDB data store. | https://github.com/almibe/ligature-rocksdb |
| wander | A scripting language for working with Ligature. | https://github.com/almibe/wander |
| ligature-ontology | Ontology/OWL support for Ligature. | https://github.com/almibe/ligature-ontology |
| ligature-test-suite | A common test suite for Ligature implementations. | https://github.com/almibe/ligature-test-suite |
| ligature-foundationdb | Implementation of Ligature for the JVM that uses FoundationDB as its data store. | https://github.com/almibe/ligature-foundationdb |
| ligature-formats | Support for various RDF serializations with Ligature | https://github.com/almibe/ligature-formats |
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
