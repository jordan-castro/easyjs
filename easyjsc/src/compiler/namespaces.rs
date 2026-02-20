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

// Variable, function, structs, classes, etc are all mangled in the Transpiler stage.
// This includes Native Varaibles and Functions. They are mangled within the Transpiler stage.
// The reason is because the Native context is compiled all at once. (i.e. all files in one).
// Which means from the native side we don't need to use namespace.get_obj_name() unless there is some DotExpression.
// And all variables/functions/structs/classes exist in their corresponding paramaters within the NativeContext, not the Namespace.

use std::collections::HashMap;
use std::path::Path;

use easyjs_utils::utils::{h::random_hash, sanatize};

use crate::typechecker::StrongValType;

/// easyjs enums. Not native enums.
#[derive(Debug, Clone)]
pub struct EJEnum {
    /// easyjs name
    pub name: String,
}

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
    /// The easyjs enums
    pub enums: Vec<EJEnum>,
    /// The namespace hash
    pub hash: String,
    /// A alias associated
    pub alias: String,
}

impl Namespace {
    /// Create a new namespace.
    pub fn new(id: String, alias: String) -> Namespace {
        Namespace {
            id: id,
            variables: vec![],
            functions: vec![],
            structs: vec![],
            macros: HashMap::new(),
            native_ctx: Native {
                functions: vec![],
                variables: vec![],
            },
            enums: vec![],
            hash: random_hash(4),
            alias,
        }
    }

    pub fn get_var(&self, name: &str) -> Option<&Variable> {
        self.variables.iter().find(|v| v.name == name)
    }

    pub fn get_enum(&self, name: &str) -> Option<&EJEnum> {
        self.enums.iter().find(|e| e.name == name)
    }

    pub fn get_function(&self, name: &str) -> Option<&Function> {
        self.functions.iter().find(|f| f.name == name)
    }

    pub fn get_struct(&self, name: &str) -> Option<&Struct> {
        self.structs.iter().find(|s| s.name == name)
    }

    pub fn get_macro(&self, name: &str) -> Option<&crate::compiler::macros::Macro> {
        self.macros.get(name)
    }

    pub fn enum_exists(&self, name: String) -> bool {
        self.enums.iter().any(|e| e.name == name)
    }

    pub fn var_exits(&self, name: String) -> bool {
        self.variables.iter().any(|var| var.name == name)
    }

    pub fn fun_exists(&self, name: String) -> bool {
        self.functions.iter().any(|fun| fun.name == name)
    }

    pub fn struct_exists(&self, name: String) -> bool {
        self.structs.iter().any(|s| s.name == name)
    }

    pub fn macro_exists(&self, name: String) -> bool {
        self.macros.contains_key(&name)
    }

    pub fn add_name(&self, name: &str) -> String {
        format!("{}_{name}", self.hash)
    }

    pub fn get_alias(&self) -> String {
        if !self.alias.is_empty() {
            self.alias.clone()
        } else {
            Path::new(&self.id)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or_default()
                .to_string()
        }
    }

    pub fn pretty_print(&self) {
        println!(
            "===============\nid: '{}'\nalias: '{}'\nhash: '{}'\n==================",
            self.id,
            self.get_alias(),
            self.hash
        );
        // println!("|\t{}|\t{}|\t{}|", self.id, self.get_alias(), self.hash);
    }
}
