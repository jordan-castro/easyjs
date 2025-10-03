use std::sync::Arc;

use easyjsr::{cstr_to_string, derefernce_jsarg, jsarg_carray, jsarg_string, jsarg_u16_array, jsarg_u8_array, JSArg, JSArgResult, JSArgType, Opaque, OpaqueObject, EJR};

/// Supported encoding types
pub const SUPPORTED_ENCODING_TYPES: [&str; 1] = ["utf-8"];

fn js_encode_text(args: Vec<JSArg>, opaque: &OpaqueObject) -> JSArgResult {
    // We need 2 args
    if args.len() != 2 {
        return None;
    }

    // First is encoding
    let encoding_arg = derefernce_jsarg(&args[0]);
    if encoding_arg.type_ != JSArgType::String.c_val() {
        return None;
    }
    let encoding = unsafe { cstr_to_string(encoding_arg.value.str_val.cast_mut()) };

    // Second is actual string to encode
    let value_arg = derefernce_jsarg(&args[1]);
    if value_arg.type_ != JSArgType::String.c_val() {
        return None;
    }
    let value = unsafe {cstr_to_string(value_arg.value.str_val.cast_mut())};

    // Check our encoding is correct
    let encoding_options: [&str; 1];
    if let Some(user_data) = &opaque.user_data {
        if let Some(v) = user_data.downcast_ref::<[&str; 1]>() {
            encoding_options = v.clone();
        } else {
            return None;
        }
    } else {
        return None;
    }

    if !encoding_options.contains(&encoding.as_str()) {
        return None;
    }

    // Encode
    match encoding.as_str() {
        "utf-8" => {
            let (bytes, _, _) = encoding_rs::UTF_8.encode(&value);
            Some(jsarg_u8_array(bytes.to_vec()))
        }
        "utf-16" => {
            let (bytes, _, _) = encoding_rs::UTF_16BE.encode(&value);
            Some(jsarg_u8_array(bytes.to_vec()))
        }
        _ => {
            None
        }
    }
}

pub fn include_text_encoder(ejr: &mut EJR) {
    ejr.register_callback("___ejr_text_encoder_encode", Box::new(js_encode_text), Some(Arc::new(SUPPORTED_ENCODING_TYPES)));

    let script = include_str!("../../ej/text_encoder.ej");

    // Compile ej script
    let js = easyjsc::compile_easy_js(script.to_string());

    let value_id = ejr.eval_script(&js, "<text_encoder>");

    if value_id < 0 {
        panic!("Could not include TextEncoder");
    }

    ejr.free_jsvalue(value_id);
}