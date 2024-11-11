use crate::commands::compile::compile;

#[derive(Debug)]
pub struct Macro {
    name: String,
    paramaters: Vec<String>,
    body: String
}

impl Macro {
    pub fn new(name: String, paramaters: Vec<String>, body: String) -> Macro {
        Macro { name, paramaters, body }
    }

    /// Compile a macro.
    pub fn compile(&self, arguments: Vec<String>) -> String {
        let mut body = self.body.clone();

        println!("{:#?}", arguments);
        println!("{:#?}", self.paramaters);

        for (i, paramater) in self.paramaters.iter().enumerate() {
            body = body.replace(paramater, &arguments[i]);
        }

        println!("{}", body);

        let res = compile(body, false, false);
        
        println!("{}", res);

        res
    }
}