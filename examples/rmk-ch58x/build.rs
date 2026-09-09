use std::env;
use std::fs;
use std::io::Read;
use std::path::PathBuf;

use const_gen::{CompileConst, const_declaration};
use xz2::read::XzEncoder;

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

    if env::var_os("CARGO_FEATURE_VIAL").is_some() {
        let definition = fs::read_to_string(manifest_dir.join("vial.json"))
            .expect("read Vial keyboard definition");
        let compact = json::stringify(json::parse(&definition).expect("parse Vial JSON"));
        let mut compressed = Vec::new();
        XzEncoder::new(compact.as_bytes(), 6)
            .read_to_end(&mut compressed)
            .expect("compress Vial keyboard definition");
        let keyboard_id = [0x52u8, 0x4d, 0x4b, 0x43, 0x48, 0x35, 0x38, 0x58];
        let generated = [
            const_declaration!(pub VIAL_KEYBOARD_DEF = compressed),
            const_declaration!(pub VIAL_KEYBOARD_ID = keyboard_id),
        ]
        .join("\n");
        fs::write(out_dir.join("vial_generated.rs"), generated)
            .expect("write generated Vial constants");
    }

    println!("cargo:rustc-link-search={}", out_dir.display());
    println!("cargo:rerun-if-changed=memory-ch582.x");
    println!("cargo:rerun-if-changed=memory-ch585.x");
    println!("cargo:rerun-if-changed=vial.json");
}
