use tauri::AppHandle;
use tauri::Manager;
use std::fs;
use std::path::PathBuf;
use std::time::SystemTime;
use std::process::Command;

#[tauri::command]
pub fn guardar_modelo_boveda(app: AppHandle, ruta_origen: String) -> Result<String, String> {
    let app_data_dir = app.path().app_data_dir().map_err(|e| format!("Error al obtener app_data_dir: {}", e))?;
    let modelos_dir = app_data_dir.join("modelos");
    
    if !modelos_dir.exists() {
        fs::create_dir_all(&modelos_dir).map_err(|e| format!("Error creando dir modelos: {}", e))?;
    }
    
    let path_origen = PathBuf::from(&ruta_origen);
    let nombre_archivo = path_origen.file_name()
        .ok_or("Ruta de origen inválida")?
        .to_string_lossy()
        .to_string();
        
    // Para evitar colisiones, le añadimos un timestamp
    let timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
    let nuevo_nombre = format!("{}_{}", timestamp, nombre_archivo);
    
    let ruta_destino = modelos_dir.join(&nuevo_nombre);
    
    fs::copy(&path_origen, &ruta_destino)
        .map_err(|e| format!("Error copiando el archivo a la bóveda: {}", e))?;
        
    Ok(ruta_destino.to_string_lossy().to_string())
}

/// Clona el modelo hacia la carpeta del caso, lo abre nativamente
#[tauri::command]
pub fn clonar_y_abrir_modelo(
    ruta_modelo: String,
    carpeta_destino: String,
    nuevo_nombre: String
) -> Result<(), String> {
    let dest_dir = PathBuf::from(&carpeta_destino);
    if !dest_dir.exists() {
        fs::create_dir_all(&dest_dir).map_err(|e| format!("Error creando carpeta destino: {}", e))?;
    }
    
    let ruta_destino = dest_dir.join(&nuevo_nombre);
    
    fs::copy(&ruta_modelo, &ruta_destino)
        .map_err(|e| format!("Error clonando el modelo: {}", e))?;
        
    // Abrir nativamente
    let ruta_destino_str = ruta_destino.to_string_lossy().to_string();
    
    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg(&ruta_destino_str)
            .spawn()
            .map_err(|e| format!("Error al abrir archivo clonado: {}", e))?;
    }

    #[cfg(target_os = "windows")]
    {
        Command::new("explorer")
            .arg(&ruta_destino_str)
            .spawn()
            .map_err(|e| format!("Error al abrir archivo clonado: {}", e))?;
    }

    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open")
            .arg(&ruta_destino_str)
            .spawn()
            .map_err(|e| format!("Error al abrir archivo clonado: {}", e))?;
    }
    
    Ok(())
}
