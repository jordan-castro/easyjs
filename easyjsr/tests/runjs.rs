#[cfg(test)]
mod tests {
    #[test]
    fn test_run() {
        let ejr = easyjsr::EJR::new();
        let result = ejr.eval_script("1 + 1", "<test>");
        assert!(result == 0);

        // Get string resutl
        let result_str = ejr.val_to_string(result);
        assert!(result_str == String::from("2"))
    }
}