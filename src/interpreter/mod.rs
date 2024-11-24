use boa_engine::{Context, Source, JsResult, JsValue};

pub fn interpret_js(js: &str, context: &mut Context) -> JsResult<JsValue> {
    let result = context.eval(Source::from_bytes(js))?;
    Ok(result)
}