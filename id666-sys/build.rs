extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() {
    // println!("cargo:rerun-if-changed=wrapper.h");
    cc::Build::new().file("src/c/id666.c").compile("libid666.a");

    let bindings = bindgen::Builder::default()
        .header("src/c/id666.h")
        .derive_copy(true)
        .derive_debug(true)
        .derive_eq(true)
        .derive_hash(true)
        .derive_ord(true)
        .generate()
        .expect("Unable to generate bindings!");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}
