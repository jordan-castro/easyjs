use git2::Repository;
use tempfile::tempdir;
use std::path::Path;
use crate::compile_easy_js;
use crate::utils::ej_config::{get_ej_config, parse_ej_config, EJConfig};

use crate::utils::{
    get_exe_dir,
    reader::{read_file, read_file_from_web, write_file},
};
use std::env;

use super::compile;

fn install_easyjs_pkg(package_path: &str) {
    let binding = get_exe_dir().unwrap();
    let path = binding.to_str().unwrap();

    let file_path = get_ej_config(package_path);
    // check we only have 1
    if file_path.len() > 1 || file_path.len() == 0 {
        panic!("We could not find the config");
    }

    let config = parse_ej_config(file_path[0].clone()).expect("Could not parse EJConfig file.");

    let pkg_name = config.name;
    let pkg_output = config.output;
    let pkg_source = config.source;
    // let pkg_runtime = config.runtime;

    // Open the contents of output if exists
    let mut contents = read_file(format!("{}/{}", package_path, pkg_output).as_str());
    if contents == "" {
        // we have to get source instead and convert it.
        let source_contents = read_file(format!("{}/{}", package_path, pkg_source).as_str());
        let compiled_easyjs = compile_easy_js(source_contents);
        contents = compiled_easyjs;
    }

    // save file
    write_file(format!("{}/{}/{}.js", path, pkg_name, pkg_name).as_str(), &contents);
    // what OS are we running?
    let os = env::consts::OS;
    match os {
        "windows" => add_windows_cmd(path, &pkg_name),
        "macos" => println!("Running on macOS"),
        "linux" => println!("Running on Linux"),
        _ => println!("Unknown operating system"),
    }

}

/// Install a package
pub fn install(package_path: String) {

    if package_path.contains("https://") {
        // This shold be a URL
        if !package_path.ends_with(".git") {
            panic!("Cant install a HTTPS easyjs without .git");
        }

        // create a temporary path
        let temp_dir = tempdir().expect("Could not create a temp directory.");

        // We have a git path
        let repo = Repository::clone(&package_path, temp_dir.path()).expect("Could not clone git repo.");

        // Boom cloned,
        install_easyjs_pkg(temp_dir.path().to_str().unwrap());
    } else {
        // check if a regular file
        if package_path.ends_with(".ejconfig") {
            // regular file get directory
            let path = Path::new(&package_path);
            let dir = path.parent().expect("Could not get parent directory");
            install_easyjs_pkg(dir.to_str().unwrap());
        } else {
            // this should be a directory then... pass directly
            install_easyjs_pkg(&package_path);
        }
    }

}

fn add_windows_cmd(path: &str, name: &str) {
    let full_path = format!("{}/{}/{}.js", path, name, name);
    let contents = format!("@echo off\nnode \"{}\" %*", full_path);

    write_file(format!("{}/{}.bat", path, name).as_str(), &contents);
}