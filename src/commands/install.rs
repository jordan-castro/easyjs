use crate::utils::{
    get_exe_dir,
    reader::{read_file, read_file_from_web, write_file},
};
use std::env;

/// Install a package
pub fn install(package_path: String) {
    let binding = get_exe_dir().unwrap();
    let path = binding.to_str().unwrap();

    let file_name = package_path.split("/").last().unwrap().split('.').next().unwrap().to_owned();
    let file_contents = {
        // Let's get our shit together dog
        if package_path.contains("https://") {
            // this is a URL.
            read_file_from_web(&package_path)
        } else {
            read_file(&package_path)
        }
    };

    // save file
    write_file(format!("{}/{}/{}.js", path, file_name, file_name).as_str(), &file_contents);
    // what OS are we running?
    let os = env::consts::OS;
    match os {
        "windows" => add_windows_cmd(path, &file_name),
        "macos" => println!("Running on macOS"),
        "linux" => println!("Running on Linux"),
        _ => println!("Unknown operating system"),
    }
}

fn add_windows_cmd(path: &str, name: &str) {
    let full_path = format!("{}/{}/{}.js", path, name, name);
    let contents = format!("@echo off\nnode \"{}\" %*", full_path);

    write_file(format!("{}/{}.bat", path, name).as_str(), &contents);
}