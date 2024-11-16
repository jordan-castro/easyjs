use boa_engine::{Context, Source, JsResult, JsValue};

pub fn interpret_js(js: &str, context: &mut Context) -> JsResult<JsValue> {
    let result = context.eval(Source::from_bytes(js))?;
    Ok(result)
}

/// Using the interpreter check if a call expression is calling a class.
pub fn is_calling_class(context: &mut Context, class_name: &str) -> bool {
    let js = format!("isClass({})", class_name);
    println!("js: {}", js);
    let result = interpret_js(&js, context);

    println!("result: {:#?}", result);

    match result {
        Ok(js_value) => {
            js_value.as_boolean().unwrap()
        },
        Err(e) => {
            println!("Error: {}", e);
            false
        }
    }
}
