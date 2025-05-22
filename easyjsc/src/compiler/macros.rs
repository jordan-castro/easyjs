#[derive(Debug)]
pub struct Macro {
    name: String,
    paramaters: Vec<String>,
    pub body: String
}

const DELIMITERS: &str = ".,()[]";

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

        // The logic is:
        // 1. Check for paramater position in the body
        // 2. If found, verify the character before is a '#'
        // 3. If both is true, replace '#' + paramater with arguments[i].

        for (i, param) in self.paramaters.iter().enumerate() {
            let replacement = arguments.get(i).cloned().unwrap_or_default();
            let needle = format!("#{}", param);

            // Replace all occurrences of "#param"
            body = body.replace(&needle, &replacement);
        }

        body
    }
}

