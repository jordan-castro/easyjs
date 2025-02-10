use std::collections::HashMap;
use wasm_encoder::{Module, TypeSection, FunctionSection, CodeSection, Instruction, ValType};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FunctionSignature {
    pub params: Vec<ValType>,
    pub results: Vec<ValType>
}

pub struct TypeRegistry {
    signatures: Vec<FunctionSignature>,
    lookup: HashMap<FunctionSignature, u32>
}

impl TypeRegistry {
    pub fn new() -> Self {
        TypeRegistry {
            signatures: Vec::new(),
            lookup: HashMap::new()
        }
    }

    /// add a signature
    pub fn add(&mut self, sig: FunctionSignature) -> u32 {
        if let Some(&idx) = self.lookup.get(&sig) {
            idx
        } else {
            let idx = self.signatures.len() as u32;
            self.signatures.push(sig.clone());
            self.lookup.insert(sig, idx);
            idx
        }
    }

    /// Emits a single type section for all registered signatures.
    pub fn emit(&self, module: &mut Module) {
        let mut types = TypeSection::new();
        for sig in &self.signatures {
            types.ty().function(sig.params.clone(), sig.results.clone());
        }
        module.section(&types);
    }
}
