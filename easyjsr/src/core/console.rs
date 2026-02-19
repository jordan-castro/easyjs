use crate::{EJR, JSArg, JSArgResult, JSArgType, OpaqueObject, derefernce_jsarg, jsarg_as_string, jsarg_to_string, jsarg_undefined, utils};

/// Console.log
fn console_log(args: Vec<JSArg>, opaque: &OpaqueObject) -> JSArgResult {
    if args.len() == 0 {
        return None;
    }
    let mut msg = utils::args_to_string(args);

    println!("{msg}");

    None
}

/// console.error
fn console_error(args: Vec<JSArg>, opaque: &OpaqueObject) -> JSArgResult {
    if args.len() == 0 {
        return None;
    }

    let mut msg = utils::args_to_string(args);

    eprintln!("{msg}");

    None
}

/// console.warn
fn console_warn(args: Vec<JSArg>, opaque: &OpaqueObject) -> JSArgResult {
    console_log(args, opaque)
}

/// Include the globalThis.console module
pub fn include_console(ejr: &mut EJR) {
    ejr.register_callback("___console_log", Box::new(console_log), None);
    ejr.register_callback("___console_error", Box::new(console_error), None);
    ejr.register_callback("___console_warn", Box::new(console_warn), None);

    let script = include_str!("../../js/console.js");
    ejr.eval_script(&script, "<console>");
}