fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    // Add an alias for `target_family = "wasm"`, to make checks easier
    // TODO figure out why the first check doesn't work
    // if cfg!(target_family = "wasm") {
    if std::env::var("TARGET").unwrap() == "wasm32-unknown-unknown" {
        println!("cargo:rustc-cfg=wasm");
    }
}
