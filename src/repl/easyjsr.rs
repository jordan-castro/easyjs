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

        println!("Including console..");
        include_console(&mut s.ejr);

        s
    }
    pub fn run(&self, js: &str) {
        let result = self.ejr.eval_script(js, "<repl>");
        
        // Print result
        println!("{}", self.ejr.val_to_string(result));

        // Free JSValue
        self.ejr.free_jsvalue(result);
    }
}