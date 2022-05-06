(ns p1)
(use 'clojure.java.io)
(require ['clojure.string :as 'str])

(defn get_fuel [mass] 
  (- (quot mass 3) 2))

(defn parse-int [string] 
  (Integer/parseInt string))

(defn get-lines [file] (str/split-lines (slurp file)))

(def input (map parse-int (get-lines "./inputs/1-2.txt")))
(println input)
(println (reduce + (map get_fuel input)))


