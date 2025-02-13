use wasm_encoder::ValType;

#[derive(Clone, Debug)]
pub struct WasmVariable {
    /// the name of the variable
    pub name: String,
    /// The idx of the variable
    pub idx: u32,
    /// The type of the variable
    pub ty: ValType,
}

#[derive(Debug)]
/// A structure to assist with keeping track of variables in a wasm context.
pub struct WasmVariables {
    variables: Vec<WasmVariable>,
}

impl WasmVariables {
    pub fn new() -> Self {
        WasmVariables { variables: vec![] }
    }

    /// Add a new variable.
    pub fn add_variable(&mut self, name: String, ty: ValType) -> u32 {
        // get the idx
        let idx = self.variables.len() as u32;
        self.variables.push(WasmVariable {
            name,
            idx,
            ty,
        });
        idx
    }

    /// Get a variable by their name.
    pub fn get_variable_by_name(&self, name:&str) -> Option<WasmVariable> {
        self.variables.iter().find(|n| n.name == name).cloned()
    }

    pub fn get_variable_by_idx(&self, idx: u32) -> Option<WasmVariable> {
        self.variables.iter().find(|n| n.idx == idx).cloned()
    }
}
