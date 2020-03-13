# ligature

Ligature is a Kotlin library for working with semantic networks.

## RDF's Data Model

| Subject    | Predicate  | Object     | Graph?     |
| ---------- | ---------- | ---------- | ---------- |
| iri        | iri        | iri        | iri        |
| blank node |            | blank node | blank node |
|            |            | literal    |            |

## Ligature's Data Model

| Subject | Predicate  | Object | Graph  |
| ------- | ---------- | ------ | ------ |
| node    | entity     | node   | entity |

### Nodes

Nodes in Ligature can be of two main types, either an entity or a literal.

#### Entities

An Entity is a Node that is represented by and Identifier.
Identifiers in Ligature are *currently* defined as strings that start with an ASCII letter or an underscore and don't contain any of the following characters:
 * whitespace (space, newline, tabs, carriage returns, etc)
 * " ' `
 * < >
 * ( )
 * { }
 * \
 * [ ]

If for some reason you need any of these characters in your identifier it is suggested that you use standard URL encoding.

Identifiers can be something that is meaningful like an IRI/URL, an id from an existing system, a name, or it can be an incrementing id via the `newEntity` method.
Below is an example statement using identifiers in Kotlin format.

`collection.addStatement(Entity("Emily"), Entity("loves"), Entity("cats"), Entity("_"))`

The `newEntity` method returns a unique entity with an identifier that looks something like this.

`_:34622`

The `newEntity` method runs inside a transaction so it is guaranteed to be unique and to not already exist in the Dataset at the time of creation.
The form `_:NUMBER` is special in Ligature and only IDs that have been already created with the `newEntity` method can be used.
For example here is some pseudo code.

```kotlin
//running in a WriteTx
val newEntity = collection.newEntity() // creates a new identifer, in this case let's say `_:34`
collection.addStatement(x, a, Entity("company"), Entity("_")) // should run fine
collection.addStatement(Entity("_:34"), Entity("name"), StringLiteral("Pear"), Entity("_")) // should run fine since _:34 has been created already
collection.addStatement(Entity("_:34567"), a, Entity("bird"), Entity("_")) // will erorr out since that identifier hasn't been created yet
```

#### Literals

TODO

### Attributes

TODO

## Building
Ligature requires Gradle to be installed.
See https://gradle.org for installation instructions.
Once that is set up use `gradle test` to run tests `gradle install` to install the artifact locally.

## Related Projects

| Name | Description | URL |
| ---- | ----------- | --- |
| ligature-formats | Support for various RDF serializations with Ligature | https://github.com/almibe/ligature-formats |
| wander | A scripting language for working with Ligature. | https://github.com/almibe/wander |
| ligature-sparql | SPARQL support for Ligature. | https://github.com/almibe/ligature-sparql |
| ligature-ontology | Ontology/OWL support for Ligature. | https://github.com/almibe/ligature-ontology |
| ligature-test-suite | A common test suite for Ligature implementations. | https://github.com/almibe/ligature-test-suite |
| ligature-in-memory | In-memory implementation of the Ligature API in Kotlin | https://github.com/almibe/ligature-in-memory |
| ligature-xodus | Implementation of Ligature for the JVM that uses the Xodus data store. | https://github.com/almibe/ligature-xodus |
| ligature-foundationdb | Implementation of Ligature for the JVM that uses FoundationDB as its data store. | https://github.com/almibe/ligature-foundationdb |
| ligature-indexdb | Implementation for Kotlin.js that uses IndexDB as its data store. | https://github.com/almibe/ligature-indexdb |
