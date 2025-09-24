use std::env;
use std::path::PathBuf;

// file is libejr_static.a in lib/

fn main() {
    // Tell cargo to look for shared libraries in the specified directory
    let lib_dir = std::env::current_dir().unwrap().join("lib");
    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-lib=static=ejr_static");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("include/wrapper.h")
        .clang_arg("-Iinclude")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    bindings
        .write_to_file("./bindings.rs")
        .expect("Couldn't write bindings!");
}