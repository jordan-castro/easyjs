use std::fs;
use std::path::{Path, PathBuf};
use std::env;

use bindgen::CargoCallbacks;

/// Collect all files with a given extension in a directory (non-recursive).
fn collect_files(dir: &Path, ext: &str) -> Vec<PathBuf> {
    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some(ext) {
                files.push(path);
            }
        }
    }
    files
}

fn build_bindings() {
    let bindings = bindgen::Builder::default()
        .header("ejr_lib/include/ejr.h")
        .parse_callbacks(Box::new(CargoCallbacks::new()))
        .generate()
        .expect("Could not generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("ejr_bindings.rs")).expect("Could not write bindings.");
}

fn main() {
    build_bindings();
    let manifest_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    
    // Paths under ejr_lib
    let ejr_root   = manifest_dir.join("ejr_lib");
    let include_dir = ejr_root.join("include");
    let src_dir     = ejr_root.join("src");
    let lib_dir     = ejr_root.join("lib");

    // Collect source files
    let cpp_sources = collect_files(&src_dir, "cpp");
    let c_sources = collect_files(&lib_dir, "c");

    if cpp_sources.is_empty() && c_sources.is_empty() {
        panic!("No source files found in ejr_lib/src or ejr_lib/lib.");
    }

    for src in cpp_sources.iter().chain(c_sources.iter()) {
        println!("cargo:rerun-if-changed={}", src.display());
    }

    if let Ok(entries) = fs::read_dir(&include_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("h") {
                println!("cargo:rerun-if-changed={}", path.display());
            }
        }
    }

    // Compile C first
    if !c_sources.is_empty() {
        let mut c_build = cc::Build::new();
        c_build.warnings(false);
        c_build.files(&c_sources);
        c_build.include(&include_dir);
        c_build.include(&lib_dir);
        c_build.flag_if_supported("-O2");
        c_build.flag_if_supported("-Wall");
        c_build.flag_if_supported("-fPIC");
        c_build.compile("ejr_c"); // static library for C files
    }

    // Compile C++
    if !cpp_sources.is_empty() {
        let mut cpp_build = cc::Build::new();
        cpp_build.cpp(true);
        cpp_build.files(&cpp_sources);
        cpp_build.include(&include_dir);
        cpp_build.include(&src_dir);
        cpp_build.include(&lib_dir);
        cpp_build.include(&ejr_root);
        cpp_build.flag_if_supported("-std=c++17");
        cpp_build.flag_if_supported("-O2");
        cpp_build.flag_if_supported("-Wall");
        cpp_build.flag_if_supported("-fPIC");

        // Link the previously compiled C library
        cpp_build.compile("ejr_cpp"); // static library for C++ files
    }

    // Linker instructions
    println!("cargo:rustc-link-lib=static=ejr_cpp");
    println!("cargo:rustc-link-lib=static=ejr_c");

    #[cfg(unix)]
    {
        println!("cargo:rustc-link-lib=m");
        println!("cargo:rustc-link-lib=dl");
        println!("cargo:rustc-link-lib=pthread");
    }

    println!("cargo:rerun-if-changed=src/lib.rs");
}
