use std::collections::HashMap;
use wasm_encoder::{CodeSection, Function, FunctionSection, Instruction, Module, TypeSection, ValType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EasyNativeFN {
    pub signature: FunctionSignature,
    pub function: Function,
    pub name: String,
    pub idx: u32
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FunctionSignature {
    pub params: Vec<ValType>,
    pub results: Vec<ValType>
}

#[derive(Clone)]
pub struct TypeRegistry {
    signatures: Vec<FunctionSignature>,
    lookup: HashMap<FunctionSignature, u32>,
    name_lookup: HashMap<String, u32>,
    type_lookup: HashMap<u32, Option<ValType>>
}

impl TypeRegistry {
    pub fn new() -> Self {
        TypeRegistry {
            signatures: Vec::new(),
            lookup: HashMap::new(),
            name_lookup: HashMap::new(),
            type_lookup: HashMap::new()
        }
    }

    /// Add a new signature
    pub fn add(&mut self, sig: FunctionSignature, name: String) -> u32 {
        let idx = self.signatures.len() as u32; // Always get a new index
        let result_value = {
            let results = sig.clone().results;
            results.first().cloned()
        };
        self.signatures.push(sig.clone());
        self.lookup.insert(sig, idx);  // Remove this line if you don't want deduplication
        self.name_lookup.insert(name, idx);
        self.type_lookup.insert(idx, result_value);
        idx
    }

    /// Get the return type of a function.
    pub fn get_return_type_of(&self, name: String) -> Option<ValType> {
        let idx = self.name_lookup.get(&name);
        if let Some(idx) = idx {
            *self.type_lookup.get(idx).unwrap()
        } else {
            None
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
