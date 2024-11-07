use crate::repl::rep::start;

pub fn start_repl(runtime: &str, crash_on_error: bool, debug:bool) {
    start(runtime, crash_on_error, debug);
}