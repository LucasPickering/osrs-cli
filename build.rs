fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    if version_check::is_feature_flaggable().unwrap_or(false) {
        println!("cargo:rustc-cfg=nightly");
    }
}
