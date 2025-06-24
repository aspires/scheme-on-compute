use std::collections::HashMap;

// Simple Scheme interpreter for demonstration
pub struct SchemeInterpreter {
    env: HashMap<String, SchemeValue>,
}

#[derive(Clone, Debug)]
pub enum SchemeValue {
    String(String),
    Number(f64),
    Boolean(bool),
    List(Vec<SchemeValue>),
    Vector(Vec<SchemeValue>),
    HashTable(std::collections::HashMap<String, SchemeValue>),
    Function(fn(&[SchemeValue], &mut HashMap<String, SchemeValue>) -> Result<SchemeValue, String>),
    Lambda {
        params: Vec<String>,
        body: Vec<String>,
        env: HashMap<String, SchemeValue>,
    },
    Symbol(String),
    Nil,
}

impl SchemeInterpreter {
    pub fn new() -> Self {
        let mut env = HashMap::new();
        
        // Add some basic functions
        env.insert("display".to_string(), SchemeValue::Function(|args, _| {
            if args.len() != 1 {
                return Err("display requires exactly one argument".to_string());
            }
            match &args[0] {
                SchemeValue::String(s) => Ok(SchemeValue::String(s.clone())),
                SchemeValue::Number(n) => Ok(SchemeValue::String(n.to_string())),
                SchemeValue::Boolean(b) => Ok(SchemeValue::String(b.to_string())),
                _ => Ok(SchemeValue::String("display".to_string())),
            }
        }));
        
        env.insert("+".to_string(), SchemeValue::Function(|args, _| {
            let mut sum = 0.0;
            for arg in args {
                match arg {
                    SchemeValue::Number(n) => sum += n,
                    _ => return Err("+ requires numeric arguments".to_string()),
                }
            }
            Ok(SchemeValue::Number(sum))
        }));

        env.insert("-".to_string(), SchemeValue::Function(|args, _| {
            if args.len() < 1 {
                return Err("- requires at least one argument".to_string());
            }
            match &args[0] {
                SchemeValue::Number(n) => {
                    if args.len() == 1 {
                        Ok(SchemeValue::Number(-n))
                    } else {
                        let mut result = *n;
                        for arg in &args[1..] {
                            match arg {
                                SchemeValue::Number(num) => result -= num,
                                _ => return Err("- requires numeric arguments".to_string()),
                            }
                        }
                        Ok(SchemeValue::Number(result))
                    }
                }
                _ => Err("- requires numeric arguments".to_string()),
            }
        }));

        env.insert("*".to_string(), SchemeValue::Function(|args, _| {
            let mut product = 1.0;
            for arg in args {
                match arg {
                    SchemeValue::Number(n) => product *= n,
                    _ => return Err("* requires numeric arguments".to_string()),
                }
            }
            Ok(SchemeValue::Number(product))
        }));

        env.insert("<".to_string(), SchemeValue::Function(|args, _| {
            if args.len() != 2 {
                return Err("< requires exactly two arguments".to_string());
            }
            match (&args[0], &args[1]) {
                (SchemeValue::Number(a), SchemeValue::Number(b)) => {
                    Ok(SchemeValue::Boolean(a < b))
                }
                _ => Err("< requires numeric arguments".to_string()),
            }
        }));

        env.insert("=".to_string(), SchemeValue::Function(|args, _| {
            if args.len() != 2 {
                return Err("= requires exactly two arguments".to_string());
            }
            match (&args[0], &args[1]) {
                (SchemeValue::Number(a), SchemeValue::Number(b)) => {
                    Ok(SchemeValue::Boolean(a == b))
                }
                _ => Err("= requires numeric arguments".to_string()),
            }
        }));

        env.insert("if".to_string(), SchemeValue::Function(|args, _| {
            if args.len() != 3 {
                return Err("if requires exactly three arguments".to_string());
            }
            match &args[0] {
                SchemeValue::Boolean(true) => Ok(args[1].clone()),
                SchemeValue::Boolean(false) => Ok(args[2].clone()),
                _ => Err("if condition must be boolean".to_string()),
            }
        }));

        // List operations
        env.insert("cons".to_string(), SchemeValue::Function(|args, _| {
            if args.len() != 2 {
                return Err("cons requires exactly two arguments".to_string());
            }
            let mut list = Vec::new();
            list.push(args[0].clone());
            match &args[1] {
                SchemeValue::List(l) => list.extend(l.clone()),
                SchemeValue::Nil => {},
                _ => list.push(args[1].clone()),
            }
            Ok(SchemeValue::List(list))
        }));

        env.insert("car".to_string(), SchemeValue::Function(|args, _| {
            if args.len() != 1 {
                return Err("car requires exactly one argument".to_string());
            }
            match &args[0] {
                SchemeValue::List(list) => {
                    if list.is_empty() {
                        Ok(SchemeValue::Nil)
                    } else {
                        Ok(list[0].clone())
                    }
                }
                _ => Err("car requires a list argument".to_string()),
            }
        }));

        env.insert("cdr".to_string(), SchemeValue::Function(|args, _| {
            if args.len() != 1 {
                return Err("cdr requires exactly one argument".to_string());
            }
            match &args[0] {
                SchemeValue::List(list) => {
                    if list.len() <= 1 {
                        Ok(SchemeValue::Nil)
                    } else {
                        Ok(SchemeValue::List(list[1..].to_vec()))
                    }
                }
                _ => Err("cdr requires a list argument".to_string()),
            }
        }));

        env.insert("list".to_string(), SchemeValue::Function(|args, _| {
            Ok(SchemeValue::List(args.to_vec()))
        }));

        env.insert("null?".to_string(), SchemeValue::Function(|args, _| {
            if args.len() != 1 {
                return Err("null? requires exactly one argument".to_string());
            }
            match &args[0] {
                SchemeValue::List(list) => Ok(SchemeValue::Boolean(list.is_empty())),
                SchemeValue::Nil => Ok(SchemeValue::Boolean(true)),
                _ => Ok(SchemeValue::Boolean(false)),
            }
        }));

        // Additional arithmetic
        env.insert(">".to_string(), SchemeValue::Function(|args, _| {
            if args.len() != 2 {
                return Err("> requires exactly two arguments".to_string());
            }
            match (&args[0], &args[1]) {
                (SchemeValue::Number(a), SchemeValue::Number(b)) => {
                    Ok(SchemeValue::Boolean(a > b))
                }
                _ => Err("> requires numeric arguments".to_string()),
            }
        }));

        env.insert(">=".to_string(), SchemeValue::Function(|args, _| {
            if args.len() != 2 {
                return Err(">= requires exactly two arguments".to_string());
            }
            match (&args[0], &args[1]) {
                (SchemeValue::Number(a), SchemeValue::Number(b)) => {
                    Ok(SchemeValue::Boolean(a >= b))
                }
                _ => Err(">= requires numeric arguments".to_string()),
            }
        }));

        env.insert("<=".to_string(), SchemeValue::Function(|args, _| {
            if args.len() != 2 {
                return Err("<= requires exactly two arguments".to_string());
            }
            match (&args[0], &args[1]) {
                (SchemeValue::Number(a), SchemeValue::Number(b)) => {
                    Ok(SchemeValue::Boolean(a <= b))
                }
                _ => Err("<= requires numeric arguments".to_string()),
            }
        }));

        env.insert("/".to_string(), SchemeValue::Function(|args, _| {
            if args.len() != 2 {
                return Err("/ requires exactly two arguments".to_string());
            }
            match (&args[0], &args[1]) {
                (SchemeValue::Number(a), SchemeValue::Number(b)) => {
                    if *b == 0.0 {
                        Err("Division by zero".to_string())
                    } else {
                        Ok(SchemeValue::Number(a / b))
                    }
                }
                _ => Err("/ requires numeric arguments".to_string()),
            }
        }));

        env.insert("and".to_string(), SchemeValue::Function(|args, _| {
            if args.is_empty() {
                return Ok(SchemeValue::Boolean(true));
            }
            for arg in args {
                match arg {
                    SchemeValue::Boolean(false) => return Ok(SchemeValue::Boolean(false)),
                    _ => continue,
                }
            }
            Ok(args.last().unwrap().clone())
        }));

        env.insert("or".to_string(), SchemeValue::Function(|args, _| {
            if args.is_empty() {
                return Ok(SchemeValue::Boolean(false));
            }
            for arg in args {
                match arg {
                    SchemeValue::Boolean(true) => return Ok(SchemeValue::Boolean(true)),
                    _ => continue,
                }
            }
            Ok(args.last().unwrap().clone())
        }));

        // Advanced control flow
        env.insert("begin".to_string(), SchemeValue::Function(|args, _env| {
            if args.is_empty() {
                return Ok(SchemeValue::Nil);
            }
            Ok(args.last().unwrap().clone())
        }));

        env.insert("let".to_string(), SchemeValue::Function(|args, _env| {
            if args.is_empty() {
                return Ok(SchemeValue::Nil);
            }
            // For now, just return the last argument (the body)
            // In a full implementation, this would evaluate bindings and create a new environment
            if args.len() >= 2 {
                Ok(args.last().unwrap().clone())
            } else {
                Ok(args[0].clone())
            }
        }));

        env.insert("cond".to_string(), SchemeValue::Function(|args, _env| {
            if args.is_empty() {
                return Ok(SchemeValue::Nil);
            }
            
            // Process condition-result pairs
            // Each pair should be a list with [condition result]
            for i in (0..args.len()).step_by(2) {
                if i + 1 < args.len() {
                    // Check if this is a condition-result pair
                    let condition = &args[i];
                    let result = &args[i + 1];
                    
                    // Evaluate the condition
                    match condition {
                        SchemeValue::Boolean(true) => {
                            return Ok(result.clone());
                        }
                        SchemeValue::Boolean(false) => {
                            continue; // Try next pair
                        }
                        _ => {
                            // For non-boolean conditions, treat as true if not false
                            return Ok(result.clone());
                        }
                    }
                }
            }
            
            // If no condition was true, return the last argument or nil
            if !args.is_empty() {
                Ok(args.last().unwrap().clone())
            } else {
                Ok(SchemeValue::Nil)
            }
        }));

        // Data structures
        env.insert("vector".to_string(), SchemeValue::Function(|args, _| {
            Ok(SchemeValue::Vector(args.to_vec()))
        }));

        env.insert("make-hash-table".to_string(), SchemeValue::Function(|_args, _| {
            Ok(SchemeValue::HashTable(std::collections::HashMap::new()))
        }));

        env.insert("hash-set!".to_string(), SchemeValue::Function(|args, _| {
            if args.len() != 3 {
                return Err("hash-set! requires exactly three arguments".to_string());
            }
            match (&args[0], &args[1], &args[2]) {
                (SchemeValue::HashTable(_), SchemeValue::String(_key), value) => {
                    // In a full implementation, this would modify the hash table
                    Ok(value.clone())
                }
                _ => Err("hash-set! requires a hash table, string key, and value".to_string()),
            }
        }));

        env.insert("hash-ref".to_string(), SchemeValue::Function(|args, _| {
            if args.len() != 2 {
                return Err("hash-ref requires exactly two arguments".to_string());
            }
            match (&args[0], &args[1]) {
                (SchemeValue::HashTable(_), SchemeValue::String(key)) => {
                    // In a full implementation, this would look up the key
                    Ok(SchemeValue::String(format!("value for key: {}", key)))
                }
                _ => Err("hash-ref requires a hash table and string key".to_string()),
            }
        }));

        // Vector operations
        env.insert("vector-ref".to_string(), SchemeValue::Function(|args, _| {
            if args.len() != 2 {
                return Err("vector-ref requires exactly two arguments".to_string());
            }
            match (&args[0], &args[1]) {
                (SchemeValue::Vector(vec), SchemeValue::Number(index)) => {
                    let idx = *index as usize;
                    if idx < vec.len() {
                        Ok(vec[idx].clone())
                    } else {
                        Err("vector index out of bounds".to_string())
                    }
                }
                _ => Err("vector-ref requires a vector and numeric index".to_string()),
            }
        }));

        env.insert("vector-length".to_string(), SchemeValue::Function(|args, _| {
            if args.len() != 1 {
                return Err("vector-length requires exactly one argument".to_string());
            }
            match &args[0] {
                SchemeValue::Vector(vec) => Ok(SchemeValue::Number(vec.len() as f64)),
                _ => Err("vector-length requires a vector".to_string()),
            }
        }));

        // Loop constructs
        env.insert("while".to_string(), SchemeValue::Function(|args, _env| {
            if args.len() < 2 {
                return Err("while requires at least condition and body".to_string());
            }
            Ok(args[0].clone())
        }));

        env.insert("for-each".to_string(), SchemeValue::Function(|args, _env| {
            if args.len() < 2 {
                return Err("for-each requires at least function and list".to_string());
            }
            Ok(args[0].clone())
        }));

        // List processing
        env.insert("length".to_string(), SchemeValue::Function(|args, _| {
            if args.len() != 1 {
                return Err("length requires exactly one argument".to_string());
            }
            match &args[0] {
                SchemeValue::List(list) => Ok(SchemeValue::Number(list.len() as f64)),
                SchemeValue::Vector(vec) => Ok(SchemeValue::Number(vec.len() as f64)),
                SchemeValue::String(s) => Ok(SchemeValue::Number(s.len() as f64)),
                _ => Err("length requires a list, vector, or string".to_string()),
            }
        }));

        env.insert("append".to_string(), SchemeValue::Function(|args, _| {
            if args.is_empty() {
                return Ok(SchemeValue::Nil);
            }
            let mut result = Vec::new();
            for arg in args {
                match arg {
                    SchemeValue::List(list) => result.extend(list.clone()),
                    _ => result.push(arg.clone()),
                }
            }
            Ok(SchemeValue::List(result))
        }));

        // Mathematical functions
        env.insert("abs".to_string(), SchemeValue::Function(|args, _| {
            if args.len() != 1 {
                return Err("abs requires exactly one argument".to_string());
            }
            match &args[0] {
                SchemeValue::Number(n) => Ok(SchemeValue::Number(n.abs())),
                _ => Err("abs requires a numeric argument".to_string()),
            }
        }));

        env.insert("sqrt".to_string(), SchemeValue::Function(|args, _| {
            if args.len() != 1 {
                return Err("sqrt requires exactly one argument".to_string());
            }
            match &args[0] {
                SchemeValue::Number(n) => Ok(SchemeValue::Number(n.sqrt())),
                _ => Err("sqrt requires a numeric argument".to_string()),
            }
        }));

        env.insert("expt".to_string(), SchemeValue::Function(|args, _| {
            if args.len() != 2 {
                return Err("expt requires exactly two arguments".to_string());
            }
            match (&args[0], &args[1]) {
                (SchemeValue::Number(base), SchemeValue::Number(exponent)) => {
                    Ok(SchemeValue::Number(base.powf(*exponent)))
                }
                _ => Err("expt requires numeric arguments".to_string()),
            }
        }));

        Self { env }
    }

    pub fn eval(&self, expr: &str) -> Result<SchemeValue, String> {
        // Simple parser and evaluator
        let expr = expr.trim();
        
        // Handle empty expressions
        if expr.is_empty() {
            return Err("Empty expression".to_string());
        }
        
        // Handle nil/empty list tokens first
        if expr == "nil" || expr == "()" {
            return Ok(SchemeValue::Nil);
        }
        
        // Handle function calls (expressions starting with '(')
        if expr.starts_with('(') && expr.ends_with(')') {
            let inner = &expr[1..expr.len()-1]; // Remove outer parentheses
            let tokens = self.tokenize(inner)?;
            
            if tokens.is_empty() {
                return Err("Empty function call".to_string());
            }
            
            let func_name = &tokens[0];
            
            // Check if the function name is a boolean literal
            if func_name == "#t" {
                return Ok(SchemeValue::Boolean(true));
            }
            if func_name == "#f" {
                return Ok(SchemeValue::Boolean(false));
            }
            
            let args: Vec<SchemeValue> = tokens[1..]
                .iter()
                .map(|token| self.eval(token))
                .collect::<Result<Vec<_>, _>>()?;
            
            if let Some(SchemeValue::Function(func)) = self.env.get(func_name) {
                return func(&args, &mut HashMap::new());
            } else {
                return Err(format!("Unknown function: {}", func_name));
            }
        }
        
        // Handle simple cases (single tokens)
        // Check if it's a number
        if let Ok(num) = expr.parse::<f64>() {
            return Ok(SchemeValue::Number(num));
        }
        
        // Check if it's a string
        if expr.starts_with('"') && expr.ends_with('"') {
            return Ok(SchemeValue::String(expr[1..expr.len()-1].to_string()));
        }
        
        // Check if it's a boolean
        if expr == "#t" {
            return Ok(SchemeValue::Boolean(true));
        }
        if expr == "#f" {
            return Ok(SchemeValue::Boolean(false));
        }
        
        // Check if it's a variable/symbol
        if let Some(value) = self.env.get(expr) {
            return Ok(value.clone());
        }
        
        // If not found, treat as a symbol
        Ok(SchemeValue::Symbol(expr.to_string()))
    }
    
    fn tokenize(&self, input: &str) -> Result<Vec<String>, String> {
        let mut tokens = Vec::new();
        let mut current = String::new();
        let mut paren_count;
        let mut in_string = false;
        let mut chars = input.chars().peekable();
        
        while let Some(ch) = chars.next() {
            match ch {
                '"' => {
                    if in_string {
                        // End of string
                        current.push(ch);
                        tokens.push(current.clone());
                        current.clear();
                        in_string = false;
                    } else {
                        // Start of string
                        if !current.is_empty() {
                            tokens.push(current.clone());
                            current.clear();
                        }
                        current.push(ch);
                        in_string = true;
                    }
                }
                '(' => {
                    if in_string {
                        current.push(ch);
                    } else {
                        if !current.is_empty() {
                            tokens.push(current.clone());
                            current.clear();
                        }
                        current.push(ch);
                        paren_count = 1;
                        
                        // Collect the entire parenthesized expression
                        while paren_count > 0 {
                            match chars.next() {
                                Some('(') => {
                                    paren_count += 1;
                                    current.push('(');
                                }
                                Some(')') => {
                                    paren_count -= 1;
                                    current.push(')');
                                }
                                Some(c) => current.push(c),
                                None => return Err("Unmatched parentheses".to_string()),
                            }
                        }
                        tokens.push(current.clone());
                        current.clear();
                    }
                }
                ' ' | '\t' | '\n' => {
                    if in_string {
                        current.push(ch);
                    } else if !current.is_empty() {
                        tokens.push(current.clone());
                        current.clear();
                    }
                }
                _ => {
                    current.push(ch);
                }
            }
        }
        
        if !current.is_empty() {
            tokens.push(current);
        }
        
        Ok(tokens)
    }
    
    pub fn run_program(&self, program: &str) -> Result<String, String> {
        let lines: Vec<&str> = program.lines().collect();
        let mut output = String::new();
        
        for (i, line) in lines.iter().enumerate() {
            let line = line.trim();
            if !line.is_empty() && !line.starts_with(';') {
                // Debug: Print the line being processed
                output.push_str(&format!("Processing line {}: '{}'\n", i, line));
                
                match self.eval(line) {
                    Ok(result) => {
                        match result {
                            SchemeValue::String(s) => output.push_str(&s),
                            SchemeValue::Number(n) => output.push_str(&n.to_string()),
                            SchemeValue::Boolean(b) => output.push_str(&b.to_string()),
                            _ => output.push_str("result"),
                        }
                        output.push('\n');
                    }
                    Err(e) => {
                        output.push_str(&format!("Error on line {}: {}\n", i, e));
                        return Err(e);
                    }
                }
            }
        }
        
        Ok(output)
    }

    pub fn display_value(&self, value: &SchemeValue) -> String {
        match value {
            SchemeValue::String(s) => s.clone(),
            SchemeValue::Number(n) => n.to_string(),
            SchemeValue::Boolean(b) => b.to_string(),
            SchemeValue::List(list) => {
                let mut result = String::from("[");
                for (i, item) in list.iter().enumerate() {
                    if i > 0 { result.push_str(", "); }
                    result.push_str(&self.display_value(item));
                }
                result.push(']');
                result
            }
            SchemeValue::Vector(vec) => {
                let mut result = String::from("#(");
                for (i, item) in vec.iter().enumerate() {
                    if i > 0 { result.push_str(" "); }
                    result.push_str(&self.display_value(item));
                }
                result.push(')');
                result
            }
            SchemeValue::HashTable(_) => "#<hash-table>".to_string(),
            SchemeValue::Function(_) => "#<function>".to_string(),
            SchemeValue::Lambda { .. } => "#<lambda>".to_string(),
            SchemeValue::Symbol(s) => s.clone(),
            SchemeValue::Nil => "()".to_string(),
        }
    }
} 