# Ligature

Ligature is an free and open source Knowledge Base.
Ligature allows you to model your data in a flexible way based on statements made about nodes.
Ligature takes a lot of inspiration from RDF but attempts to be more general purpose and applicable outside of the semantic web.
This project provides the API all implementations of Ligature should implement.

## Implementations

 * ligature-in-memory - an example implementation of Ligature based on in-memory graphs (specifically Google Guava's Networks).
 * ligature-xodus - an embeddable version of this API that works in-memory or persisted to disk (based on JetBrains' Xodus database).
 * ligature-foundationdb - a distributed version of Ligature (based on Apple's FoundationDB).

## How Does Ligature Model Data?

Below is a chart showing what types are expected for each part of a statement in Ligature.

| Entity | Attribute | Value   | Context |
| ------ | --------- | ------- | ------- |
| Node   | Node      | Node    | Node    |
|        |           | Literal |         |

Ligature strives to contain the bare number of concepts needed to represent data via statements.
A Node is simply something that can be referred to.
A Node can be referenced by its Id (which is autogenerated by the database) or it can have a label (set by the user) that it is referenced by.
Node labels must be unique for a given dataset but unlike Ids they can be changed by the user.
Context is the only optional part of a Statement and if a Context isn't given the data is stored in the Entity Node's own Context.
So for example the following two Statements (expressed in pseudo N-Quads format) are identical.

```
<lydia> <isA> <tabaxi>.
<lydia> <isA> <tabaxi> <lydia>.
```

This demonstrates a key difference between how Ligature models data and how RDF Quad Stores work.
Namely there is no "default" graph, all Statements belong to a specific Context that can be referenced.
You are free to create your own <default> but it is typically encouraged to keep Contexts small and focused.
Queries can be made against either a specific Context or against all the data in a Dataset.
