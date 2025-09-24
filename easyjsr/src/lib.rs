use crate::ffi::EJR;

mod ffi;

pub fn run_js(js: &str) -> i32 {
    let ejr = EJR::new();

    // Run some JS
    let value = ejr.eval_script(js, "<test>");
    // ejr gets freed here...

    return value;
}

// use crate::{
//     builtins::{
//         console::add_console, text_decoder::add_text_decoder, text_encoder::add_text_encoder,
//     },
//     modules::set_easyjsr_module_loader,
// };

// mod builtins;
// mod modules;
// pub mod utils;

// /// For power users, if you need access to the lower level quickjs apis.
// pub mod raw {
//     pub use rquickjs::*;
// }

// /// Good'ole easyjs runtime.
// pub struct EasyJSR {
//     pub rt: Runtime,
//     pub ctx: Context,
// }

// impl EasyJSR {
//     pub fn new() -> Result<EasyJSR> {
//         let rt = Runtime::new()?;
//         let ctx = Context::full(&rt)?;

//         let easyjsr = EasyJSR { rt, ctx };

//         Ok(easyjsr)
//     }

//     /// Optionally add the internal methods in builtins/
//     pub fn add_internal_methods(&mut self) -> Result<()> {
//         // add console methods
//         add_console(&self.ctx)?;
//         add_text_encoder(&self.ctx)?;
//         add_text_decoder(&self.ctx)?;

//         Ok(())
//     }

//     /// Optionally add support for modules
//     pub fn add_support_for_modules(&mut self) {
//         set_easyjsr_module_loader(&self.rt);
//     }

//     /// Run some JS code using the easyjs runtime.
//     /// 
//     /// Returns a String result.
//     pub fn eval(&mut self, js: &str) -> Result<String> {
//         // Ctx it
//         let result = self.ctx.with(|ctx| -> Result<String> {
//             let promise = Module::evaluate(ctx.clone(), "<eval>", js)?;
//             let result = promise.finish::<Value>()?;

//             Ok(convert_value_to_string!(result))
//         })?;

//         Ok(result)
//     }

//     /// Call a specific function from a object in the runtime.
//     pub fn call_function(&mut self, object_name: &str, fn_name: &str, args: Args) where Args: IntoArgs<'js>, Ret: FromJs {
//         self.ctx.with(|ctx| -> Result<()> {
//             let globals = ctx.globals();
//             let object: Object = globals.get(object_name)?;
//             let function: Function = object.get(fn_name)?;

//             function.call(args);

//             Ok(())
//         });
//     }
// }
