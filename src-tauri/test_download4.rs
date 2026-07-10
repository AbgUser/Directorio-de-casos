use tauri::{AppHandle, Manager, Emitter, WebviewUrl, WebviewWindowBuilder, webview::DownloadEvent};
use std::path::PathBuf;
use serde::Serialize;

#[derive(Clone, Serialize)]
struct DownloadPayload {
    caso_id: i64,
    nombre_archivo: String,
    ruta_archivo: String,
}

fn test_download(app: &AppHandle, caso_id: i64, carpeta_documentos: String) {
    let app_handle_clone = app.clone();
    let builder = WebviewWindowBuilder::new(app, "test", WebviewUrl::External("https://example.com".parse().unwrap()))
        .on_download(move |window, event| {
            match event {
                DownloadEvent::Requested { url, destination } => {
                    if let Some(filename) = destination.file_name().and_then(|n| n.to_str()) {
                        if let Ok(docs) = app_handle_clone.path().document_dir() {
                            let mut custom_path = docs;
                            custom_path.push("Directorio_Casos");
                            custom_path.push(&carpeta_documentos);
                            let _ = std::fs::create_dir_all(&custom_path);
                            custom_path.push(filename);
                            *destination = custom_path;
                        }
                    }
                }
                DownloadEvent::Finished { url, path, success } => {
                    if success {
                        if let Some(p) = path {
                            if let Some(filename) = p.file_name().and_then(|n| n.to_str()) {
                                let payload = DownloadPayload {
                                    caso_id,
                                    nombre_archivo: filename.to_string(),
                                    ruta_archivo: filename.to_string(),
                                };
                                let _ = app_handle_clone.emit("documento-descargado", payload);
                            }
                        }
                    }
                }
                _ => {}
            }
            true
        });
}
