fn main() {
    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-search=./external/NetFilter_SDK/x64_capi");
    println!("cargo:rustc-link-lib=nfapi");
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    // println!("cargo:rerun-if-changed=wrapper.h");

}
