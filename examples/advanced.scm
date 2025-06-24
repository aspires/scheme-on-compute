;; Advanced Scheme Examples for Fastly Compute
;; This file demonstrates recursion, list operations, and complex expressions

;; Recursive factorial function (conceptually)
;; (define (factorial n)
;;   (if (= n 0) 1 (* n (factorial (- n 1)))))

;; Let's calculate factorial manually for n=5
;; 5! = 5 * 4 * 3 * 2 * 1 = 120

;; Calculate factorial step by step
(* 5 (* 4 (* 3 (* 2 1))))

;; Recursive Fibonacci (conceptually)
;; (define (fib n)
;;   (if (< n 2) n (+ (fib (- n 1)) (fib (- n 2)))))

;; Calculate Fibonacci numbers manually
;; F(0) = 0, F(1) = 1, F(2) = 1, F(3) = 2, F(4) = 3, F(5) = 5, F(6) = 8

;; List operations with Fibonacci numbers
(list 0 1 1 2 3 5 8 13 21 34)

;; Calculate sum of first 5 Fibonacci numbers
(+ 0 1 1 2 3)

;; Calculate product of first 4 Fibonacci numbers
(* 0 1 1 2)

;; List manipulation examples
(cons 42 (list 1 2 3))

(car (list 10 20 30 40))

(cdr (list 10 20 30 40))

;; Nested list operations
(car (cdr (list 1 2 3 4)))

(cdr (cdr (list 1 2 3 4)))

;; Conditional expressions with lists
(if (null? (list)) "empty" "not empty")

(if (> (car (list 5 2 8)) 3) "first element is greater than 3" "first element is not greater than 3")

;; Arithmetic with list elements
(+ (car (list 10 20 30)) (car (cdr (list 10 20 30))))

(* (car (list 2 4 6)) (car (cdr (cdr (list 2 4 6)))))

;; Comparison operations
(> 10 5)
(< 3 7)
(= 5 5)
(>= 10 10)
(<= 3 8)

;; Complex nested expressions
(if (> (+ 5 3) (* 2 3)) "sum is greater than product" "product is greater than sum")

(if (= (car (list 5 10 15)) (+ 2 3)) "first element equals 2+3" "first element does not equal 2+3")

;; List building with cons
(cons 1 (cons 2 (cons 3 (list))))

;; Display messages
(display "Advanced Scheme operations working!")
(display "Recursion and lists are powerful concepts!") 