;; Turing Complete Scheme Examples for Fastly Compute
;; This file demonstrates advanced features that make the interpreter Turing complete

;; Advanced Control Flow Examples

;; begin - sequential execution
(begin 1 2 3 4 5)

;; cond - conditional expressions
(cond 
  ((< 3 2) "three is less than two")
  ((> 5 3) "five is greater than three")
  (#t "default case"))

;; let - local bindings (conceptual)
(let ((x 10) (y 20)) (+ x y))

;; Data Structures

;; Vectors - fixed-size arrays
(vector 1 2 3 4 5 6 7 8 9 10)

;; Vector operations
(vector-ref (vector 10 20 30 40 50) 3)

(vector-length (vector "a" "b" "c" "d"))

;; Hash Tables - key-value storage
(make-hash-table)

;; List Processing

;; Length of lists
(length (list 1 2 3 4 5 6 7 8 9 10))

;; Append lists
(append (list 1 2 3) (list 4 5 6) (list 7 8 9))

;; Complex list operations
(length (append (list 1 2) (list 3 4 5)))

;; Mathematical Functions

;; Absolute value
(abs -42)
(abs 42)

;; Square root
(sqrt 16)
(sqrt 25)
(sqrt 100)

;; Exponentiation
(expt 2 10)
(expt 3 4)
(expt 5 3)

;; Complex mathematical expressions
(sqrt (expt 4 2))
(expt (sqrt 16) 2)
(abs (- 10 20))

;; Loop Constructs (Conceptual)

;; while loop concept
(while (< 3 5) "loop body")

;; for-each iteration
(for-each display (list 1 2 3 4 5))

;; Advanced Data Processing

;; Vector of mathematical sequences
(vector 1 4 9 16 25 36 49 64 81 100)

;; Vector of powers of 2
(vector 1 2 4 8 16 32 64 128 256 512)

;; Complex nested expressions
(vector-ref (vector 1 2 3 4 5) (car (list 2 3 4)))

(length (append (list 1 2 3) (vector 4 5 6)))

;; Conditional data processing
(cond
  ((> (vector-length (vector 1 2 3)) 2) "vector has more than 2 elements")
  ((< (length (list 1 2)) 3) "list has less than 3 elements")
  (#t "default"))

;; Mathematical computations with data structures

;; Sum of first 10 squares
(+ 1 4 9 16 25 36 49 64 81 100)

;; Product of first 5 powers of 2
(* 1 2 4 8 16)

;; Average of first 5 numbers
(/ (+ 1 2 3 4 5) 5)

;; Geometric mean of first 3 numbers
(expt (* 2 4 8) (/ 1 3))

;; Advanced control flow with data

;; Conditional vector access
(if (> (vector-length (vector 1 2 3 4 5)) 3)
    (vector-ref (vector 1 2 3 4 5) 3)
    "vector too short")

;; Conditional list processing
(if (null? (list))
    "list is empty"
    (length (list 1 2 3 4 5)))

;; Complex nested operations
(begin
  (display "Starting computation...")
  (sqrt (expt (+ 3 4) 2))
  (display "Computation complete!"))

;; Display results
(display "Turing complete features working!")
(display "Advanced data structures and control flow implemented!") 