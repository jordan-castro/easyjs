use rquickjs::{prelude::Func, CatchResultExt, Context, Function, Module, Object, Persistent, Result, Runtime, Value};

use crate::{
    builtins::{
        console::add_console, text_decoder::add_text_decoder, text_encoder::add_text_encoder,
    },
    modules::set_easyjsr_module_loader,
};

mod builtins;
mod modules;
pub mod utils;

/// For power users, if you need access to the lower level quickjs apis.
pub mod raw {
    pub use rquickjs::*;
}

/// Good'ole easyjs runtime.
pub struct EasyJSR {
    pub rt: Runtime,
    pub ctx: Context,
}

impl EasyJSR {
    pub fn new() -> Result<EasyJSR> {
        let rt = Runtime::new()?;
        let ctx = Context::full(&rt)?;

        let mut easyjsr = EasyJSR { rt, ctx };

        easyjsr.add_internal_methods()?;

        Ok(easyjsr)
    }

    fn add_internal_methods(&mut self) -> Result<()> {
        // add modules
        set_easyjsr_module_loader(&self.rt);

        // add console methods
        add_console(&self.ctx)?;
        add_text_encoder(&self.ctx)?;
        add_text_decoder(&self.ctx)?;

        Ok(())
    }

    /// Run some JS code using the easyjs runtime.
    pub fn run(&mut self, js: &str) -> Result<()> {
        // Ctx it
        self.ctx.with(|ctx| -> Result<()> {
            let global = ctx.globals();
            let console: Object = global.get("console")?;
            let js_log: Function = console.get("log")?;

            Module::evaluate(ctx.clone(), "easyjs", js)
                .unwrap()
                .finish()
                .and_then(|ret: Value| js_log.call::<(Value<'_>,), ()>((ret.into(),)))
                .catch(&ctx)
                .unwrap_or_else(|err| println!("{}", err));
            Ok(())
        })?;

        Ok(())
    }

}
