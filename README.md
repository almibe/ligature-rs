# Ligature

Ligature is a library that supports parsing N-Triples and Turtle and loading them into a
graph data structure for processing with JVM applications.

The goals of this project are as follows:
* Support the N-Triples (http://www.w3.org/TR/n-triples/) and 
Turtle (https://www.w3.org/TR/turtle/) specifications with ANTRL grammars
* Provide simple Kotlin api for working with library
* Express graph data using Google Guava's network implementation
* Have extensive example based tests written in Spock (http://spockframework.org/)
* Support OSGi

Related projects
* almibe/ligature-grammar - the ANTLR grammars used by this project
* almibe/ligature-store - backend for Ligature based on using OrientDB to persist values
