use easyjsr::{EJR};

use crate::repl::builtins::console::include_console;

pub struct EasyJSR {
    ejr: EJR,
}


impl EasyJSR {
    pub fn new() -> Self {
        let ejr = EJR::new();

        let mut s = Self {
            ejr: ejr
        };

        include_console(&mut s.ejr);

        s
    }
    pub fn run(&self, js: &str) {
        let result = self.ejr.eval_script(js, "<repl>");
        println!("result id: {}", result);

        // Print result
        let str = self.ejr.val_to_string(result);
        if let Some(str) = str {
            println!("{}", str);
            // Free JSValue
            self.ejr.free_jsvalue(result);
        }
    }
}