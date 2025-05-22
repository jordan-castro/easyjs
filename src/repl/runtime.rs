use crate::commands::compile;
use easy_utils::utils;
use std::{
    io::{BufRead, BufReader, Read, Write},
    process::{Child, ChildStdout, Command, Stdio},
    thread::sleep,
    time::Duration,
};

const EASY_JS_CONSTANT: &str = "001101";

pub struct Runtime {
    /// Our JS runtime process
    process: Child,
    /// NodeJS, Deno, Bun, etc?
    runtime: String,
    /// Should we crash on error?
    _crash_on_error: bool,
    stdout_reader: BufReader<ChildStdout>,
}

impl Runtime {
    pub fn new(runtime: &str, crash_on_error: bool) -> Runtime {
        let mut p = match runtime {
            "node" => Command::new("node")
                .arg("-i")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()
                .expect("Failed to start Node.js"),
            "deno" => Command::new("deno")
                .arg("repl")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()
                .expect("Failed to start Deno"),
            _ => {
                panic!("Unknown runtime: {}", runtime);
            }
        };
        let stdout_reader = BufReader::new(p.stdout.take().expect("FAILED TO GET STDOUT"));

        let mut runtime = Runtime {
            process: p,
            runtime: runtime.to_string(),
            _crash_on_error: crash_on_error,
            stdout_reader: stdout_reader,
        };

        sleep(Duration::from_secs(1));
        runtime.send_command(&format!("const EASY_JS_CONSTANT = '{}';", EASY_JS_CONSTANT));
        runtime
    }

    pub fn send_command(&mut self, command: &str) -> Vec<String> {
        let stdin = self.process.stdin.as_mut().expect("FAILED TO GET STDIN");

        let command_with_marker = command.to_owned() + "\n EASY_JS_CONSTANT\n";
        stdin
            .write_all(command_with_marker.as_bytes())
            .expect("FAILED TO WRITE TO STDIN");
        stdin.flush().expect("FAILED TO FLUSH STDIN");

        let mut output = vec![];

        for line in self.stdout_reader.by_ref().lines() {
            let line = line.unwrap();
            if line.contains(EASY_JS_CONSTANT) {
                break;
            }
            output.push(line);
        }

        output
    }

    pub fn close(&mut self) {
        self.process.kill().expect("FAILED TO KILL PROCESS");
    }
}

/// run a ej file.
pub fn run_file(runtime: &str, path: &str, arguments: Vec<String>) {
    let input = std::fs::read_to_string(path).expect("FAILED TO READ FILE");
    let js_content = compile::compile_main(input, path);
    let js_content = format!("const EASYJS_RUNTIME='{}';\n{}", runtime, js_content);

    let js_file_path = format!("{}.js", utils::h::generate_hash(path));

    // write JS file
    std::fs::write(&js_file_path, js_content).expect("Failed to write file.");

    match runtime {
        "node" => {
            let mut child = Command::new("node")
                .arg(&js_file_path)
                .args(arguments)
                .spawn()
                .expect("FAILED TO RUN NODE");
            child.wait().expect("FAILED TO WAIT ON NODE");
            std::fs::remove_file(js_file_path).expect("FAILED TO REMOVE JS FILE");
        }
        "deno" => {
            let mut child = Command::new("deno")
                .arg(&js_file_path)
                .args(arguments)
                .spawn()
                .expect("FAILED TO RUN DENO");
            child.wait().expect("FAILED TO WAIT ON DENO");
            std::fs::remove_file(js_file_path).expect("FAILED TO REMOVE JS FILE");
        }
        "bun" => {
            let mut child = Command::new("bun")
                .arg(&js_file_path)
                .args(arguments)
                .spawn()
                .expect("FAILED TO RUN BUN");
            child.wait().expect("FAILED TO WAIT ON BUN");
            std::fs::remove_file(js_file_path).expect("FAILED TO REMOVE JS FILE");
        }
        _ => {
            println!(
                "This runtime is not currently supported. Please use (node, deno, bun) instead."
            );
        }
    }
}
