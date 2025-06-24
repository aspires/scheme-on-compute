#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scheme_interpreter_basic() {
        let interpreter = SchemeInterpreter::new();
        
        // Test number evaluation
        assert!(matches!(interpreter.eval("42"), Ok(SchemeValue::Number(42.0))));
        
        // Test string evaluation
        assert!(matches!(interpreter.eval("\"hello\""), Ok(SchemeValue::String(_))));
        
        // Test boolean evaluation
        assert!(matches!(interpreter.eval("#t"), Ok(SchemeValue::Boolean(true))));
        assert!(matches!(interpreter.eval("#f"), Ok(SchemeValue::Boolean(false))));
    }

    #[test]
    fn test_scheme_interpreter_functions() {
        let interpreter = SchemeInterpreter::new();
        
        // Test display function
        let result = interpreter.eval("(display \"test\")");
        assert!(result.is_ok());
        
        // Test addition function
        let result = interpreter.eval("(+ 1 2 3)");
        assert!(matches!(result, Ok(SchemeValue::Number(6.0))));
    }

    #[test]
    fn test_scheme_program_execution() {
        let interpreter = SchemeInterpreter::new();
        let program = r#"
            (display "Hello")
            (+ 1 2)
            #t
        "#;
        
        let result = interpreter.run_program(program);
        assert!(result.is_ok());
        
        let output = result.unwrap();
        assert!(output.contains("Hello"));
        assert!(output.contains("3"));
        assert!(output.contains("true"));
    }

    #[test]
    fn test_examples_run_without_errors() {
        let interpreter = SchemeInterpreter::new();

        // List of all example files
        let examples = [
            include_str!("../examples/fibonacci.scm"),
            include_str!("../examples/advanced.scm"),
            include_str!("../examples/list-processing.scm"),
            include_str!("../examples/turing-complete.scm"),
            include_str!("../examples/computational-patterns.scm"),
        ];

        for (i, example) in examples.iter().enumerate() {
            let result = interpreter.run_program(example);
            assert!(
                result.is_ok(),
                "Example file {} failed to run: {:?}",
                i,
                result.err()
            );
            // Optionally: check for some expected output
            let output = result.unwrap();
            assert!(!output.is_empty(), "Example file {} produced no output", i);
        }
    }
} 