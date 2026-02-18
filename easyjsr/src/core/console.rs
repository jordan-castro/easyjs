use crate::{EJR, JSArg, JSArgResult, JSArgType, OpaqueObject, derefernce_jsarg, jsarg_as_string, jsarg_to_string, jsarg_undefined};

/// Console.log
fn console_log(args: Vec<JSArg>, opaque: &OpaqueObject) -> JSArgResult {
    if args.len() == 0 {
        return None;
    }
    let mut msg = String::new();

    for arg in args {
        let jsarg = unsafe { derefernce_jsarg(&arg) };
        
        if jsarg.type_ != JSArgType::String as u32 {
            // Convert into a string
            let res = jsarg_to_string(arg);
            if let Some(res) = res {
                msg.push_str(res.as_str());
            }
        } else {
            msg.push_str(jsarg_as_string(arg).unwrap().as_str());
        }
    }

    println!("{msg}");

    None
}

/// Include the globalThis.console module
pub fn include_console(ejr: &mut EJR) {
    ejr.register_callback("___console_log", Box::new(console_log), None);

    let script = include_str!("../../js/console.js");
    ejr.eval_script(&script, "<console>");
}