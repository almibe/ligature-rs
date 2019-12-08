(ns ligature.core)

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

(defn subject? [])

(defn predicate? [])

(defn object? [])

(defn iri? [])

(defn blank-node? [])

(defn literal? [])

(defn lang-literal? [])

(defn typed-literal? [])

(defn statement? [])

(defn statement
  ([subject predicate object] (statement subject predicate object nil))
  ([subject predicate object graph] :a))


