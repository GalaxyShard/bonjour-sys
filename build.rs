use std::env;
use std::path::PathBuf;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");
    println!("cargo:rerun-if-changed=wrapper.h");

    bindgen::Builder::default()
        .header("wrapper.h")
        .blocklist_function("strtold") // not ffi safe
        // Tell cargo to invalidate the built crate whenever any of the included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}
