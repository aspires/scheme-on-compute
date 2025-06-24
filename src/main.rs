#[cfg(feature = "fastly-binary")]
use fastly::http::{Method, StatusCode};
#[cfg(feature = "fastly-binary")]
use fastly::{mime, Error, Request, Response};
use lisp_compute::SchemeInterpreter;

#[cfg(feature = "fastly-binary")]
#[fastly::main]
fn main(req: Request) -> Result<Response, Error> {
    // Include the example files at compile time
    const FIBONACCI_EXAMPLE: &str = include_str!("../examples/fibonacci.scm");
    const ADVANCED_EXAMPLE: &str = include_str!("../examples/advanced.scm");
    const LIST_PROCESSING_EXAMPLE: &str = include_str!("../examples/list-processing.scm");
    const TURING_COMPLETE_EXAMPLE: &str = include_str!("../examples/turing-complete.scm");
    const COMPUTATIONAL_PATTERNS_EXAMPLE: &str = include_str!("../examples/computational-patterns.scm");
    
    // Only allow GET requests
    match req.get_method() {
        &Method::GET => (),
        _ => {
            return Ok(Response::from_status(StatusCode::METHOD_NOT_ALLOWED)
                .with_content_type(mime::TEXT_PLAIN_UTF_8)
                .with_body("Method not allowed"));
        }
    }
    
    // Create Scheme interpreter
    let interpreter = SchemeInterpreter::new();
    
    // Run the example files
    let mut output = String::new();
    
    // Run Fibonacci example
    output.push_str("=== FIBONACCI EXAMPLE ===\n");
    match interpreter.run_program(FIBONACCI_EXAMPLE) {
        Ok(result) => output.push_str(&result),
        Err(e) => output.push_str(&format!("Error running fibonacci.scm: {}\n", e)),
    }
    output.push_str("\n");
    
    // Run Advanced example
    output.push_str("=== ADVANCED EXAMPLE ===\n");
    match interpreter.run_program(ADVANCED_EXAMPLE) {
        Ok(result) => output.push_str(&result),
        Err(e) => output.push_str(&format!("Error running advanced.scm: {}\n", e)),
    }
    output.push_str("\n");
    
    // Run List Processing example
    output.push_str("=== LIST PROCESSING EXAMPLE ===\n");
    match interpreter.run_program(LIST_PROCESSING_EXAMPLE) {
        Ok(result) => output.push_str(&result),
        Err(e) => output.push_str(&format!("Error running list-processing.scm: {}\n", e)),
    }
    output.push_str("\n");
    
    // Run Turing Complete example
    output.push_str("=== TURING COMPLETE EXAMPLE ===\n");
    match interpreter.run_program(TURING_COMPLETE_EXAMPLE) {
        Ok(result) => output.push_str(&result),
        Err(e) => output.push_str(&format!("Error running turing-complete.scm: {}\n", e)),
    }
    output.push_str("\n");
    
    // Run Computational Patterns example
    output.push_str("=== COMPUTATIONAL PATTERNS EXAMPLE ===\n");
    match interpreter.run_program(COMPUTATIONAL_PATTERNS_EXAMPLE) {
        Ok(result) => output.push_str(&result),
        Err(e) => output.push_str(&format!("Error running computational-patterns.scm: {}\n", e)),
    }
    output.push_str("\n");
    
    let html_content = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Scheme Lisp on Fastly Compute</title>
    <style>
        body {{
            font-family: 'Courier New', monospace;
            max-width: 800px;
            margin: 0 auto;
            padding: 20px;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            min-height: 100vh;
        }}
        .container {{
            background: rgba(255, 255, 255, 0.1);
            padding: 30px;
            border-radius: 15px;
            backdrop-filter: blur(10px);
            box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
        }}
        h1 {{
            text-align: center;
            margin-bottom: 30px;
            font-size: 2.5em;
            text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.3);
        }}
        .scheme-output {{
            background: rgba(0, 0, 0, 0.3);
            padding: 20px;
            border-radius: 10px;
            border-left: 4px solid #4CAF50;
            white-space: pre-wrap;
            font-size: 1.1em;
            line-height: 1.6;
        }}
        .info {{
            margin-top: 30px;
            padding: 20px;
            background: rgba(255, 255, 255, 0.1);
            border-radius: 10px;
            border-left: 4px solid #2196F3;
        }}
        .highlight {{
            color: #4CAF50;
            font-weight: bold;
        }}
    </style>
</head>
<body>
    <div class="container">
        <h1>🚀 Scheme Lisp on Fastly Compute</h1>
        
        <div class="scheme-output">{}</div>
        
        <div class="info">
            <h3>About this demo:</h3>
            <p>This page demonstrates a simple Scheme Lisp interpreter running on <span class="highlight">Fastly Compute@Edge</span>.</p>
            <p>The Scheme code is executed server-side and the results are displayed above.</p>
            <p>This shows how you can run custom programming languages at the edge for dynamic content generation.</p>
        </div>
    </div>
</body>
</html>"#,
        output
    );
    
    Ok(Response::from_status(StatusCode::OK)
        .with_content_type(mime::TEXT_HTML_UTF_8)
        .with_body(html_content))
} 