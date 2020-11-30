"Some constructions to explains the lambda calculus ideas"

" ## BOLEAN LOGIC ## "
"True & False"
(def true (λ a (λ b a)))
(def false (λ a (λ b b)))

"Logic Gates"
(def Or (λ a (λ b (a a b))))
(def And (λ a (λ b (a b a))))
(def Not (λ a (a false true)))


" ## ARITHMETIC ## "
"Zero"
(def _0 (λ a false))

"Successor function"
(def S (λ n (λ b (b n true))))

"Equality"
(def Eq (λ m
           (λ n
              (((m false) ((n false) (λ _ (Eq (m true) (n true)))
                                    (λ _ false))
                         ((n false) (λ _ false)
                                    (λ _ true))) nil))))

"Addition"
(def Add (λ m
            (λ n
               (((n false) (λ _ (S (Add m (n true))))
                           (λ _ m)) nil))))

"Multiplication"
(def Mul (λ m
            (λ n
               (((n false) (λ _ (Add (Mul m (n true)) m))
                           (λ _ _0)) nil))))

"Inequalities"
(def Lte (λ m
            (λ n
               (((m false) ((n false) (λ _ (Lte (m true) (n true)))
                                      (λ _ false))
                           (λ _ true)) nil))))
(def Gte (λ m (λ n (Lte n m))))
(def Gt (λ m (λ n (Not (Lte m n)))))
(def Lt (λ m (λ n (Gt n m))))


" ## LISTS ## "
"Empty List"
(def L0 (λ a false))

"Push List"
(def Lpush (λ l (λ c (λ a (a (λ b (b l c)) true)))))

"Get List"
(def Lget (λ l ((l true) false)))

"Pop List"
(def Lpop (λ l ((l true) true)))

"Len List"
(def Llen (λ l (((l false) (λ _ (S (Llen (Lpop l)))) (λ _ _0)) nil)))


" ## UTILITY FUNCTIONS ## "
(def tobool (λ a (a 1 0)))
(def tonum (λ n (((n false) (λ _ (+ (tonum (n true)) 1)) (λ _ 0)) nil)))
(def fromnum (λ n (if (= n 0) _0 (S (fromnum (- n 1))))))

(print (tonum (Mul (fromnum 5) (fromnum 10))))
(print (tonum (Llen (Lpush (Lpush L0 _0) _0))))
