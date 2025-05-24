// use crate::compile_easy_js;
use easyjsc::compile_easy_js;
use easy_utils::utils::ej_config::{get_ej_config, parse_ej_config, EJConfig};
use git2::Repository;
use std::path::Path;
use tempfile::tempdir;

use easy_utils::utils::{
    get_exe_dir,
    reader::{read_file, write_file},
};
use std::env;

use super::compile;

fn install_easyjs_pkg(package_path: &str, forced_dir: Option<String>) {
    let path = if let Some(ref forced_dir) = forced_dir {
        forced_dir.clone()
    } else {
        let binding = get_exe_dir().unwrap();
        binding.to_str().unwrap().to_string()
    };

    let file_path = get_ej_config(package_path);
    // check we only have 1
    if file_path.len() > 1 || file_path.len() == 0 {
        panic!("We could not find the config");
    }

    let config = parse_ej_config(file_path[0].clone()).expect("Could not parse EJConfig file.");

    let pkg_name = &config.name;
    let pkg_output = &config.output;
    let pkg_source = &config.source;
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
    write_file(
        format!("{}/{}/{}", path, pkg_name, pkg_output).as_str(),
        &contents,
    );
    // Write the config file
    write_file(
        format!("{}/{}/pkg.ejconfig", path, pkg_name).as_str(),
        &serde_json::to_string(&config).unwrap(),
    );

    if forced_dir.is_none() {
        // what OS are we running?
        let os = env::consts::OS;
        match os {
            "windows" => add_windows_cmd(&path, &pkg_name, &pkg_output),
            "macos" => println!("Running on macOS"),
            "linux" => println!("Running on Linux"),
            _ => println!("Unknown operating system"),
        }
    }
}

/// Install a package
pub fn install(package_path: String, forced_dir: Option<String>) {
    if package_path.contains("https://") {
        // This shold be a URL
        if !package_path.ends_with(".git") {
            panic!("Cant install a HTTPS easyjs without .git");
        }

        // create a temporary path
        let temp_dir = tempdir().expect("Could not create a temp directory.");

        // We have a git path
        let repo =
            Repository::clone(&package_path, temp_dir.path()).expect("Could not clone git repo.");

        // Boom cloned,
        install_easyjs_pkg(temp_dir.path().to_str().unwrap(), forced_dir);
    } else {
        // check if a regular file
        if package_path.ends_with(".ejconfig") {
            // regular file get directory
            let path = Path::new(&package_path);
            let dir = path.parent().expect("Could not get parent directory");
            install_easyjs_pkg(dir.to_str().unwrap(), forced_dir);
        } else {
            // this should be a directory then... pass directly
            install_easyjs_pkg(&package_path, forced_dir);
        }
    }
}

fn add_windows_cmd(path: &str, name: &str, output: &str) {
    let full_path = format!("{}/{}/{}", path, name, output);
    let contents = format!("@echo off\nnode \"{}\" %*", full_path);

    write_file(format!("{}/{}.bat", path, name).as_str(), &contents);
}
