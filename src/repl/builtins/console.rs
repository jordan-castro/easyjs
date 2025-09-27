use easyjsr::{cstr_to_string, derefernce_jsarg, JSArg, JSArgResult, JSArgType, Opaque, EJR};

/// Console.log
fn ___print(args: Vec<JSArg>, opaque: Opaque) -> JSArgResult {
    if args.len() == 0 {
        return None;
    }

    println!("Here 1");

    let first = derefernce_jsarg(&args[0]);
    if first.type_ != JSArgType::String.c_val() {
        return None;
    }
    println!("Here 2");

    // We have a string in position 0
    let cstr = unsafe { first.value.str_val };
    let rs_str = cstr_to_string(cstr.cast_mut());
    println!("Here 3");

    // Done!
    println!("{rs_str}");

    None
}

/// Console.error
fn ___error(args: Vec<JSArg>, opaque: Opaque) -> JSArgResult {
    ___print(args, opaque)
}

/// Console.warn
fn ___warn(args: Vec<JSArg>, opaque: Opaque) -> JSArgResult {
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