;; List Processing and Mathematical Sequences
;; Demonstrates advanced list operations and mathematical concepts

;; Create a list of powers of 2
(list 1 2 4 8 16 32 64 128 256 512)

;; Create a list of square numbers
(list 0 1 4 9 16 25 36 49 64 81 100)

;; Create a list of triangular numbers (sum of first n natural numbers)
(list 0 1 3 6 10 15 21 28 36 45 55)

;; List operations on mathematical sequences

;; Get the 5th power of 2 (index 4)
(car (cdr (cdr (cdr (cdr (list 1 2 4 8 16 32))))))

;; Get the 4th square number (index 3)
(car (cdr (cdr (cdr (list 0 1 4 9 16 25)))))

;; Calculate sum of first 5 powers of 2
(+ 1 2 4 8 16)

;; Calculate sum of first 4 square numbers
(+ 0 1 4 9)

;; Calculate sum of first 3 triangular numbers
(+ 0 1 3)

;; List concatenation using cons
(cons 100 (list 200 300 400))

;; Nested list operations
(car (cdr (list 10 20 30 40 50)))

(cdr (cdr (cdr (list 1 2 3 4 5))))

;; Conditional list processing
(if (> (car (list 15 5 25)) 10) "first element is greater than 10" "first element is not greater than 10")

(if (null? (list)) "list is empty" "list is not empty")

;; Mathematical operations with list elements

;; Calculate average of first 3 numbers in a list (conceptually)
;; (10 + 20 + 30) / 3 = 60 / 3 = 20
(/ (+ 10 20 30) 3)

;; Calculate geometric mean of first 3 numbers (conceptually)
;; (2 * 4 * 8)^(1/3) = 64^(1/3) = 4
;; We'll calculate 2 * 4 * 8 = 64
(* 2 (* 4 8))

;; List building patterns

;; Build a list step by step
(cons 1 (cons 2 (cons 3 (list))))

;; Build a list with arithmetic progression
(list 1 (+ 1 1) (+ 1 1 1) (+ 1 1 1 1))

;; Build a list with geometric progression
(list 2 (* 2 2) (* 2 2 2) (* 2 2 2 2))

;; Complex nested expressions with lists

;; Check if first element is greater than sum of rest
(if (> (car (list 20 5 5 5)) (+ 5 5 5)) "first dominates" "first does not dominate")

;; Check if list has at least 3 elements and first is positive
(if (and (> (car (list 10 -5 15)) 0) (> (car (cdr (cdr (list 10 -5 15)))) 0)) "conditions met" "conditions not met")

;; Display results
(display "List processing examples completed!")
(display "Mathematical sequences are fascinating!") 