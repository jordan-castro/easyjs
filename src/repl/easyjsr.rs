use easyjsr::{EJR, derefernce_jsarg, jsarg_as_string};

use crate::repl::builtins::{console::include_console, text_decoder::include_text_decoder, text_encoder::include_text_encoder, io::include_io};

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
        include_text_encoder(&mut s.ejr);
        include_text_decoder(&mut s.ejr);
        include_io(&mut s.ejr);

        s
    }

    pub fn run(&self, js: &str) {
        let result = self.ejr.eval_script(js, "<repl>");

        // Print result
        let str = self.ejr.val_to_string(result);
        if let Some(str) = str {
            println!("{}", str);
        }
        // Free JSValue
        self.ejr.free_jsvalue(result);
    }

    pub fn run_file(&self, js_content: &str, file_name: &str) {
        let result = self.ejr.eval_module(js_content, file_name);

        // TODO: ejr_await_promise

        // Print result
        let str = self.ejr.val_to_string(result);
        self.ejr.free_jsvalue(result);
        // if let Some(str) = str {
        //     println!("{str}");
        //     // Free val
        //     self.ejr.free_jsvalue(result);
        // } else {
        //     println!("Nigga");
        // }
    }
}