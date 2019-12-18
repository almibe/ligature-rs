(ns org.almibe.ligature.core)

(defprotocol LigatureStore
  "A Store manages many named Datasets."
  (get-dataset [this dateset-name]
    "Returns an existing or new dataset based on the name passed.")
  (delete-dataset [this dateset-name]
    "Deletes the dataset of the name given.")
  (all-datasets [this]
    "Returns a seq of all existing datasets.")
  (close [this]
    "Close connection with the Store.")
  (location [this]
    "Returns a String representation of this Store's location."))

(defprotocol LigatureDataset
  "A Dataset manages a collection of Statements and supports ontologies and querying."
  (add-statements [this statements]
    "Accepts a seq of statement tuples")
  (remove-statements [this statements]
    "Accepts a seq of statement tuples")
  (all-statements [this]
    "Accepts nothing but returns a seq of all Statements in the Dataset.")
  (new-identifier [this]
    "Returns a unique, new identifier in the form _:NUMBER")
  (match [this pattern])
  (dataset-name [this])
  (set-ontology [this ontology])
  (get-ontology [this])
  (sparql-query [this query])
  (wander-query [this query]))

(defn identifier?
  "Accepts a String representing an identifier and returns true or false depending on if it is valid."
  [identifier]
  (and
    (string? identifier)
    (not (nil?(re-matches #"[a-zA-Z_][^\s\(\)\[\]\{\}\'\"`<>\\]*" identifier)))))

(defn lang-tag?
  "Accepts a String representing a lang tag and returns true or false depending on if it is valid."
  [lang]
  (and
    (string? lang)
    (not (nil?(re-matches #"[a-zA-Z]+(-[a-zA-Z0-9]+)*" lang)))))

(defn lang-literal?
  "Accepts a Map and returns true or false depending on if it is a valid lang literal.
  A lang literal should contain a :value key with a valid string literal and a :lang key with a valid lang code."
  [literal]
  (and
    (map? literal)
    (= (keys literal) [:lang :value])
    (lang-tag? (:lang literal))
    (string? (:value literal))))

(defn typed-literal?
  "Accepts a Map and returns true or false depending on if it is a valid typed literal.
  A typed literal should contain a :valud key with a valid string literal and a :type key with a valid identifier."
  [literal]
  (and
    (map? literal)
    (= (keys literal) [:type :value])
    (identifier? (:type literal))
    (string? (:value literal))))

(defn literal?
  "Accepts a String or Map representing a literal and returns true or false depending on if it is valid."
  [literal]
  (or (lang-literal? literal) (typed-literal? literal)))

(defn subject?
  "Accepts a String representing a subject and returns true or false depending of
  whether or not that String is a valid identifier."
  [subject]
  (identifier? subject))

(defn predicate? [predicate]
  "Accepts a String representing a predicate and returns true or false depending on if it is valid."
  (identifier? predicate))

(defn object? [object]
  "Accepts a String or Map representing an object and returns true or false depending on if it is valid."
  (or (identifier? object) (literal? object)))

(defn graph?
  "Checks that a passed String value is either a valid identifier or nil"
  [graph]
  (or
    (nil? graph)
    (identifier? graph)))

(defn statement?
  "Accepts a Map and returns true or false depending on if it is a valid Statement.
  A valid Statement contains a :subject, :predicate, :object, and optionally a :graph key with valid values."
  [statement]
  (and
    (map? statement)
    (or
      (= (keys statement) [:subject :predicate :object])
      (= (keys statement) [:subject :predicate :object :graph]))
    (subject? (:subject statement))
    (predicate? (:predicate statement))
    (object? (:object statement))
    (graph? (:graph statement))))

(defn statement
  "This function acts as a helper function for creating Statement maps."
  ([subject predicate object]
   {:subject subject :predicate predicate :object object})
  ([subject predicate object graph]
    {:subject subject :predicate predicate :object object :graph graph}))
