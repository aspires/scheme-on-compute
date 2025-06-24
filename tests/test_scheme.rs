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
} 