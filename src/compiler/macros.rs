#[derive(Debug)]
pub struct Macro {
    name: String,
    paramaters: Vec<String>,
    pub body: String
}

impl Macro {
    pub fn new(name: String, paramaters: Vec<String>, body: String) -> Macro {
        Macro { name, paramaters, body }
    }

    /// Compile a macro.
    pub fn compile(&self, arguments: Vec<String>) -> String {
        let mut body = self.body.clone();

        if arguments.len() == 0 {
            return body;
        }

        if self.paramaters.len() == 0 {
            return body;
        }

        for (i, paramater) in self.paramaters.iter().enumerate() {
            body = body.replace(paramater, &arguments[i]);
        }

        body
    }
}