use tauri::{AppHandle, WebviewUrl, WebviewWindowBuilder, webview::DownloadEvent};

fn test_download(app: &AppHandle) {
    let builder = WebviewWindowBuilder::new(app, "test", WebviewUrl::External("https://example.com".parse().unwrap()))
        .on_download(|app_handle, event| {
            match event {
                DownloadEvent::Requested { url, destination } => {
                    println!("url: {}, dest: {:?}", url, destination);
                }
                DownloadEvent::Finished { url, path, success } => {
                }
                _ => {}
            }
            true
        });
}
