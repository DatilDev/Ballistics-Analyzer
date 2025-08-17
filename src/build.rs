use std::env;

fn main() {
    // Force bundled SQLite
    println!("cargo:rustc-env=RUSQLITE_BUNDLED=1");
    println!("cargo:rustc-env=LIBSQLITE3_SYS_BUNDLING=1");
    
    let target = env::var("TARGET").unwrap();
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    
    // Platform-specific linking
    match target_os.as_str() {
        "macos" => {
            println!("cargo:rustc-link-arg=-framework");
            println!("cargo:rustc-link-arg=CoreFoundation");
            println!("cargo:rustc-link-arg=-framework");
            println!("cargo:rustc-link-arg=Security");
            println!("cargo:rustc-link-arg=-framework");
            println!("cargo:rustc-link-arg=SystemConfiguration");
        }
        "linux" => {
            println!("cargo:rustc-link-lib=pthread");
            println!("cargo:rustc-link-lib=dl");
            println!("cargo:rustc-link-lib=m");
        }
        "windows" => {
            println!("cargo:rustc-link-lib=ws2_32");
            println!("cargo:rustc-link-lib=userenv");
            println!("cargo:rustc-link-lib=bcrypt");
        }
        _ => {}
    }
    
    // Handle secp256k1 vendoring
    if !target.contains("wasm32") {
        cc::Build::new()
            .define("ECMULT_GEN_PREC_BITS", "4")
            .define("ECMULT_WINDOW_SIZE", "15")
            .define("ENABLE_MODULE_RECOVERY", "1")
            .define("USE_EXTERNAL_DEFAULT_CALLBACKS", "1")
            .flag_if_supported("-std=c99")
            .compile("secp256k1");
    }
}