use rquickjs::{Context, Function, Result};

use crate::{add_function_to_object, get_global};

/// console.log
fn log(s: String) {
    println!("{s}");
}

// console.error
fn error(msg:String) {
    println!("ERROR: {msg}");
}

/// Add the console object with methods to the globalThi objecrt
pub fn add_console(context: &Context) -> Result<()> {
    // console contents (this will run at compile time)
    let contents = include_str!("../../js/console.js");

    context.with(|ctx| -> Result<()> {
        let global = get_global!(ctx);

        // Add the method for .log
        add_function_to_object!(ctx, global, log, "___print");
        // Add the method for .error
        add_function_to_object!(ctx, global, error, "___error");

        ctx.eval::<(), _>(contents)
    })?;
    Ok(())
}
