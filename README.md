# Ligature

Ligature is a minimalist library that supports parsing N-Triples and Turtle documents into an immutable Kotlin object model for processing with JVM applications.

The goals of this project are as follows:
* Support the N-Triples (http://www.w3.org/TR/n-triples/) and Turtle (https://www.w3.org/TR/turtle/) specifications with ANTRL grammars
* Have an immutable Kotlin based object model that represents concepts from RDF
* Have extensive example based tests written in Spock (http://spockframework.org/)
* Support OSGi

Related projects
* almibe/ligature-grammar - the ANTLR grammars used by this project
* almibe/ligature-store - an example library that stores Ligature's rdf model in OrientDB
