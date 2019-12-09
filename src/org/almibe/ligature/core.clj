(ns org.almibe.ligature.core)

(defprotocol LigatureStore
  (get-dataset [this dateset-name])
  (delete-dataset [this dateset-name])
  (all-datasets [this])
  (close [this]))

(defprotocol LigatureDataset
  (add-statements [this statements]) ; accepts a seq of statement tuples
  (remove-statements [this statements]) ; accepts a seq of statement tuples
  (all-statements [this]) ; accepts nothing but returns a seq
  (match [this pattern])
  (dataset-name [this])
  (set-ontology [this ontology])
  (get-ontology [this])
  (sparql-query [this query])
  (wander-query [this query]))

(defn blank-node? [blank-node]
  "Accepts a String representing a Blank Node and returns true or false depending on if it is valid."
  )

(defn iri? [iri]
  "Accepts a String representing an iri and returns true or false depending on if it is valid."

  )

(defn subject? [subject]
  "Accepts a String representing a subject and returns true or false depending of
  whether or not that String is a valid IRI or Blank Node"
  (or (iri? subject) (blank-node? subject)))

(defn predicate? [])

(defn object? [])



(defn literal? [])

(defn lang-literal? [])

(defn typed-literal? [])

(defn statement? [])

(defn statement
  ([subject predicate object] (statement subject predicate object nil))
  ([subject predicate object graph] :a))


