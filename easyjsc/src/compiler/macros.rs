use crate::parser::ast;

#[derive(Debug, Clone)]
pub struct Macro {
    name: String,
    paramaters: Vec<String>,
    pub body: ast::Statement
}

const DELIMITERS: &str = ".,()[]";

impl Macro {
    pub fn new(name: String, paramaters: Vec<String>, body: ast::Statement) -> Macro {
        Macro { name, paramaters, body }
    }

    /// Compile a macro.
    pub fn compile(&self, arguments: Vec<String>, transpiled_body: String) -> String {
        let mut body = transpiled_body.clone();

        if arguments.len() == 0 {
            return body;
        }

        if self.paramaters.len() == 0 {
            return body;
        }

        // The logic is:
        // loop through each paramater and replace it where there is a '#' infront

        for (i, param) in self.paramaters.iter().enumerate() {
            let replacement = arguments.get(i).cloned().unwrap_or_default();
            let needle = format!("#{}", param);

            // Replace all occurrences of "#param"
            body = body.replace(&needle, &replacement);
        }

        body
    }
}

