#[cfg(test)]
mod tests {
    use easyjsr::{JSArg, JSArgResult, JSMethod, OpaqueObject, jsarg_as_int, jsarg_int};

    fn add(args: Vec<JSArg>, op: &OpaqueObject) -> JSArgResult {
        let num1 = jsarg_as_int(args[0]).unwrap();
        let num2 = jsarg_as_int(args[1]).unwrap();

        Some(jsarg_int(num1 + num2))
    }

    #[test]
    fn test_run() {
        let ejr = easyjsr::EJR::new();
        let result = ejr.eval_script("1 + 1", "<test>");
        assert!(result == 0);

        // Get string resutl
        let result_str = ejr.val_to_string(result);
        assert!(result_str.unwrap() == String::from("2"))
    }

    #[test]
    fn test_module() {
        let mut ejr = easyjsr::EJR::new();
        let module_fns = vec![
            JSMethod::new("__add", Box::new(add))
        ];

        ejr.register_module("ejr:test", module_fns, None);
        let script = r#"
        import {add} from "ejr:test";
        add(1,2);
        "#;
        let result = ejr.eval_module(script, "<test_module>");

        assert!(result > -1)
    }

}