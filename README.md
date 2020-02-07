# ligature
Ligature is a TypeScript library for working with semantic data.
This project provides the main interfaces for Ligature as well as some common helper functions.
It is based on RDF and related standards but has a more flexible approach to working with semantic data.
Its main difference is that it is intended to be used in a broader context than strict RDF.
This means that identifiers do not have to be IRIs and blank nodes only exist in compatibility contexts.

## RDF's Data Model

| Subject    | Predicate  | Object     | Graph      |
| ---------- | ---------- | ---------- | ---------- |
| iri        | iri        | iri        | iri        |
| blank node |            | blank node |            |
|            |            | literal    |            |

## Ligature's Data Model

| Subject    | Predicate  | Object     | Graph      |
| ---------- | ---------- | ---------- | ---------- |
| identifier | identifier | identifier | identifier |
|            |            | literal    |            |

Identifiers in Ligature are *currently* defined as strings that start with an ASCII letter or an underscore and don't contain any of the following characters:
 * whitespace (space, newline, tabs, carriage returns, etc)
 * " ' `
 * < >
 * ( )
 * { }
 * \
 * [ ]

If for some reason you need any of these characters in your identifier it is suggested that you use standard URL encoding.

Identifiers can be something that is meaningful like an IRI/URL, an id from an existing system, a name, or it can be an incrementing id via the `newIdentifier` method.
Below is an example statement using identifiers in n-triples format.

`<Emily> <loves> <cats>`

The `newIdentifier` method returns a unique identifier that probably looks something like this.

`_:34622`

The `newIdentifier` method runs inside a transaction so it is guarenteed to be unique and to not already exist in the Dataset at the time of creatation.
The form `_:NUMBER` is special in Ligature and only IDs that have been already created with the `newIdentifier` method can be used.
For example here is some pseudo code.

```typescript
const x = collection.newIdentifier() // x = _:34
collection.addStatements([x, a, "company"]) // should run fine
collection.addStatements(["_:34", "name", "Pear"]) // should run fine since _:34 has been created already
collection.addStatements([ "_:34567", a, "bird"])
// probably will error out since I doubt that identifer has been created....but it could....but it probably wasn't
```

## Building
Ligature requires npm to be installed.
See https://npmjs.com for installation instructions.
Once that is set up use `npm test` to run tests `npm run build` to build the artifact locally.

## Related Projects

| Name | Description | URL |
| ---- | ----------- | --- |
| ligature-formats | Support for various RDF serializations with Ligature | https://github.com/almibe/ligature-formats |
| wander | A scripting language for working with Ligature. | https://github.com/almibe/wander |
| ligature-sparql | SPARQL support for Ligature. | https://github.com/almibe/ligature-sparql |
| ligature-ontology | Ontology/OWL support for Ligature. | https://github.com/almibe/ligature-ontology |
| ligature-test-suite | A common test suite for Ligature implementations. | https://github.com/almibe/ligature-test-suite |
| ligature-in-memory | In-memory implementation of the Ligature API. | https://github.com/almibe/ligature-in-memory |
| ligature-foundationdb | Implementation of Ligature for Node that uses FoundationDB as its data store. | https://github.com/almibe/ligature-foundationdb |
| ligature-level | Implementation for that uses Level as its data store. | https://github.com/almibe/ligature-level |
