// extern crate bindgen;
// use std::env;
// use std::path::PathBuf;
const LIBPATH: &str = "../build";
const LIBNAME: &str = "flashinfer_rust";
const CUDARTPATH: &str = "/usr/local/cuda/lib64";
// const WRAPPER: &str = "./csrc/c_wrapper.h";

fn main() {
    println!("cargo:rustc-link-search=native={}", LIBPATH);
    println!("cargo:rustc-link-search=native={}", CUDARTPATH);
    println!("cargo:rustc-link-lib=static={}", LIBNAME);
    println!("cargo:rustc-link-lib=cudart");
    println!("cargo:rustc-link-lib=stdc++");
    // let bindings = bindgen::Builder::default()
    //     .header(WRAPPER)
    //     .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
    //     .generate()
    //     .expect("Unable to generate bindings");
    // let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    // bindings
    //     .write_to_file(out_path.join("bindings.rs"))
    //     .expect("Couldn't write bindings!");
    // println!("cargo:rustc-rerun-if-changed={}", WRAPPER);
}


