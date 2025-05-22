
pub enum EasyError{
    Transpiler(String),
    UnsupportedType(String),
    Expected(String),
    NotSupported(String)
}