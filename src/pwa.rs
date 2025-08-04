#[cfg(target_arch = "wasm32")]
pub mod web {
    use wasm_bindgen::prelude::*;
    use web_sys::{window, Navigator};

    pub async fn register_service_worker() {
        if let Some(window) = window() {
            let navigator: Navigator = window.navigator();
            if let Some(container) = navigator.service_worker() {
                match container.register("./sw.js").await {
                    Ok(_registration) => {
                        web_sys::console::log_1(&"Service Worker registered successfully".into());
                    }
                    Err(e) => {
                        web_sys::console::error_1(
                            &format!("Service Worker registration failed: {:?}", e).into(),
                        );
                    }
                }
            }
        }
    }

    pub async fn request_notification_permission() {
        // TODO: implement if needed
        let _ = window();
    }

    pub fn is_standalone() -> bool {
        if let Some(window) = window() {
            if let Ok(mq) = window.match_media("(display-mode: standalone)") {
                if let Some(mq) = mq {
                    return mq.matches();
                }
            }
        }
        false
    }
}