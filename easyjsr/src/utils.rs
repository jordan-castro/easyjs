use crate::{JSArg, JSArgType, derefernce_jsarg, jsarg_as_string, jsarg_to_string};

/// Convert a Vec<JSArg> into strings.
pub fn args_to_string(args: Vec<JSArg>) -> String {
    let mut str = String::new();
    for arg in args {
        let darg = derefernce_jsarg(&arg);
        if darg.type_ == JSArgType::String as u32 {
            str.push_str(&jsarg_as_string(arg).unwrap());
        } else {
            str.push_str(&jsarg_to_string(arg).unwrap());
        }
        str.push_str(" ");
    }
    str.remove(str.len() - 1);

    str
}
