use rquickjs::{
    CatchResultExt, Context, Function, Object, Result, Runtime, Value,
};

use crate::builtins::{console::add_console, text_encoder::add_text_encoder};

mod builtins;
mod utils;

/// Good'ole easyjs runtime. 
pub struct EasyJSR {
    pub rt: Runtime,
    pub ctx: Context
}

impl EasyJSR {
    pub fn new() -> Result<EasyJSR> {
        let rt = Runtime::new()?;
        let ctx = Context::full(&rt)?;

        let mut easyjsr = EasyJSR { rt, ctx};

        easyjsr.add_internal_methods()?;

        Ok(easyjsr)
    }

    fn add_internal_methods(&mut self) -> Result<()> {
        // add console methods
        add_console(&self.ctx)?;
        add_text_encoder(&self.ctx)?;

        Ok(())
    }

    /// Run some JS code using the easyjs runtime.
    pub fn run_js(&mut self, js: &str) -> Result<()> {
        // Ctx it
        self.ctx.with(|ctx| -> Result<()> {
            let global = ctx.globals();
            let console: Object = global.get("console")?;
            let js_log: Function = console.get("log")?;

            ctx.eval::<Value, _>(js)
                .and_then(|ret| js_log.call::<(Value<'_>,), ()>((ret,)))
                .catch(&ctx)
                .unwrap_or_else(|err| println!("{}", err));
            Ok(())
        })?;

        Ok(())
    }
}