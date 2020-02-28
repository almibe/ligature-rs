# ligature

Ligature is a Clojure library for working with semantic data.
This project provides the main protocols for Ligature as well as Spec support and some common helper functions.
It is based on RDF and related standards but has a more flexible approach to working with semantic data.
Its main difference is that it is intended to be used in a broader context than strict RDF.
In practice the main differences are identifiers do not have to be IRIs, blank nodes only exist in compatibility contexts, and quads are used for all data.

## RDF's Data Model

| Subject    | Predicate  | Object     | Graph?     |
| ---------- | ---------- | ---------- | ---------- |
| iri        | iri        | iri        | iri        |
| blank node |            | blank node | blank node |
|            |            | literal    |            |

## Ligature's Data Model

| Subject    | Predicate  | Object     | Graph      |
| ---------- | ---------- | ---------- | ---------- |
| identifier | identifier | identifier | identifier |
|            |            | literal    |            |

### Triples and Quads vs Just Quads

Unlike RDF formats like N-Quads where statements can be either a triple or a quad, in Ligature everything is a quad.
The reason for making this distinction is that in N-Quads there is no way to reference the default graph.
If something is represented as a triple it's in the default graph and if it's represented as a quad it isn't.
Relying on arity like this can make things like searching and matching confusing.
For example do these patterns match the same set of statements?

`[:? :? :?]`

`[:? :? :? :?]`

It isn't immedately clear (to me at least).
By forcing the use of quads for all statements and making the default namespace explicitly referenceable as the identifier `_` this ambiguity is removed at the cost of a couple extra key presses.

### Identifiers

Identifiers in Ligature are *currently* defined as strings that start with an ASCII letter or an underscore and don't contain any of the following characters:
 * whitespace (space, newline, tabs, carriage returns, etc)
 * " ' `
 * < >
 * ( )
 * { }
 * \
 * [ ]

If for some reason you need any of these characters in your identifier it is suggested that you use standard URL encoding.

Identifiers can be something that is meaningful like an IRI/URL, an id from an existing system, a name, or it can be an incrementing id via the `new-identifier` function.
Below is an example statement using identifiers in Clojure format.

`["Emily" "loves" "cats" "_"]`

The `new-identifier` function returns a unique identifier that looks something like this.

`_:34622`

The `new-identifier` function runs inside a transaction so it is guarenteed to be unique and to not already exist in the Dataset at the time of creatation.
The form `_:NUMBER` is special in Ligature and only IDs that have been already created with the `new-identifier` function can be used.
For example here is some pseudo code.

```clojure
; running in a WriteTx with Ligature core required as `l`
(def x (l/new-identifier tx))  ; creates a new identifer, in this case let's say `x = _:34`
(l/add-statement tx [x l/a "company" l/_]) ; should run fine
(l/add-statement tx ["_:34" "name" "Pear" l/_]) ; should run fine since _:34 has been created already
(l/add-statement tx ["_:34567" l/a "bird" l/_]) ; will erorr out since that identifier hasn't been created yet
```

Also worth pointing out in the above code is the use of two defined constants in Ligature.
`l/a` represents the identifier `http://www.w3.org/1999/02/22-rdf-syntax-ns#type` and `l/_` is used as a namespaced reference for the default graph identifer `_`.

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
