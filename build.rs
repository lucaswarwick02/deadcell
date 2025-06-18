use std::path::PathBuf;

fn main() {
    let dir = PathBuf::from("vendor/tree-sitter-python");
    println!("cargo:rerun-if-changed={}", dir.display());

    cc::Build::new()
        .include(&dir)
        .file(dir.join("src/parser.c"))
        .file(dir.join("src/scanner.c"))
        .compile("tree-sitter-python");
}
