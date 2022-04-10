(defn messenger
([] (messenger "Hello World!"))
([msg] (println msg)))

(messenger)

(defn hello [greeting & who]
    (println greeting who))

(hello "Hello" "world" "class")

((fn [message] (println message)) "hello my friend!")
