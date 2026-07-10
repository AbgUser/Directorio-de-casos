use tauri::{AppHandle, WebviewUrl, WebviewWindowBuilder, webview::DownloadEvent};

fn test_download(app: &AppHandle) {
    let builder = WebviewWindowBuilder::new(app, "test", WebviewUrl::External("https://example.com".parse().unwrap()))
        .on_download(|app_handle, event| {
            match event {
                DownloadEvent::Requested { url, destination } => {
                    destination.push("my_file.pdf");
                }
                _ => {}
            }
            true
        });
}
