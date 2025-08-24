#![cfg(target_os = "android")]

use android_activity::AndroidApp;
use egui::Context;
use std::sync::Arc;

mod app;
mod storage_handler;
mod ui;
mod platform;

use app::MobileApp;

#[no_mangle]
pub extern "C" fn android_main(app: AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default()
            .with_min_level(log::Level::Debug)
    );
    
    log::info!("Starting Ballistics Analyzer Mobile");
    
    // Initialize the app
    let mobile_app = MobileApp::new(&app);
    
    // Run the main loop
    mobile_app.run();
}

// JNI exports for Java integration
#[cfg(target_os = "android")]
pub mod android {
    use jni::JNIEnv;
    use jni::objects::{JClass, JObject};
    use jni::sys::{jint, jfloat};

    #[no_mangle]
    pub extern "system" fn Java_com_ballisticsanalyzer_mobile_NativeLib_init(
        _env: JNIEnv,
        _class: JClass,
        _context: JObject,
    ) {
        // Initialize the app
    }

    #[no_mangle]
    pub extern "system" fn Java_com_ballisticsanalyzer_mobile_NativeLib_onTouch(
        _env: JNIEnv,
        _class: JClass,
        x: jfloat,
        y: jfloat,
        action: jint,
    ) {
        // Handle touch events
    }
}