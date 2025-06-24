; Fibonacci sequence example for Scheme on Fastly Compute
; This demonstrates how you could extend the interpreter

; Note: This is a conceptual example - the current interpreter
; would need to be extended to support recursion and more complex
; list operations to run this code

; Define a simple factorial function
(define factorial
  (lambda (n)
    (if (= n 0)
        1
        (* n (factorial (- n 1))))))

; Calculate factorial of 5
(factorial 5)

; Define a simple list
(define my-list '(1 2 3 4 5))

; Get the first element
(car my-list)

; Get the rest of the list
(cdr my-list)

; Simple arithmetic operations
(+ 10 20 30)
(* 2 3 4)
(- 100 50)
(/ 100 4)

; String operations
(display "Hello from Scheme!")
(string-append "Fastly" " " "Compute" " " "Edge")

; Boolean operations
(and #t #f)
(or #t #f)
(not #f) 