use tauri::{AppHandle, WebviewUrl, WebviewWindowBuilder};

fn test_download(app: &AppHandle) {
    let builder = WebviewWindowBuilder::new(app, "test", WebviewUrl::External("https://example.com".parse().unwrap()))
        .on_download(|app_handle, event| {
            true
        });
}
