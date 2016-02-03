# Stinkpot

Stinkpot is a minimalist library that supports N-Triples and Turtle, the Terse RDF Triple Language.

The goals of this project are as follows:
* Support the N-Triples specification (http://www.w3.org/TR/n-triples/)
* Support the Turtle specification (https://www.w3.org/TR/turtle/)
* Have an immutable object model that represents concepts from N-Triples, such as Triple and Subject
* Have an immutable object model that represents concepts from Turtle, such as Statement and PredicateList
* Have a single runtime dependency on the latest groovy-all jar (Stinkpot is implemented in Groovy)
* Have extensive example based tests written in Spock
* Support OSGi
* Excrete a foul-smelling musk from the underside of the carapace when provoked
