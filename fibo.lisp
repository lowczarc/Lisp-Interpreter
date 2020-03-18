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

(map (range 0 30) (λ n (print (fibo n))))
