use core::{panic, str};
use std::fs::read_dir;

pub mod commands;
pub mod repl;

use crate::commands::{compile::compile_main, install::install, repl::start_repl};
use crate::repl::runtime::run_file;

use clap::{Parser, Subcommand, Arg};
use minifier::js::minify as js_minify;

#[derive(Parser, Debug)]
#[command(name = "EasyJS", version = easyjs_utils::utils::version::VERSION_CODE, author = "Jordan Castro <jorda@grupojvm.com>")]
#[command(about = "EasyJS compiler, repl, and runner.")]
struct Args {
    /// Activate debug mode
    #[arg(short, long)]
    debug: bool,

    /// input .ej file
    ej_file: Option<String>,

    /// output .js file
    js_file: Option<String>,

    /// Minify?
    #[arg(short, long)]
    minify: bool,

    /// Pretty output?
    #[arg(short, long)]
    pretty: bool,

    /// Output compiled result to terminal
    #[arg(short, long)]
    terminal: bool,
    
    /// Runtime option
    #[arg(short, long, default_value="easyjsr")]
    runtime: String,
    
    /// Trailing arguments
    #[arg(long, allow_hyphen_values = true, num_args = 0..)]
    args: Vec<String>
}

#[derive(Debug)]
enum Commands {
    /// open the repl
    Repl,
    /// Compile the EasyJS file/project
    Compile,
    /// Run a EasyJS file/project
    Run,
    // /// Install a easyjs package
    // Install {
    //     /// The path to the .ejconfig file
    //     path_to_js_file: String,

    //     /// The directory to place the .js file
    //     #[arg(short, long, default_value = None)]
    //     forced_dir: Option<String>,
    // },
}

fn easyjs(args: Args, cmd: Commands) {
        match cmd {
            Commands::Repl => {
                start_repl(&args.runtime, false, args.debug);
            }
            Commands::Compile => {
                let file = args.ej_file.unwrap();
                // Get path.
                let ej_code_bytes: Vec<u8> = std::fs::read(&file).expect("Failed to read file.");
                let ej_code = str::from_utf8(&ej_code_bytes).expect("Unable to parse bytes.");
                let mut js_code = compile_main(ej_code.to_string(), &file);

                let extension = {
                    if args.minify {
                        ".min.js"
                    } else {
                        ".js"
                    }
                };

                if args.minify {
                    js_code = js_minify(&js_code).to_string();
                }

                // Check if we are outputing to the terminal
                if args.terminal {
                    println!("{}", js_code);

                    return;
                }

                let out_file = {
                    if let Some(output) = args.js_file {
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
            Commands::Run => {
                run_file(&args.runtime, &args.ej_file.unwrap(), args.args);
            }
            // Commands::Install {
                // path_to_js_file,
                // forced_dir,
            // } => {
                // install(path_to_js_file, forced_dir);
            // }
    }

}

fn main() {
    let args = Args::parse();

    // Repl is no ej_file exists.
    if args.ej_file.is_none() {
        easyjs(args, Commands::Repl);
        return;
    }

    // Compile is if there is a .ej and .js file
    if args.ej_file.is_some() && args.js_file.is_some() {
        easyjs(args, Commands::Compile);
        return
    }

    // Run is if there is only a .ej file
    if args.ej_file.is_some() {
        easyjs(args, Commands::Run);
        return;
    }
}
