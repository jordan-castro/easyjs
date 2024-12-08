use boa_engine::{Context, Source, JsResult, JsValue};

pub fn interpret_js(js: &str, context: &mut Context) -> JsResult<JsValue> {
    let result = context.eval(Source::from_bytes(js))?;
    Ok(result)
}

/// Check if a JS variable is already defined
pub fn is_javascript_var_defined(var: &str, context: &mut Context) -> bool {
    // Proper JavaScript to check if the variable is defined
    let js_code = format!(
        "typeof {} !== 'undefined'",
        var // Escape the variable name if needed
    );

    match interpret_js(&js_code, context) {
        Ok(JsValue::Boolean(v)) => {
            v
        }
        Ok(_) => {
            println!("Unexpected non-boolean result for '{}'", var);
            false
        }
        Err(e) => {
            println!("Error evaluating '{}': {}", var, e);
            false
        }
    }
}