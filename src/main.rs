use core::{panic, str};
use std::fs::read_dir;

pub mod commands;
pub mod interpreter;
pub mod repl;

use crate::commands::{repl::start_repl, install::install, compile::compile_main};
use crate::repl::runtime::run_file;

use clap::{Parser, Subcommand};
use minifier::js::minify as js_minify;

#[derive(Parser, Debug)]
#[command(name = "EasyJS", version = easy_utils::utils::version::VERSION_CODE, author = "Jordan Castro <jorda@grupojvm.com>")]
#[command(about = "EasyJS compiler, repl, and runner.")]
/// Activate debug mode
struct Args {
    #[arg(short, long)]
    debug: bool,

    /// Subcommand to run
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// open the repl
    Repl {
        /// The runtime for the repl
        #[arg(short, long, default_value = "node")]
        runtime: String,

        #[arg(short, long, action)]
        debug: bool,
    },
    /// Compile the EasyJS file/project
    Compile {
        /// The file path
        file: String,

        /// Pretty output?
        #[arg(short, long, action)]
        pretty: bool,

        /// Minify the output
        #[arg(short, long, action)]
        minify: bool,

        /// New file name
        #[arg(short, long, default_value = None)]
        output: Option<String>,

        /// Output to the terminal.
        #[arg(short, long, action)]
        terminal: bool
    },
    /// Run a EasyJS file/project
    Run {
        /// The file path
        file: String,

        /// The runtime to use.
        #[arg(short, long, default_value = "node")]
        runtime: String,

        /// Trailing arguments
        #[arg(trailing_var_arg = true)]
        #[arg(num_args=0..)]
        args: Vec<String>
    },
    /// Install a easyjs package
    Install {
        /// The path to the .ejconfig file
        path_to_js_file: String,

        /// The directory to place the .js file
        #[arg(short, long, default_value = None)]
        forced_dir: Option<String>
    }
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Repl { runtime, debug } => {
            start_repl(&runtime, false, debug);
        }
        Commands::Compile {
            file,
            pretty,
            minify,
            output,
            terminal
        } => {
            // Get path.
            let ej_code_bytes: Vec<u8> = std::fs::read(&file).expect("Failed to read file.");
            let ej_code = str::from_utf8(&ej_code_bytes).expect("Unable to parse bytes.");
            let mut js_code = compile_main(ej_code.to_string(), &file);

            let extension = {
                if minify {
                    ".min.js"
                } else {
                    ".js"
                }
            };

            if minify {
                js_code = js_minify(&js_code).to_string();
            }

            // Check if we are outputing to the terminal
            if terminal {
                println!("{}", js_code);

                return;
            }

            let out_file = {
                if let Some(output) = output {
                    output
                } else {
                    file.replace(".ej", &extension)
                }
            };
            let out_file = out_file
                .replace("\\", "/")
                .split("/")
                .collect::<Vec<_>>()
                .last()
                .unwrap()
                .to_string();

            // write to file
            std::fs::write(out_file, js_code).expect("Filed to write file.");
        }
        Commands::Run { file, runtime , args} => {
            run_file(&runtime, &file, args);
        }
        Commands::Install { path_to_js_file , forced_dir} => {
            install(path_to_js_file, forced_dir);
        }
    }
}
