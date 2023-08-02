use std::env;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    // Put `memory.x` in our output directory and ensure it's
    // on the linker search path.
    let out_dir = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let out_file = out_dir.join("memory.x");
    let contents = include_bytes!("memory.x");
    let mut f = std::fs::File::create(out_file).unwrap();
    f.write_all(contents).unwrap();
    // Tell the linker to look in `out_dir`
    println!("cargo:rustc-link-search={}", out_dir.display());

    // Only re-run build script if memory.x changes:
    println!("cargo:rerun-if-changed=memory.x");

    // Specify linker arguments.
    println!("cargo:rustc-link-arg=--nmagic");
    println!("cargo:rustc-link-arg=-Tlink.x");
}
