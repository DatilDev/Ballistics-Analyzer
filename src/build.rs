use std::env;

fn main() {
    let target = env::var("TARGET").unwrap_or_default();
    
    // Skip native library setup for WASM
    if target.contains("wasm32") {
        return;
    }
    
    // Force bundled SQLite for non-WASM targets
    println!("cargo:rustc-env=RUSQLITE_BUNDLED=1");
    println!("cargo:rustc-env=LIBSQLITE3_SYS_BUNDLING=1");
    
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    
    // Platform-specific linking for desktop builds
    match target_os.as_str() {
        "macos" => {
            println!("cargo:rustc-link-arg=-framework");
            println!("cargo:rustc-link-arg=CoreFoundation");
            println!("cargo:rustc-link-arg=-framework");
            println!("cargo:rustc-link-arg=Security");
        }
        "linux" => {
            println!("cargo:rustc-link-lib=pthread");
            println!("cargo:rustc-link-lib=dl");
        }
        "windows" => {
            println!("cargo:rustc-link-lib=ws2_32");
            println!("cargo:rustc-link-lib=userenv");
        }
        _ => {}
    }
}