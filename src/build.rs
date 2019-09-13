use std::fs::File;
use std::io::Write;

fn main() {
    // TODO: unwrap()
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let path = std::path::Path::new(&out_dir).join("hash.rs");
    let git = std::process::Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .output()
        .expect("failed call git command");
    let hash = match String::from_utf8(git.stdout) {
        Ok(o) => o,
        Err(_) => String::new(),
    };
    let output = format!("const HASH: &'static str = \"{}\";", hash.trim());
    let mut file = File::create(path).unwrap();
    file.write_all(output.as_bytes()).unwrap();
}

