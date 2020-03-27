(def ifibo
     (λ n
        (if (= n 1)
          (list 0 1)
          ((λ l (push l (+ (last l) (last (pop l))))) (ifibo (- n 1))))))

(def fibo
     (λ n
        (if (< n 2)
          n
          (+ (fibo (- n 1)) (fibo (- n 2))))))

(def range
     (λ min
        (λ max
           (if (= min max)
             (list)
             (push (range min (- max 1)) (- max 1))))))

(def map
     (λ l
        (λ f
           (if (= (len l) 0)
             (list)
             (push (map (pop l) f)
                   (f (last l)))))))

(def remove
     (λ l
        (λ i
           (if (= (len l) (+ i 1))
             (pop l)
             (if (= (len l) 0)
               (list)
               (push (remove (pop l) i) (last l)))))))

(def merge
     (λ l1
        (λ l2
           (if (= (len l2) 0)
             l1
             (push (merge l1 (pop l2)) (last l2))))))

(def rev
     (λ l
        (if (= (len l) 0)
          (list)
          (merge (list (last l)) (rev (pop l))))))

(map (range 1 47) (λ n (print (ifibo n))))
