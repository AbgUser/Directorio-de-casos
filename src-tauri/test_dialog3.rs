use tauri::{AppHandle, WebviewUrl, WebviewWindowBuilder, webview::DownloadEvent, Manager};
use tauri_plugin_dialog::DialogExt;

fn test(app: &AppHandle) {
    let builder = WebviewWindowBuilder::new(app, "test", WebviewUrl::default())
        .on_download(move |window, event| {
            match event {
                DownloadEvent::Requested { url, destination } => {
                    let default_name = destination.file_name().unwrap_or_default().to_string_lossy().to_string();
                    let window_clone = window.clone();
                    let url_clone = url.clone();
                    tauri::async_runtime::spawn(async move {
                        window_clone.dialog().file()
                            .set_file_name(default_name)
                            .set_directory("/some/path")
                            .save_file(move |file_path| {
                                if let Some(path) = file_path {
                                    // Got path
                                    let cookies = window_clone.cookies_for_url(&url_clone);
                                }
                            });
                    });
                    false
                }
                _ => true
            }
        });
}
