use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let ch582 = env::var_os("CARGO_FEATURE_CH582").is_some();
    let ch585 = env::var_os("CARGO_FEATURE_CH585").is_some();
    let memory = match (ch582, ch585) {
        (true, false) => "memory-ch582.x",
        (false, true) => "memory-ch585.x",
        _ => panic!("rmk-ch58x requires exactly one chip feature: ch582 or ch585"),
    };

    let manifest_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    fs::copy(manifest_dir.join(memory), out_dir.join("memory.x"))
        .expect("copy selected linker memory layout");

    println!("cargo:rustc-link-search={}", out_dir.display());
    println!("cargo:rerun-if-changed=memory-ch582.x");
    println!("cargo:rerun-if-changed=memory-ch585.x");
}
