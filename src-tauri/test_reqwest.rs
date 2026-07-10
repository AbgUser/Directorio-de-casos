use tauri::{AppHandle, WebviewUrl, WebviewWindowBuilder, webview::DownloadEvent, Manager};
use tauri_plugin_dialog::DialogExt;

fn test(app: &AppHandle) {
    let builder = WebviewWindowBuilder::new(app, "test", WebviewUrl::default())
        .on_download(move |window, event| {
            match event {
                DownloadEvent::Requested { url, destination } => {
                    let window_clone = window.clone();
                    let url_clone = url.clone();
                    tauri::async_runtime::spawn(async move {
                        window_clone.dialog().file().save_file(move |file_path| {
                            if let Some(path) = file_path {
                                tauri::async_runtime::spawn(async move {
                                    let mut cookie_str = String::new();
                                    if let Ok(cookies) = window_clone.cookies_for_url(&url_clone) {
                                        // Try accessing properties
                                        for c in cookies {
                                            cookie_str.push_str(&format!("{}={}; ", c.name, c.value));
                                        }
                                    }
                                });
                            }
                        });
                    });
                    false
                }
                _ => true
            }
        });
}
