(def fibo
     (λ n
        (if (= n 0)
          0
          (if (= n 1)
            1
            (+ (fibo (- n 1)) (fibo (- n 2)))))))

(print (fibo 0) " " (fibo 1) " " (fibo 2) " " (fibo 3) " " (fibo 4) " " (fibo 5))
