use std::collections::HashMap;
use wasm_encoder::{
    CodeSection, ConstExpr, Function, FunctionSection, Instruction, Module, TypeSection, ValType
};

use super::utils::StrongValType;

/// A function that is represented only via instructions (not via easyjs code.)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EasyNativeFN {
    pub signature: FunctionSignature,
    pub function: Function,
    pub name: String,
    pub idx: u32,
    pub is_public: bool,
}

/// Context for a native variable.
/// 
/// The value is for global variables only.
pub struct EasyNativeVar {
    /// Name of variable
    pub name: String,
    /// The idx of variable
    pub idx: u32,
    /// Whether or not the variable is global
    pub is_global: bool,
    /// The value of the variable (this is only used for global variables)
    pub value: ConstExpr, 
    /// The type of the variable
    pub val_type: StrongValType,
    /// Is the variable mutable?
    pub is_mut: bool
}

/// A signature for a function.
/// 
/// Think of a signature as (params, results)
/// 
/// When creating a signature use the FunctionSignature::create function.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FunctionSignature {
    pub params: Vec<ValType>,
    pub results: Vec<ValType>,
    pub param_strong: Vec<StrongValType>, 
    pub results_strong: Vec<StrongValType>,
}

/// Used to create the registry for functions.
/// 
/// To use call create_type_section(signatures)
#[derive(Clone)]
struct TypeRegistry {
    signatures: Vec<FunctionSignature>,
    lookup: HashMap<FunctionSignature, u32>,
    name_lookup: HashMap<String, u32>,
    type_lookup: HashMap<u32, Option<ValType>>,
    strong_type_lookup: HashMap<u32, StrongValType>,
}

/// Create the type section of our wasm module.
pub fn create_type_section(signatures: Vec<FunctionSignature>, function_section: &mut FunctionSection) -> TypeSection {
    let mut registry = TypeRegistry::new();

    for (idx, sig) in signatures.iter().enumerate() {
        let type_idx = registry.add(sig.clone(), format!("fn_{}", idx));
        function_section.function(type_idx);
    }

    registry.emit()
} 

impl TypeRegistry {
    /// Create a new registry.
    fn new() -> Self {
        TypeRegistry {
            signatures: Vec::new(),
            lookup: HashMap::new(),
            name_lookup: HashMap::new(),
            type_lookup: HashMap::new(),
            strong_type_lookup: HashMap::new(),
        }
    }

    /// Add a new signature
    fn add(&mut self, sig: FunctionSignature, name: String) -> u32 {
        let idx = self.signatures.len() as u32; // Always get a new index
        let result_value = {
            let results = sig.clone().results;
            results.first().cloned()
        };
        let sig_clone = sig.clone();
        self.signatures.push(sig.clone());
        self.lookup.insert(sig, idx); // Remove this line if you don't want deduplication
        self.name_lookup.insert(name, idx);
        self.type_lookup.insert(idx, result_value);
        if sig_clone.results_strong.len() > 0 {
            self.strong_type_lookup
                .insert(idx, sig_clone.results_strong.first().unwrap().clone());
        }
        idx
    }

    /// Get the return type of a function.
    fn get_return_type_of(&self, name: String) -> Option<ValType> {
        let idx = self.name_lookup.get(&name);
        if let Some(idx) = idx {
            *self.type_lookup.get(idx).unwrap()
        } else {
            None
        }
    }

    /// Get the strong return type of a function.
    fn get_strong_return_type_of(&self, name: String) -> Option<StrongValType> {
        let idx = self.name_lookup.get(&name);
        if let Some(idx) = idx {
            Some(self.strong_type_lookup.get(idx).unwrap().clone())
        } else {
            None
        }
    }

    /// Emits a single type section for all registered signatures.
    fn emit(&self) -> TypeSection {
        let mut types = TypeSection::new();
        for sig in &self.signatures {
            types.ty().function(sig.params.clone(), sig.results.clone());
        }
        types
        // module.section(&types);
    }
}
