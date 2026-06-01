// easyjs/native namespaces.
// Namespaces in easyjs are for type checking and compile time function association.
// as an example take this ej code:
// file.ej
//      x = 0
// file2.ej
//      file := import('file.ej')
//      print(file.x)
// ^ the above migth compile into
// result
//      as_21e = 0
//      console.log(as_21e)
// Variables are all mangled in the Transpiler stage.

use std::collections::HashMap;
use std::path::Path;

use easyjs_utils::utils::{h::random_hash, sanatize};

use crate::typechecker::EJType;

/// easyjs Variable Type. All variables must be of these types.
/// Any underline type can be any of these too but they must inherit from another.
/// 
/// For example if I make a type `Void` instead of `none`:
/// ```easyjs
/// Void := type none 
/// ```
/// Or a function type
/// ```easyjs
/// FooType := type fn() :: none
/// ```
/// Or a class type
/// ```easyjs
/// BarClass := type class { 
///    bar := fn() {}
/// }
/// ```
/// 
/// Now when using `Void` it is of type `none`. When using `FooType` it is of type `fn`. And `BarClass` is of type `class` with a static function 
/// named `bar` that accepts no paramaters and returns nothing.
pub enum EJVarType {
    Int(i32),
    Float(f32),
    Bool(bool),
    String(String),
    Array(Box<Vec<EJVariable>>),
    Function(Box<EJFunction>),
    Class(Box<EJClass>),
    Dynamic,
    None
}

/// Function in easyjs holds paramaters and a return type.
pub struct EJFunction {
    /// The paramaters
    params: Vec<EJVariable>,
    /// return type
    tp: EJVarType
}

/// Class in easyjs holds a list of variables.
pub struct EJClass {
    /// Variables within class
    variables: Vec<EJVariable>
}

/// A easyjs variable.
pub struct EJVariable {
    pub name: String,
    pub value: EJVarType,
    pub is_constant: bool,
    pub nullable: bool
}