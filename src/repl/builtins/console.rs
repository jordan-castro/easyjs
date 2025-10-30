use easyjsr::{cstr_to_string, derefernce_jsarg, jsarg_undefined, JSArg, JSArgResult, JSArgType, Opaque, OpaqueObject, EJR};

/// Console.log
fn ___print(args: Vec<JSArg>, opaque: &OpaqueObject) -> JSArgResult {
    if args.len() == 0 {
        return Some(jsarg_undefined());
    }

    let first = derefernce_jsarg(&args[0]);
    if first.type_ != JSArgType::String.c_val() {
        return Some(jsarg_undefined());
    }

    // We have a string in position 0
    let cstr = unsafe { first.value.str_val };
    let rs_str = cstr_to_string(cstr.cast_mut());

    // Done!
    println!("{rs_str}");

    Some(jsarg_undefined())
}

/// Console.error
fn ___error(args: Vec<JSArg>, opaque: &OpaqueObject) -> JSArgResult {
    ___print(args, opaque)
}

/// Console.warn
fn ___warn(args: Vec<JSArg>, opaque: &OpaqueObject) -> JSArgResult {
    ___print(args, opaque)
}

/// Include the globalThis.console module
pub fn include_console(ejr: &mut EJR) {
    ejr.register_callback("___ejr_print", Box::new(___print), None);
    ejr.register_callback("___ejr_error", Box::new(___error), None);
    ejr.register_callback("___ejr_warn", Box::new(___warn), None);

    let script = include_str!("../../ej/console.ej");

    // Compile ej script
    let js = easyjsc::compile_easy_js(script.to_string());

    ejr.eval_script(&js, "<console>");
}