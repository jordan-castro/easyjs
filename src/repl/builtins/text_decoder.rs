use std::sync::Arc;

use easyjsr::{cstr_to_string, derefernce_jsarg, jsarg_carray, jsarg_string, jsarg_u8_array, jsarg_undefined, JSArg, JSArgResult, JSArgType, Opaque, OpaqueObject, EJR};

use crate::repl::builtins::text_encoder::SUPPORTED_ENCODING_TYPES;

fn js_decode_text(args: Vec<JSArg>, opaque: &OpaqueObject) -> JSArgResult {
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

    // Second is actual data to decode
    let value_arg = derefernce_jsarg(&args[1]);

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
        return Some(jsarg_undefined());
    }

    // Decode
    match encoding.as_str() {
        "utf-8" => {
            if value_arg.type_ == JSArgType::UInt8Array.c_val() {
                unsafe {
                    let items = value_arg.value.u8_array_val.items;
                    let count = value_arg.value.u8_array_val.count;
                    let bytes = Vec::from_raw_parts(items.cast_mut(), count, count);
                    let (decoding, _, _) = encoding_rs::UTF_8.decode(&bytes);
                    Some(jsarg_string(&decoding.clone()))
                }
            } else {
                None
            }
        }
        _ => {None}
    }

}

pub fn include_text_decoder(ejr: &mut EJR) {
    ejr.register_callback("___ejr_text_decoder_decode", Box::new(js_decode_text), Some(Arc::new(SUPPORTED_ENCODING_TYPES)));

    let script = include_str!("../../ej/text_decoder.ej"); 

    // Compile ej script
    let js = easyjsc::compile_easy_js(script.to_string());

    let value_id = ejr.eval_script(&js, "<text_decoder>");

    if value_id < 0 {
        panic!("Could not include TextEncoder");
    }

    ejr.free_jsvalue(value_id);
}