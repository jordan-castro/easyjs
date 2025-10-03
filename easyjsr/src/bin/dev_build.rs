use std::path::Path;

fn main() {
    let include_dir = "C:/msys64/mingw64/include";
    // Gen rust bindings    
    let wrapper_header = "include/wrapper.h";
    if !Path::new(wrapper_header).exists() {
        panic!("wrapper.h not found in include/");
    }

    let bindings = bindgen::Builder::default()
        .header(wrapper_header)
        .clang_arg(format!("-I{}", include_dir))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings with bindgen");

    let out_path = "bindings.rs";
    bindings
        .write_to_file(&out_path)
        .expect("Couldn't write bindings!");

}