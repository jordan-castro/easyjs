use crate::parser::ast;
use easyjsr::EJR;

#[derive(Debug, Clone)]
pub struct Macro {
    /// the name of the macro.
    name: String,
    /// The paramaters to be passed to said macro.
    pub paramaters: Vec<String>,
    /// The macro body statement.
    pub body: ast::Statement,
    /// Is this macro hygenic?
    pub is_hygenic: bool
}

impl Macro {
    pub fn new(name: String, paramaters: Vec<String>, body: ast::Statement, is_hygenic: bool) -> Macro {
        Macro {
            name,
            paramaters,
            body,
            is_hygenic
        }
    }

    /// Compile a macro.
    pub fn compile(&self, arguments: Vec<String>, transpiled_body: String, ejr_ref: &mut EJR) -> String {
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

        if self.is_hygenic {
            let val = ejr_ref.eval_script(&body, format!("<{}>", self.name).as_str());
            if val == -1 {
                return String::from("");
            }

            return ejr_ref.val_to_string(val).unwrap();
        }

        body
    }
}
