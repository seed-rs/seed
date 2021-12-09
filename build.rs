//!
//! This build script detects if we are nightly or not
//!

extern crate version_check;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    if version_check::is_feature_flaggable() == Some(true) {
        println!("cargo:rustc-cfg=use_nightly");
    }
}
