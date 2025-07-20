fn main() {
    println!("cargo:rustc-link-arg=-Wl,-stack_size"); // Probably only MacOS clang / ld :/
    println!("cargo:rustc-link-arg=-Wl,0x3000000"); // This has to be commented out since for some reason `#[cfg(target_arch = "wasm32")]` doesn't seem to work
    // println!("cargo:rustc-link-arg=-zstack-size=50331648"); // WebAssembly; not used, see `.cargo/config_wasm.toml`
}
