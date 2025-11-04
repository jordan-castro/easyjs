use std::{fs, io::{BufReader, Read}, path::Path};

use easyjsr::{EJR, JSArg, JSArgResult, JSArgType, JSMethod, OpaqueObject, cstr_to_string, derefernce_jsarg, jsarg_as_string, jsarg_carray, jsarg_exception, jsarg_list, jsarg_string};

use crate::{repl::builtins::exceptions, rust_error_exception};

/// File.read
fn file_read(args: Vec<JSArg>, op: &OpaqueObject) -> JSArgResult {
    // We need at least 1 arg
    if args.len() == 0 {
        return exceptions::argument_count_exception(1, 0)
    }

    let file_path = jsarg_as_string(args[0]);
    if file_path.is_none() {
        return Some(jsarg_exception("No string for file_path", "name"));
    }

    let data = std::fs::read(file_path.unwrap());
    if data.is_err() {
        let err = data.err().unwrap();
        return Some(jsarg_exception(err.to_string().as_str(), "RuntimeError"));
    }
    let data = data.expect("Could not parse");
    let mut br = BufReader::new(data.as_slice()); 
    let mut string_result = String::new();
    br.buffer().read_to_string(&mut string_result);

    Some(jsarg_string(&string_result))
}

/// Dir.read
fn dir_read(args: Vec<JSArg>, op: &OpaqueObject) -> JSArgResult {
    // We need 1 arg
    if args.len() < 1 {
        return Some(jsarg_exception("Dir path needed", "VariableCountException"));
    }
    let dir_path_var = derefernce_jsarg(&args[0]);
    if dir_path_var.type_ != JSArgType::String.c_val() {
        return Some(jsarg_exception("Dir path var type must be a string.", "WrongVariableTypeException"));
    }
    let dir_path = unsafe {
        cstr_to_string(dir_path_var.value.str_val.cast_mut())
    };
    let mut paths = vec![];
    let path = Path::new(&dir_path);
    if path.is_dir() {
        let entries = fs::read_dir(path);
        if entries.is_err() {
            let err = entries.err().unwrap();
            return rust_error_exception!(err);
            // return Some(jsarg_exception(err.to_string().as_str(), "RuntimeException"));
        }
        let entries = entries.expect("Could not get entries");
        for entry in entries {
            if entry.is_err() {
                let err = entry.err();
                return rust_error_exception!(err.unwrap());
            }
            let entry = entry.expect("Could not get entry");
            let entry_path = entry.path();
            let entry_path_str = entry_path.to_string_lossy().to_string();
            paths.push(entry_path_str);
        }
    }

    let arr = jsarg_carray(paths.iter().map(|f: &String| jsarg_string(f)).collect::<Vec<JSArg>>());
    Some(arr)
}

/// File.write
fn file_write(args: Vec<JSArg>, op: &OpaqueObject) -> JSArgResult {
    if args.len() < 3 {
        return exceptions::argument_count_exception(3, 0);
    }

    let file_path = jsarg_as_string(args[0]);
    if file_path.is_none() {
        return exceptions::jsarg_parsing_exception("file_path", "string");
    }

    let contents = jsarg_as_string(args[1]);
    if contents.is_none() {
        return exceptions::jsarg_parsing_exception("contents", "string");
    }

    let encoding = jsarg_as_string(args[2]);
    if encoding.is_none() {
        return exceptions::jsarg_parsing_exception("encoding", "string");
    }

    // Write file
    let r = std::fs::write(file_path.unwrap(), contents.unwrap());
    if r.is_err() {
        return rust_error_exception!(r.err().unwrap());
    }

    None
}

pub fn include_io(ejr: &mut EJR) {
    let module_fns = vec![
        JSMethod::new("__ejr_file_read", Box::new(file_read)),
        JSMethod::new("__ejr_file_write", Box::new(file_write)),
        JSMethod::new("__ejr_dir_read", Box::new(dir_read))
    ];
    let module_script = include_str!("../../ej/io.ej");

    ejr.register_module("ejr:io", module_fns, None);
    ejr.eval_script(module_script, "<io>");
}