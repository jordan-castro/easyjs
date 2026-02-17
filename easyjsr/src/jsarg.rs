use std::collections::HashMap;

use anyhow::{Result, anyhow};

/// JSArg enum container
pub enum JSArg {
    Int(i32),
    Double(f64),
    String(String),
    Float(f32),
    Bool(bool),
    Int64(i64),
    Uint32(u32),
    Array(Vec<JSArg>),
    Object(HashMap<String, JSArg>),
    Null,
    Undefined,
    Uint8Array(Vec<u8>),
    Int32Array(Vec<i32>),
    Uint32Array(Vec<u32>),
    Int64Array(Vec<i64>),
    Int8Array(Vec<i8>),
    Uint16Arrat(Vec<u16>),
    Int16Arrat(Vec<i16>),
    Uint64Array(Vec<u64>),
    FloatArray(Vec<f32>),
    // Name, Message
    Exception(String, String)
}

impl JSArg {
    /// Get the int value or Error
    pub fn get_int(&self) -> Result<i32> {
        match self {
            JSArg::Int(num) => Ok(num.clone()),
            _ => Err(anyhow!("JSArg is not a Int"))
        }
    }

    /// Get the string value or Error
    pub fn get_string(&self) -> Result<String> {
        match self {
            JSArg::String(val) => Ok(val.clone()),
            _ => Err(anyhow!("JSArg is not a String"))
        }
    }

    /// Get the double value or Error.
    pub fn get_double(&self) -> Result<f64> {
        match self {
            JSArg::Double(num) => Ok(num.clone()),
            _ => Err(anyhow!("JSArg is not a Double"))
        }
    }

}