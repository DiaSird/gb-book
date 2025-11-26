fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let crate_root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let lib_path = crate_root.join("../../SDL2-2.32.10/lib/x86");

    println!("cargo:rustc-link-search={}", lib_path.display());
    // https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-lib
    println!("cargo:rustc-link-lib=static=SDL2");
    Ok(())
}