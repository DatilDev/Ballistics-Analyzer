use objc::{msg_send, sel, sel_impl};
use objc::runtime::{Class, Object};
use objc_foundation::{INSString, NSString};
use std::ffi::CString;

pub fn start_location_updates() {
    unsafe {
        let class = Class::get("LocationService").unwrap();
        let service: *mut Object = msg_send![class, sharedInstance];
        let _: () = msg_send![service, startLocationUpdates];
    }
}

pub fn stop_location_updates() {
    unsafe {
        let class = Class::get("LocationService").unwrap();
        let service: *mut Object = msg_send![class, sharedInstance];
        let _: () = msg_send![service, stopLocationUpdates];
    }
}

pub fn take_photo() {
    unsafe {
        let class = Class::get("CameraService").unwrap();
        let service: *mut Object = msg_send![class, sharedInstance];
        let _: () = msg_send![service, takePhoto];
    }
}

pub fn share_text(text: &str, title: &str) {
    unsafe {
        let text_ns = NSString::from_str(text);
        let title_ns = NSString::from_str(title);
        
        let class = Class::get("ShareService").unwrap();
        let service: *mut Object = msg_send![class, sharedInstance];
        let _: () = msg_send![service, shareText:text_ns withTitle:title_ns];
    }
}

pub fn copy_to_clipboard(text: &str) {
    unsafe {
        let text_ns = NSString::from_str(text);
        
        let pasteboard_class = Class::get("UIPasteboard").unwrap();
        let pasteboard: *mut Object = msg_send![pasteboard_class, generalPasteboard];
        let _: () = msg_send![pasteboard, setString:text_ns];
    }
}

pub fn get_documents_directory() -> String {
    unsafe {
        let file_manager_class = Class::get("NSFileManager").unwrap();
        let file_manager: *mut Object = msg_send![file_manager_class, defaultManager];
        
        let urls: *mut Object = msg_send![
            file_manager,
            URLsForDirectory:9 // NSDocumentDirectory
            inDomains:1 // NSUserDomainMask
        ];
        
        let url: *mut Object = msg_send![urls, firstObject];
        let path: *mut Object = msg_send![url, path];
        
        let c_str: *const i8 = msg_send![path, UTF8String];
        let rust_str = CString::from_raw(c_str as *mut i8)
            .to_string_lossy()
            .into_owned();
        
        rust_str
    }
}