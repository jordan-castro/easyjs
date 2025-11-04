use std::error::Error;

use easyjsr::{JSArg, JSArgResult, JSArgType, jsarg_exception};

pub fn argument_count_exception(expected: i32, provided: i32) -> JSArgResult {
    let msg = format!("Functions expects {expected} arguments, only {provided} were provided");
    Some(jsarg_exception(msg.as_str(), "ArgumentCountException"))
}

pub fn jsarg_parsing_exception(var_name: &str, expected_type: &str) -> JSArgResult {
    let msg = format!("Could not parse: {var_name}, should be {expected_type}");
    Some(jsarg_exception(msg.as_str(), "JSArgParsingException"))
}

#[macro_export]
macro_rules! rust_error_exception {
    ($error:expr) => {{
        Some(jsarg_exception($error.to_string().as_str(), "RustErrorException"))
    }};
}
