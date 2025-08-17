use std::env;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=index.html");
    println!("cargo:rerun-if-changed=manifest.json");
    println!("cargo:rerun-if-changed=sw.js");
    println!("cargo:rerun-if-changed=style.css");
    
    // Set wasm-opt flags for optimization
    println!("cargo:rustc-env=WASM_OPT_FLAGS=-Oz");
    
    // Copy static files to output directory in release builds
    if env::var("PROFILE").unwrap() == "release" {
        copy_static_files();
    }
}

fn copy_static_files() {
    use std::fs;
    use std::path::Path;
    
    let out_dir = env::var("OUT_DIR").unwrap();
    let target_dir = Path::new(&out_dir)
        .parent().unwrap()
        .parent().unwrap()
        .parent().unwrap();
    
    // Create directories
    let _ = fs::create_dir_all(target_dir.join("assets"));
    
    // Copy files
    let files = vec![
        "index.html",
        "manifest.json",
        "sw.js",
        "style.css",
    ];
    
    for file in files {
        if Path::new(file).exists() {
            let _ = fs::copy(file, target_dir.join(file));
        }
    }
}