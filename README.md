# ligature
Ligature is a Clojure library for working with semantic data.
This project provides the main protocols for Ligature as well as Spec support and some common helper functions.
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

Identifiers can be something that is meaningful like an IRI/URL, an id from an existing system, a name, or it can be an incrementing id via the `new-identifier` method.
Below is an example statement using identifiers in n-triples format.

`<Emily> <loves> <cats>`

The `new-identifier` method returns a unique identifier that probably looks something like this.

`_:34622`

The `new-identifier` method runs inside a transaction so it is guarenteed to be unique and to not already exist in the Dataset at the time of creatation.
The form `_:NUMBER` is special in Ligature and only IDs that have been already created with the `new-identifier` method can be used.
For example here is some pseudo code.

```clojure
(def x (new-identifier collection))  ; x = _:34
(add-statements collection [[ x :a "company"]]) ; should run fine
(add-statements collection [[ "_:34" "name" "Pear"]]) ; should run fine since _:34 has been created already
(add-statements collection [[ "_:34567" :a "bird"]]) ; probably will error out since I doubt that identifer has been created....but it could....but it probably wasn't
```

## Building
Ligature requires Leiningen to be installed.
See https://leiningen.org for installation instructions.
Once that is set up use `lein test` to run tests `lein install` to install the artifact locally.

## Related Projects

| Name | Description | URL |
| ---- | ----------- | --- |
| ligature-formats | Support for various RDF serializations with Ligature | https://github.com/almibe/ligature-formats |
| wander | A scripting language for working with Ligature. | https://github.com/almibe/wander |
| ligature-sparql | SPARQL support for Ligature. | https://github.com/almibe/ligature-sparql |
| ligature-ontology | Ontology/OWL support for Ligature. | https://github.com/almibe/ligature-ontology |
| ligature-test-suite | A common test suite for Ligature implementations. | https://github.com/almibe/ligature-test-suite |
| ligature-in-memory | In-memory implementation of the Ligature API in Clojure | https://github.com/almibe/ligature-in-memory |
| ligature-xodus | Implementation of Ligature for the JVM that uses the Xodus data store. | https://github.com/almibe/ligature-xodus |
| ligature-foundationdb | Implementation of Ligature for the JVM that uses FoundationDB as its data store. | https://github.com/almibe/ligature-foundationdb |
| ligature-indexdb | Implementation for ClojureScript that uses IndexDB as its data store. | https://github.com/almibe/ligature-indexdb |
