use tauri::{AppHandle, WebviewUrl, WebviewWindowBuilder, webview::DownloadEvent};
use tauri_plugin_dialog::DialogExt;
use std::path::PathBuf;

fn test(app: &AppHandle) {
    let builder = WebviewWindowBuilder::new(app, "test", WebviewUrl::default())
        .on_download(move |window, event| {
            match event {
                DownloadEvent::Requested { destination, .. } => {
                    let default_name = destination.file_name().unwrap_or_default().to_string_lossy().to_string();
                    let file_path = window.dialog().file()
                        .set_file_name(default_name)
                        .set_directory("/some/path")
                        .blocking_save_file();
                        
                    if let Some(path) = file_path {
                        *destination = path.into_path().unwrap();
                        true
                    } else {
                        false
                    }
                }
                DownloadEvent::Finished { .. } => true,
                _ => true
            }
        });
}
