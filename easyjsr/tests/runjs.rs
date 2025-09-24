#[cfg(test)]
mod tests {
    use easyjsr::run_js;

    fn test_run() {
        assert!(run_js("1 + 1") == 0);
    }
}