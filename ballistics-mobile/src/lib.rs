#![cfg_attr(target_os = "android", feature(panic_info_message))]

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

pub mod app;
pub mod ui;
pub mod platform;
pub mod storage;


// Android JNI exports
#[cfg(target_os = "android")]
pub mod android {
    use super::*;
    use jni::objects::{JClass, JObject, JString};
    use jni::sys::{jlong, jstring};
    use jni::JNIEnv;
    use std::sync::Mutex;

    static APP_INSTANCE: Mutex<Option<MobileApp>> = Mutex::new(None);

    #[no_mangle]
    pub extern "C" fn Java_com_ballistics_analyzer_MainActivity_initApp(
        env: JNIEnv,
        _class: JClass,
        assets_path: JString,
    ) -> jlong {
        android_logger::init_once(
            android_logger::Config::default()
                .with_max_level(log::LevelFilter::Debug)
                .with_tag("BallisticsAnalyzer"),
        );

        let assets_path: String = env
            .get_string(assets_path)
            .expect("Invalid string")
            .into();

        let app = MobileApp::new(&assets_path);
        let app_ptr = Box::into_raw(Box::new(app));
        
        app_ptr as jlong
    }

    #[no_mangle]
    pub extern "C" fn Java_com_ballistics_analyzer_MainActivity_runFrame(
        _env: JNIEnv,
        _class: JClass,
        app_ptr: jlong,
    ) {
        let app = unsafe { &mut *(app_ptr as *mut MobileApp) };
        app.update();
    }

    #[no_mangle]
    pub extern "C" fn Java_com_ballistics_analyzer_MainActivity_touchEvent(
        _env: JNIEnv,
        _class: JClass,
        app_ptr: jlong,
        x: f32,
        y: f32,
        event_type: i32,
    ) {
        let app = unsafe { &mut *(app_ptr as *mut MobileApp) };
        app.handle_touch(x, y, event_type);
    }

    #[no_mangle]
    pub extern "C" fn Java_com_ballistics_analyzer_MainActivity_onResume(
        _env: JNIEnv,
        _class: JClass,
        app_ptr: jlong,
    ) {
        let app = unsafe { &mut *(app_ptr as *mut MobileApp) };
        app.on_resume();
    }

    #[no_mangle]
    pub extern "C" fn Java_com_ballistics_analyzer_MainActivity_onPause(
        _env: JNIEnv,
        _class: JClass,
        app_ptr: jlong,
    ) {
        let app = unsafe { &mut *(app_ptr as *mut MobileApp) };
        app.on_pause();
    }

    #[no_mangle]
    pub extern "C" fn Java_com_ballistics_analyzer_MainActivity_destroyApp(
        _env: JNIEnv,
        _class: JClass,
        app_ptr: jlong,
    ) {
        let _app = unsafe { Box::from_raw(app_ptr as *mut MobileApp) };
        // App is dropped here
    }
}

// iOS exports
#[cfg(target_os = "ios")]
pub mod ios {
    use super::*;
    use std::sync::Mutex;

    static APP_INSTANCE: Mutex<Option<MobileApp>> = Mutex::new(None);

    #[no_mangle]
    pub extern "C" fn ballistics_init(storage_path: *const c_char) -> *mut MobileApp {
        let storage_path = unsafe {
            CStr::from_ptr(storage_path)
                .to_string_lossy()
                .into_owned()
        };

        let app = Box::new(MobileApp::new(&storage_path));
        Box::into_raw(app)
    }

    #[no_mangle]
    pub extern "C" fn ballistics_update(app: *mut MobileApp) {
        let app = unsafe { &mut *app };
        app.update();
    }

    #[no_mangle]
    pub extern "C" fn ballistics_render(app: *mut MobileApp) {
        let app = unsafe { &mut *app };
        app.render();
    }

    #[no_mangle]
    pub extern "C" fn ballistics_touch_event(
        app: *mut MobileApp,
        x: f32,
        y: f32,
        event_type: i32,
    ) {
        let app = unsafe { &mut *app };
        app.handle_touch(x, y, event_type);
    }

    #[no_mangle]
    pub extern "C" fn ballistics_on_resume(app: *mut MobileApp) {
        let app = unsafe { &mut *app };
        app.on_resume();
    }

    #[no_mangle]
    pub extern "C" fn ballistics_on_pause(app: *mut MobileApp) {
        let app = unsafe { &mut *app };
        app.on_pause();
    }

    #[no_mangle]
    pub extern "C" fn ballistics_destroy(app: *mut MobileApp) {
        unsafe {
            let _ = Box::from_raw(app);
        }
    }

    #[no_mangle]
    pub extern "C" fn ballistics_get_json_result(app: *mut MobileApp) -> *mut c_char {
        let app = unsafe { &mut *app };
        let result = app.get_calculation_result();
        CString::new(result).unwrap().into_raw()
    }

    #[no_mangle]
    pub extern "C" fn ballistics_free_string(s: *mut c_char) {
        unsafe {
            if !s.is_null() {
                let _ = CString::from_raw(s);
            }
        }
    }
}