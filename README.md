# ligature
Ligature is a Clojure library for working with semantic data.
It is based on RDF and related standards but has a more flexible approach to working with semantic data.
It's main difference is that it is intended to be used in a broader context than strict RDF.
This means that identifiers do not have to be IRIs and blank nodes only exist in compatibility contexts.

| Subject    | Predicate  | Object     | Graph      |
| ---------- | ---------- | ---------- | ---------- |
| iri        | iri        | iri        | iri        |
| blank node |            | blank node |            |
|            |            | literal    |            |

| Subject    | Predicate  | Object     | Graph      |
| ---------- | ---------- | ---------- | ---------- |
| identifier | identifier | identifier | identifier |
|            |            | literal    |            |

Identifiers in Ligature are *currently* defined as strings that don't contain any of the following characters:
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
(def x (new-identifier))  ; x = _:34
(add-statements [(statement x "isa" "company")]) ; should run fine
(add-statements [(statement "_:34" "name" "Pear")]) ; should run fine since _:34 has been created already
(add-statements [(statement "_:34567" "isa" "bird")]) ; probably will error out since I doubt that identifer has been created....but it could....but it probably wasn't
```
