(def iter
     (λ n
        (λ f
           (if (= n 0)
             (f 0)
             (+ (iter (- n 1) f) (f n))))))

(def fibo
     (λ n
        (if (= n 0)
          0
          (if (= n 1)
            1
            (+ (fibo (- n 1)) (fibo (- n 2)))))))

(iter 15 (λ n (print (fibo n))))
