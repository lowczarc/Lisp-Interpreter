(def iter
     (λ n
        (λ f
           (if (< n 1)
             0
             (+ (iter (- n 1) f) (if (f (- n 1)) 1 1))))))

(def fibo
     (λ n
        (if (< n 2)
          n
          (+ (fibo (- n 1)) (fibo (- n 2))))))

(def fac
     (λ n
        (if (< n 1)
          1
          (* n (fac (- n 1))))))

(iter 15 (λ n (print (fibo n))))
(iter 15 (λ n (print (fac n))))
