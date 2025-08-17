fn main() {
    // Set environment variables for bundled builds
    println!("cargo:rerun-if-changed=build.rs");
    
    // Force bundled SQLite
    println!("cargo:rustc-env=RUSQLITE_BUNDLED=1");
    println!("cargo:rustc-env=LIBSQLITE3_SYS_BUNDLING=1");
    
    // Platform-specific configurations
    let target = std::env::var("TARGET").unwrap();
    
    if target.contains("apple") {
        // macOS specific
        println!("cargo:rustc-link-arg=-framework");
        println!("cargo:rustc-link-arg=CoreFoundation");
        println!("cargo:rustc-link-arg=-framework");
        println!("cargo:rustc-link-arg=Security");
    }
    
    if target.contains("linux") {
        // Linux specific
        println!("cargo:rustc-link-lib=dylib=pthread");
        println!("cargo:rustc-link-lib=dylib=dl");
    }
    
    if target.contains("windows") {
        // Windows specific
        println!("cargo:rustc-link-lib=dylib=ws2_32");
        println!("cargo:rustc-link-lib=dylib=userenv");
    }
}