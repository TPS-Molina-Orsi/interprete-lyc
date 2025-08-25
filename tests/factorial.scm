(define (factorial n)
  (if (= n 0)                      ; is the argument 0?
      1                            ; then the answer is 1
      (* n (factorial (- n 1)))))  ; otherwise, it's n * (n - 1)!
