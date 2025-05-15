use crate::repl::runtime;

pub fn run(input: String, runtime: &str) {
    runtime::run_file(runtime, &input, vec![]);
}