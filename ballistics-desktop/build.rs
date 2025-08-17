use std::env;
use std::path::PathBuf;

fn main() {
    // Rerun build script if it changes
    println!("cargo:rerun-if-changed=build.rs");
    
    // Get target OS
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    
    println!("cargo:warning=Building for OS: {}, Arch: {}", target_os, target_arch);
    
    // Platform-specific configuration
    match target_os.as_str() {
        "windows" => {
            setup_windows();
        }
        "macos" => {
            setup_macos();
        }
        "linux" => {
            setup_linux();
        }
        _ => {
            println!("cargo:warning=Unknown target OS: {}", target_os);
        }
    }
    
    // Copy assets to output directory
    copy_assets();
    
    // Setup icon for Windows
    #[cfg(windows)]
    embed_windows_icon();
}

fn setup_windows() {
    // Windows-specific build configuration
    println!("cargo:rustc-cfg=windows");
    
    // Link Windows libraries if needed
    println!("cargo:rustc-link-lib=user32");
    println!("cargo:rustc-link-lib=gdi32");
    
    // Set Windows subsystem for release builds
    #[cfg(not(debug_assertions))]
    {
        println!("cargo:rustc-link-arg=/SUBSYSTEM:WINDOWS");
        println!("cargo:rustc-link-arg=/ENTRY:mainCRTStartup");
    }
}

fn setup_macos() {
    // macOS-specific build configuration
    println!("cargo:rustc-cfg=macos");
    
    // Link macOS frameworks if needed
    println!("cargo:rustc-link-lib=framework=CoreGraphics");
    println!("cargo:rustc-link-lib=framework=AppKit");
    
    // Set minimum macOS version
    println!("cargo:rustc-env=MACOSX_DEPLOYMENT_TARGET=10.14");
}

fn setup_linux() {
    // Linux-specific build configuration
    println!("cargo:rustc-cfg=linux");
    
    // Only use pkg_config if it's available as a dependency
    #[cfg(feature = "use-pkg-config")]
    setup_linux_with_pkg_config();
    
    // Fallback for when pkg_config is not available
    #[cfg(not(feature = "use-pkg-config"))]
    setup_linux_fallback();
}

#[cfg(feature = "use-pkg-config")]
fn setup_linux_with_pkg_config() {
    #[cfg(not(target_env = "musl"))]
    {
        // Use pkg-config to find system libraries
        if let Ok(lib) = pkg_config::probe_library("gtk+-3.0") {
            for path in &lib.link_paths {
                println!("cargo:rustc-link-search=native={}", path.display());
            }
        }
        
        // Additional libraries that might be needed
        if pkg_config::probe_library("x11").is_ok() {
            println!("cargo:rustc-link-lib=X11");
        }
        
        if pkg_config::probe_library("xcb").is_ok() {
            println!("cargo:rustc-link-lib=xcb");
        }
    }
}

#[cfg(not(feature = "use-pkg-config"))]
fn setup_linux_fallback() {
    // Fallback: try standard library paths
    println!("cargo:rustc-link-search=/usr/lib");
    println!("cargo:rustc-link-search=/usr/lib/x86_64-linux-gnu");
    println!("cargo:rustc-link-search=/usr/local/lib");
    
    // Link common libraries
    println!("cargo:rustc-link-lib=X11");
    println!("cargo:rustc-link-lib=xcb");
    println!("cargo:rustc-link-lib=xkbcommon");
    
    // For static linking on Alpine/musl
    #[cfg(target_env = "musl")]
    {
        println!("cargo:rustc-link-lib=static=X11");
        println!("cargo:rustc-link-lib=static=xcb");
    }
}

fn copy_assets() {
    // Copy assets to output directory for easy access
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = PathBuf::from(out_dir);
    let assets_path = out_path.parent().unwrap().parent().unwrap().parent().unwrap();
    
    // Create assets directory in target
    let target_assets = assets_path.join("assets");
    std::fs::create_dir_all(&target_assets).ok();
    
    // Copy icon files
    let icon_files = vec![
        "icon.png",
        "icon.ico",
        "Roboto-Regular.ttf",
    ];
    
    for file in icon_files {
        let src = PathBuf::from("assets").join(file);
        if src.exists() {
            let dst = target_assets.join(file);
            std::fs::copy(&src, &dst).ok();
            println!("cargo:warning=Copied asset: {}", file);
        } else {
            println!("cargo:warning=Asset not found: {}", file);
        }
    }
}

#[cfg(windows)]
fn embed_windows_icon() {
    // Only try to use winres if it's available
    #[cfg(feature = "embed-resource")]
    {
        let mut res = winres::WindowsResource::new();
        
        // Set icon
        if PathBuf::from("assets/icon.ico").exists() {
            res.set_icon("assets/icon.ico");
        }
        
        // Set version info
        res.set("ProductName", "Ballistics Analyzer");
        res.set("CompanyName", "Ballistics Analyzer Contributors");
        res.set("FileDescription", "Professional Ballistics Calculator");
        res.set("LegalCopyright", "Copyright (c) 2025");
        
        // Set version from Cargo.toml
        if let Ok(version) = env::var("CARGO_PKG_VERSION") {
            res.set("ProductVersion", &version);
            res.set("FileVersion", &version);
        }
        
        // Compile the resource
        if let Err(e) = res.compile() {
            println!("cargo:warning=Failed to compile Windows resources: {}", e);
        }
    }
}

#[cfg(not(windows))]
fn embed_windows_icon() {
    // No-op on non-Windows platforms
}
