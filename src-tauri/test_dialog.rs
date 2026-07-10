use tauri::{AppHandle, WebviewUrl, WebviewWindowBuilder, webview::DownloadEvent};
use tauri_plugin_dialog::DialogExt;
use std::path::PathBuf;

fn test(app: &AppHandle) {
    let app_clone = app.clone();
    let builder = WebviewWindowBuilder::new(app, "test", WebviewUrl::default())
        .on_download(move |window, event| {
            match event {
                DownloadEvent::Requested { destination, .. } => {
                    let default_name = destination.file_name().unwrap_or_default().to_string_lossy().to_string();
                    if let Some(path) = window.dialog().file().set_file_name(default_name).blocking_save_file() {
                        *destination = path.into_path().unwrap();
                    }
                }
                _ => {}
            }
            true
        });
}
