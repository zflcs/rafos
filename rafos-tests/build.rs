fn main() {
    println!("cargo:rerun-if-changed=*/src");
    println!("cargo:rerun-if-changed=./build.rs");
    println!("cargo:rustc-link-args=-fpie -fno-builtin-function");
}
