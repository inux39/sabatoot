use std::fs::File;
use std::io::Write;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let path = std::path::Path::new(&out_dir).join("hash.rs");
    let git = std::process::Command::new("/usr/bin/git")
        .args(&["rev-parse", "HEAD"])
        .output().unwrap();
    let hash = String::from_utf8(git.stdout).unwrap();
    let output = format!("const HASH: &'static str = \"{}\";", hash.trim());
    let mut file = File::create(path).unwrap();
    file.write_all(output.as_bytes()).unwrap();
}

