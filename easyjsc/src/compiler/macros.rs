use crate::parser::ast;

#[derive(Debug, Clone)]
pub struct Macro {
    /// the name of the macro.
    name: String,
    /// The paramaters to be passed to said macro.
    pub paramaters: Vec<String>,
    /// The macro body statement.
    pub body: ast::Statement,
}

impl Macro {
    pub fn new(name: String, paramaters: Vec<String>, body: ast::Statement) -> Macro {
        Macro {
            name,
            paramaters,
            body,
        }
    }

    /// Compile a macro.
    pub fn compile(&self, arguments: Vec<String>, transpiled_body: String) -> String {
        let mut body = transpiled_body.clone();

        println!("arguments: {:#?}", arguments);

        if arguments.len() == 0 {
            return body;
        }

        if self.paramaters.len() == 0 {
            return body;
        }

        // The logic is:
        // loop through each paramater and replace it where there is a '#' infront

        for (i, param) in self.paramaters.iter().enumerate() {
            // Make sure name does not contain ...!
            let param_name = if param.contains("...") {
                param.replace("...", "")
            } else {
                param.clone()
            };
            // Make sure name does not contain key=value
            let param_name = if param.contains("=") {
                param.split("=").collect::<Vec<&str>>().get(0).unwrap().to_string()
            } else {
                param_name
            };

            let param_name = param_name.trim();

            let replacement = arguments.get(i).cloned().unwrap_or_default();
            let needle = format!("#{}", param_name);

            // Replace all occurrences of "#param"
            body = body.replace(&needle, &replacement);
        }

        body
    }
}
