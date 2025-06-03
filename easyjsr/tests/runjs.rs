use easyjsr::run_js;

#[cfg(test)]
mod tests {
    use easyjsr::run_js;

    fn test_run() {
        let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
        let mut has_err = false;
        if let Err(error) = runtime.block_on(run_js("./example.js")) {
            eprintln!("error: {}", error);
            has_err = true;
        }

        assert!(has_err != true);
    }
}