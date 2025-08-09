// easyjs/native namespaces.
// Namespaces in easyjs are for type checking and compile time function association.
// as an example take this ej code:
// file.ej
//      x = 0
// file2.ej
//      import 'file.ej'
//      import 'std' as _
//      @print(file.x)
// ^ the above will compile into
// result
//      file_x = 0
//      console.log(file_x)

use std::collections::HashMap;

use crate::typechecker::StrongValType;

#[derive(Debug, Clone)]
/// easyjs variables. Not native variables.
pub struct Variable {
    /// The name of the variable.
    pub name: String,
    /// If variable is mutable
    pub is_mut: bool,
    /// The variable type
    pub val_type: StrongValType,
}

#[derive(Debug, Clone)]
/// easyjs functions. Not native functions.
pub struct Function {
    /// The function name
    pub name: String,
    /// The function paramaters
    pub params: Vec<Variable>,
    /// The function return type
    pub return_type: StrongValType,
}

#[derive(Debug, Clone)]
/// easyjs Structs. Not native structs.
pub struct Struct {
    /// The name of the struct
    pub name: String,
    /// The constructor paramaters
    pub params: Vec<Variable>,
    /// Other variables in the struct
    pub variables: Vec<Variable>,
    /// The non static methods of the struct
    pub methods: Vec<Function>,
    /// The static methods of the struct
    pub static_methods: Vec<Function>,
}

// /// Used only in transpiler and type checker.
// /// Used to track native function calls.
// ///
// /// i.e. convert native() to __easyjs_native_call("native", ["params"], ["returns"], ...args);
// #[derive(Debug, Clone)]
// pub struct NativeFunction {
//     params: Vec<StrongValType>,
//     returns: Vec<StrongValType>,
//     name: String,
// }

/// Used only in transpiler and type checker.
/// Holds all native for project.
#[derive(Debug, Clone)]
pub struct Native {
    pub functions: Vec<Function>,
    pub variables: Vec<Variable>,
}

#[derive(Debug, Clone)]
/// easyjs namespace. File based
pub struct Namespace {
    /// The id of the namespace. i.e. filename or libname for std lib
    pub id: String,
    /// The alias of the namespace.
    pub alias: String,
    /// The variables associated with the namespace. In order to access a variable you have to use id.variable
    pub variables: Vec<Variable>,
    /// The functions associated with the namespace. In order to access a function you have to use id.function
    pub functions: Vec<Function>,
    /// The structs associated with the namespace. In order to access a struct you have to use id.struct
    pub structs: Vec<Struct>,
    /// The macros associated with the namespace. In order to access a macro you have to use id.@macro
    pub macros: HashMap<String, crate::compiler::macros::Macro>,
    /// The native context of this namespace
    pub native_ctx: Native,
}

impl Namespace {
    /// Create a new namespace.
    pub fn new(name: String, alias: String) -> Namespace {
        Namespace {
            id: name,
            alias,
            variables: vec![],
            functions: vec![],
            structs: vec![],
            macros: HashMap::new(),
            native_ctx: Native {
                functions: vec![],
                variables: vec![],
            },
        }
    }

    fn var_exits(&self, name: String) -> bool {
        self.variables.iter().any(|var| var.name == name)
    }

    fn fun_exists(&self, name: String) -> bool {
        self.functions.iter().any(|fun| fun.name == name)
    }

    fn struct_exists(&self, name: String) -> bool {
        self.structs.iter().any(|s| s.name == name)
    }

    fn macro_exists(&self, name: String) -> bool {
        self.macros.contains_key(&name)
    }

    /// Get the actual name of a object in this namespace.
    ///
    /// Works with variables, functions, structs, and macros.
    pub fn get_obj_name(&self, obj_name: &String) -> String {
        if self.alias.is_empty() {
            if self.id.is_empty() {
                obj_name.to_string()
            } else {
                format!(
                    "__{}_{}",
                    self.id.split('.').collect::<Vec<&str>>().first().unwrap(),
                    obj_name
                )
            }
        } else if self.alias == "_" {
            obj_name.to_string()
        } else {
            format!("__{}_{}", self.alias, obj_name)
        }
    }

    /// Check if this namespace has said name
    pub fn has_name(&self, name: &String) -> bool {
        if &self.alias == name || self.id.split('.').collect::<Vec<&str>>().first().unwrap() == name
        {
            true
        } else {
            false
        }
    }
}
