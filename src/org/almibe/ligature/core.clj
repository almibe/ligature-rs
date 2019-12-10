(ns org.almibe.ligature.core)

(defprotocol LigatureStore
  "A Store manages many named Datasets."
  (get-dataset [this dateset-name])
  (delete-dataset [this dateset-name])
  (all-datasets [this])
  (close [this]))

(defprotocol LigatureDataset
  "A Dataset manages a collection of Statements and supports ontologies and querying."
  (add-statements [this statements]) ; accepts a seq of statement tuples
  (remove-statements [this statements]) ; accepts a seq of statement tuples
  (all-statements [this]) ; accepts nothing but returns a seq
  (match [this pattern])
  (dataset-name [this])
  (set-ontology [this ontology])
  (get-ontology [this])
  (sparql-query [this query])
  (wander-query [this query]))

(defn blank-node?
  "Accepts a String representing a Blank Node and returns true or false depending on if it is valid."
  [blank-node]
  (throw (RuntimeException. "TODO")))

(defn iri?
  "Accepts a String representing an iri and returns true or false depending on if it is valid."
  [iri]
  (throw (RuntimeException. "TODO")))

(defn string-literal?
  "Accepts a String and returns if it is a valid string literal."
  [literal]
  (string? literal))

(defn lang-literal?
  "Accepts a Map and returns true or false depending on if it is a valid lang literal.
  A lang literal should contain a :value key with a valid string literal and a :lang key with a valid lang code."
  [literal]
  (throw (RuntimeException. "TODO")))

(defn typed-literal?
  "Accepts a Map and returns true or false depending on if it is a valid typed literal.
  A typed literal should contain a :valud key with a valid string literal and a :type key with a valid IRI."
  [literal]
  (throw (RuntimeException. "TODO")))

(defn literal?
  "Accepts a String or Map representing a literal and returns true or false depending on if it is valid."
  [literal]
  (or (string-literal? literal) (lang-literal? literal) (typed-literal? literal)))

(defn subject?
  "Accepts a String representing a subject and returns true or false depending of
  whether or not that String is a valid IRI or Blank Node"
  [subject]
  (or (iri? subject) (blank-node? subject)))

(defn predicate? [predicate]
  "Accepts a String representing a predicate and returns true or false depending on if it is valid."
  (iri? predicate))

(defn object? [object]
  "Accepts a String or Map representing an object and returns true or false depending on if it is valid."
  (or (iri? object) (blank-node? object) (literal? object)))

(defn statement?
  "Accepts a Map and returns true or false depending on if it is a valid Statement.
  A valid Statement contains a :subject, :predicate, :object, and optionally a :graph key with valid values."
  [statement]
  (throw (RuntimeException. "TODO")))

(defn statement
  "This function acts as a helper function for creating Statement maps."
  ([subject predicate object]
   {:subject subject :predicate predicate :object object})
  ([subject predicate object graph]
    {:subject subject :predicate predicate :object object :graph graph}))
