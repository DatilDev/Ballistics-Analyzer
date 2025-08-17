use jni::objects::{JObject, JString, JValue};
use jni::{JNIEnv, JavaVM};
use std::sync::Mutex;

static JAVA_VM: Mutex<Option<JavaVM>> = Mutex::new(None);

pub fn init_java_vm(vm: JavaVM) {
    *JAVA_VM.lock().unwrap() = Some(vm);
}

pub fn start_location_updates() {
    if let Some(vm) = JAVA_VM.lock().unwrap().as_ref() {
        let env = vm.attach_current_thread().unwrap();
        
        let class = env.find_class("com/ballistics/analyzer/LocationService").unwrap();
        let method = env.get_static_method_id(
            class,
            "startLocationUpdates",
            "()V"
        ).unwrap();
        
        env.call_static_method_unchecked(
            class,
            method,
            &[],
        ).unwrap();
    }
}

pub fn stop_location_updates() {
    if let Some(vm) = JAVA_VM.lock().unwrap().as_ref() {
        let env = vm.attach_current_thread().unwrap();
        
        let class = env.find_class("com/ballistics/analyzer/LocationService").unwrap();
        let method = env.get_static_method_id(
            class,
            "stopLocationUpdates",
            "()V"
        ).unwrap();
        
        env.call_static_method_unchecked(
            class,
            method,
            &[],
        ).unwrap();
    }
}

pub fn take_photo() {
    if let Some(vm) = JAVA_VM.lock().unwrap().as_ref() {
        let env = vm.attach_current_thread().unwrap();
        
        let class = env.find_class("com/ballistics/analyzer/CameraService").unwrap();
        let method = env.get_static_method_id(
            class,
            "takePhoto",
            "()V"
        ).unwrap();
        
        env.call_static_method_unchecked(
            class,
            method,
            &[],
        ).unwrap();
    }
}

pub fn share_text(text: &str, title: &str) {
    if let Some(vm) = JAVA_VM.lock().unwrap().as_ref() {
        let env = vm.attach_current_thread().unwrap();
        
        let text = env.new_string(text).unwrap();
        let title = env.new_string(title).unwrap();
        
        let class = env.find_class("com/ballistics/analyzer/ShareService").unwrap();
        let method = env.get_static_method_id(
            class,
            "shareText",
            "(Ljava/lang/String;Ljava/lang/String;)V"
        ).unwrap();
        
        env.call_static_method_unchecked(
            class,
            method,
            &[
                JValue::Object(text.into()),
                JValue::Object(title.into()),
            ],
        ).unwrap();
    }
}

pub fn copy_to_clipboard(text: &str) {
    if let Some(vm) = JAVA_VM.lock().unwrap().as_ref() {
        let env = vm.attach_current_thread().unwrap();
        
        let text = env.new_string(text).unwrap();
        
        let class = env.find_class("com/ballistics/analyzer/ClipboardService").unwrap();
        let method = env.get_static_method_id(
            class,
            "copyToClipboard",
            "(Ljava/lang/String;)V"
        ).unwrap();
        
        env.call_static_method_unchecked(
            class,
            method,
            &[JValue::Object(text.into())],
        ).unwrap();
    }
}