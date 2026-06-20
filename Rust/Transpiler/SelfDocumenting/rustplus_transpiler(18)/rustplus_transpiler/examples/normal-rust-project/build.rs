fn main() {
    rustplus::cargo_integration::compile_sibling_files("src")
        .expect("failed to generate Rust files from .rp files");
}
