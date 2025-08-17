use std::env;

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    
    println!("cargo:rerun-if-changed=build.rs");
    
    match target_os.as_str() {
        "android" => {
            setup_android(&target_arch);
        }
        "ios" => {
            setup_ios(&target_arch);
        }
        _ => {
            println!("cargo:warning=Building for unsupported mobile platform: {}", target_os);
        }
    }
}

fn setup_android(target_arch: &str) {
    println!("cargo:rustc-link-lib=log");
    println!("cargo:rustc-link-lib=android");
    println!("cargo:rustc-link-lib=EGL");
    println!("cargo:rustc-link-lib=GLESv2");
    
    // Set Android-specific environment variables
    if let Ok(ndk_home) = env::var("ANDROID_NDK_HOME") {
        let ndk_version = "25.2.9519653";
        let host_tag = if cfg!(target_os = "macos") {
            "darwin-x86_64"
        } else if cfg!(target_os = "windows") {
            "windows-x86_64"
        } else {
            "linux-x86_64"
        };
        
        let target_triple = match target_arch {
            "aarch64" => "aarch64-linux-android",
            "arm" => "armv7a-linux-androideabi",
            "x86" => "i686-linux-android",
            "x86_64" => "x86_64-linux-android",
            _ => panic!("Unsupported Android architecture: {}", target_arch),
        };
        
        let api_level = "21";
        
        // Set up toolchain paths
        let toolchain_path = format!("{}/toolchains/llvm/prebuilt/{}", ndk_home, host_tag);
        let sysroot = format!("{}/sysroot", toolchain_path);
        
        println!("cargo:rustc-env=CC={}/bin/{}{}-clang", toolchain_path, target_triple, api_level);
        println!("cargo:rustc-env=CXX={}/bin/{}{}-clang++", toolchain_path, target_triple, api_level);
        println!("cargo:rustc-env=AR={}/bin/llvm-ar", toolchain_path);
        println!("cargo:rustc-env=RANLIB={}/bin/llvm-ranlib", toolchain_path);
        
        // Set linker flags
        println!("cargo:rustc-link-search=native={}/usr/lib/{}", sysroot, target_triple);
        println!("cargo:rustc-link-arg=--sysroot={}", sysroot);
    } else {
        println!("cargo:warning=ANDROID_NDK_HOME not set, build may fail");
    }
    
    // JNI configuration
    println!("cargo:rustc-cfg=android_jni");
}

fn setup_ios(target_arch: &str) {
    // iOS-specific linking
    println!("cargo:rustc-link-lib=framework=UIKit");
    println!("cargo:rustc-link-lib=framework=CoreGraphics");
    println!("cargo:rustc-link-lib=framework=QuartzCore");
    println!("cargo:rustc-link-lib=framework=CoreLocation");
    println!("cargo:rustc-link-lib=framework=AVFoundation");
    println!("cargo:rustc-link-lib=framework=Photos");
    
    // Set minimum iOS version
    let min_ios_version = "12.0";
    println!("cargo:rustc-env=IPHONEOS_DEPLOYMENT_TARGET={}", min_ios_version);
    
    // Architecture-specific settings
    match target_arch {
        "aarch64" => {
            println!("cargo:rustc-cfg=ios_arm64");
        }
        "x86_64" => {
            // iOS Simulator
            println!("cargo:rustc-cfg=ios_simulator");
        }
        _ => {
            println!("cargo:warning=Unsupported iOS architecture: {}", target_arch);
        }
    }
    
    // Set up SDK paths
    if let Ok(sdk_path) = get_ios_sdk_path() {
        println!("cargo:rustc-link-search=native={}/usr/lib", sdk_path);
        println!("cargo:rustc-link-arg=-isysroot");
        println!("cargo:rustc-link-arg={}", sdk_path);
    }
}

#[cfg(target_os = "macos")]
fn get_ios_sdk_path() -> Result<String, std::io::Error> {
    use std::process::Command;
    
    let output = Command::new("xcrun")
        .args(&["--sdk", "iphoneos", "--show-sdk-path"])
        .output()?;
    
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

#[cfg(not(target_os = "macos"))]
fn get_ios_sdk_path() -> Result<String, std::io::Error> {
    Err(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "iOS SDK only available on macOS"
    ))
}