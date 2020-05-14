"Working with infinite list

We define an iterator as a list that contains the current element and a function that returns an iterator to the next element

We can define [0:+∞[, an iterator that return all the natural number from 0 to +∞
"

(def [0:+∞[
     (list 0
           (λ l
              (list (+ (last (pop l)) 1)
                    (last l)))))

"To make the code more readable we will define the functions:
    - next which return the iterator to the next element 
     and
    - get which return the current element
"

(def next (λ i ((last i) i)))

(def get (λ i (last (pop i))))

"We can define a function map which takes a function and an iterator and return a new iterator where all the elements are passed to the function
"

(def map (λ f (λ i (list (f (get i)) (λ l (map f (next i)))))))

"And a function filter which takes a function and an iterator and return an new iterator where all the elements where the function return a false value are skipped
"
(def filter
     (λ f
        (λ i
           (if (f (get i))
             (list (get i) (λ l (filter f (next i))))
             (filter f (next i))))))

"To test our infinite lists we can make a function sieve to find prime number (Sieve of Eratosthenes method)

We also define some basic functions to make the code more readable
"
(def % (λ x (λ y (- x (* (/ x y) y)))))

(def not (λ x (if x 0 1)))

(def sieve
     (λ i
        (list (get i)
              (λ l (sieve (filter (λ x (not (= (% x (get i)) 0))) (next i)))))))

"The list of all primes is the result of sieve([2:+∞[)
"
(def primes (sieve (next (next [0:+∞[))))

(def printi
     (λ i
        (list (print (get i)) (printi (next i)))))


(printi primes)
