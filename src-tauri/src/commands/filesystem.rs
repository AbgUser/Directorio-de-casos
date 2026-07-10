// =============================================================================
// commands/filesystem.rs
// Comandos Tauri para operaciones del sistema de archivos:
// abrir en Finder/Explorer, abrir archivos, imprimir, crear directorios
// y generar slugs seguros para nombres de carpetas.
// =============================================================================

use std::process::Command;
use serde::Serialize;
use std::time::SystemTime;

#[derive(Serialize)]
pub struct DocumentoFisico {
    pub id: String,
    pub nombre: String,
    pub ruta_absoluta: String,
    pub tamano_bytes: u64,
    pub created_at: u64, // ms since epoch
    pub tipo_archivo: String,
}

/// Abre la carpeta contenedora de un archivo en Finder (macOS) o Explorer (Windows).
///
/// # Parámetros
/// - `path`: Ruta absoluta al archivo cuya carpeta contenedora se desea abrir.
///
/// # Comportamiento por plataforma
/// - **macOS**: Ejecuta `open -R <path>` (revela el archivo en Finder).
/// - **Windows**: Ejecuta `explorer /select,<path>`.
/// - **Linux**: Ejecuta `xdg-open` sobre el directorio padre.
#[tauri::command]
pub fn open_in_finder(path: String) -> Result<(), String> {
    if !std::path::Path::new(&path).exists() {
        return Err("El archivo físico fue eliminado del disco o movido de su ubicación original.".into());
    }
    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg("-R")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("Error al abrir en Finder: {}", e))?;
    }

    #[cfg(target_os = "windows")]
    {
        Command::new("explorer")
            .arg(format!("/select,{}", &path))
            .spawn()
            .map_err(|e| format!("Error al abrir en Explorer: {}", e))?;
    }

    #[cfg(target_os = "linux")]
    {
        let parent = std::path::Path::new(&path)
            .parent()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| path.clone());
        Command::new("xdg-open")
            .arg(&parent)
            .spawn()
            .map_err(|e| format!("Error al abrir directorio: {}", e))?;
    }

    Ok(())
}

/// Abre un archivo con la aplicación predeterminada del sistema operativo.
///
/// # Parámetros
/// - `path`: Ruta absoluta al archivo a abrir.
///
/// # Comportamiento por plataforma
/// - **macOS**: Ejecuta `open <path>`.
/// - **Windows**: Ejecuta `explorer <path>`.
/// - **Linux**: Ejecuta `xdg-open <path>`.
#[tauri::command]
pub fn abrir_documento(path: String) -> Result<(), String> {
    if !std::path::Path::new(&path).exists() {
        return Err("El archivo físico fue eliminado del disco o movido de su ubicación original.".into());
    }
    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("Error al abrir archivo: {}", e))?;
    }

    #[cfg(target_os = "windows")]
    {
        Command::new("explorer")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("Error al abrir archivo: {}", e))?;
    }

    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("Error al abrir archivo: {}", e))?;
    }

    Ok(())
}

/// Imprime un archivo utilizando el sistema de impresión del SO.
///
/// # Parámetros
/// - `path`: Ruta absoluta al archivo a imprimir.
///
/// # Comportamiento por plataforma
/// - **macOS**: Ejecuta `lp <path>` (envía a la impresora predeterminada).
/// - **Windows**: Ejecuta `print <path>`.
///
/// # Nota
/// En macOS, el archivo se envía directamente a la cola de impresión.
/// Para elegir impresora, usar el diálogo de impresión del frontend.
#[tauri::command]
pub fn imprimir_documento(path: String) -> Result<(), String> {
    if !std::path::Path::new(&path).exists() {
        return Err("El archivo físico fue eliminado del disco o movido de su ubicación original.".into());
    }
    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg("-a")
            .arg("Preview")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("Error al abrir para imprimir: {}", e))?;
    }

    #[cfg(target_os = "windows")]
    {
        Command::new("cmd")
            .args(["/C", "print", &path])
            .spawn()
            .map_err(|e| format!("Error al imprimir archivo: {}", e))?;
    }

    #[cfg(target_os = "linux")]
    {
        Command::new("lpr")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("Error al imprimir archivo: {}", e))?;
    }

    Ok(())
}

/// Crea un directorio de forma recursiva (incluyendo directorios padres).
///
/// # Parámetros
/// - `path`: Ruta absoluta del directorio a crear.
///
/// # Retorna
/// - `Ok(())`: Directorio creado exitosamente (o ya existía).
/// - `Err(String)`: Error si no se pudo crear el directorio.
#[tauri::command]
pub fn create_directory(path: String) -> Result<(), String> {
    std::fs::create_dir_all(&path)
        .map_err(|e| format!("Error al crear directorio '{}': {}", path, e))
}

/// Copia un archivo usando std::fs::copy directamente desde Rust.
/// Esto evita los bloqueos de scope de permisos del plugin-fs en el frontend
/// cuando el usuario selecciona un archivo con el diálogo (que puede estar en cualquier lado).
#[tauri::command]
pub fn copy_file_native(source: String, dest: String) -> Result<(), String> {
    std::fs::copy(&source, &dest)
        .map(|_| ())
        .map_err(|e| format!("Error copiando '{}' a '{}': {}", source, dest, e))
}

/// Lee los bytes de un archivo de forma nativa
#[tauri::command]
pub fn read_file_bytes(path: String) -> Result<Vec<u8>, String> {
    std::fs::read(&path).map_err(|e| format!("Error leyendo archivo '{}': {}", path, e))
}

/// Escribe bytes en un archivo de forma nativa
#[tauri::command]
pub fn write_file_bytes(path: String, bytes: Vec<u8>) -> Result<(), String> {
    std::fs::write(&path, bytes).map_err(|e| format!("Error escribiendo archivo '{}': {}", path, e))
}

/// Convierte un nombre en un slug seguro para el sistema de archivos.
///
/// Transforma el texto a minúsculas, reemplaza caracteres acentuados por
/// sus equivalentes sin acento, elimina caracteres no alfanuméricos
/// (excepto guiones), y colapsa guiones consecutivos.
///
/// # Ejemplo
/// ```
/// // "García López & Asociados" → "garcia-lopez-asociados"
/// // "Caso Nº 2024-001 (Familia)" → "caso-no-2024-001-familia"
/// ```
///
/// # Parámetros
/// - `name`: Texto a convertir en slug.
///
/// # Retorna
/// - Slug seguro para uso como nombre de carpeta o identificador.
#[tauri::command]
pub fn generate_slug(name: String) -> String {
    let lowered = name.to_lowercase();

    // Reemplazar caracteres acentuados comunes del español
    let replaced: String = lowered
        .chars()
        .map(|c| match c {
            'á' | 'à' | 'ä' | 'â' => 'a',
            'é' | 'è' | 'ë' | 'ê' => 'e',
            'í' | 'ì' | 'ï' | 'î' => 'i',
            'ó' | 'ò' | 'ö' | 'ô' => 'o',
            'ú' | 'ù' | 'ü' | 'û' => 'u',
            'ñ' => 'n',
            'ç' => 'c',
            ' ' | '_' | '.' => '-',
            'º' | 'ª' => '-',
            c if c.is_ascii_alphanumeric() || c == '-' => c,
            _ => '-',
        })
        .collect();

    // Colapsar guiones consecutivos y eliminar guiones al inicio/final
    let mut result = String::with_capacity(replaced.len());
    let mut last_was_hyphen = true; // true para eliminar guión inicial

    for c in replaced.chars() {
        if c == '-' {
            if !last_was_hyphen {
                result.push('-');
                last_was_hyphen = true;
            }
        } else {
            result.push(c);
            last_was_hyphen = false;
        }
    }

    // Eliminar guión final si existe
    result
}

/// Lee el directorio físico del expediente y devuelve la lista de archivos reales
#[tauri::command]
pub fn leer_directorio_expediente(ruta_absoluta: String) -> Result<Vec<DocumentoFisico>, String> {
    let mut documentos = Vec::new();
    let dir = std::path::Path::new(&ruta_absoluta);
    
    // Crear el directorio si no existe para evitar errores
    if !dir.exists() {
        if let Err(_) = std::fs::create_dir_all(dir) {
            return Ok(documentos);
        }
    }
    
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return Ok(documentos), // Interceptar el error y devolver lista vacía
    };
    
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_file() {
            // Ignorar archivos ocultos como .DS_Store
            let file_name = entry.file_name();
            let nombre = file_name.to_string_lossy().to_string();
            if nombre.starts_with('.') {
                continue;
            }
            
            let metadata = entry.metadata().ok();
            let tamano_bytes = metadata.as_ref().map(|m| m.len()).unwrap_or(0);
            
            let created_at = metadata
                .and_then(|m| m.modified().ok())
                .and_then(|t| t.duration_since(SystemTime::UNIX_EPOCH).ok())
                .map(|d| d.as_millis() as u64)
                .unwrap_or(0);
                
            let extension = path.extension().unwrap_or_default().to_string_lossy().to_lowercase();
            
            let tipo_archivo = match extension.as_str() {
                "pdf" => "application/pdf",
                "doc" | "rtf" => "application/msword",
                "docx" => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
                "xls" => "application/vnd.ms-excel",
                "xlsx" => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
                "jpg" | "jpeg" => "image/jpeg",
                "png" => "image/png",
                "txt" => "text/plain",
                _ => "application/octet-stream",
            }.to_string();
            
            documentos.push(DocumentoFisico {
                id: nombre.clone(), // Usamos el nombre como ID único en el scope del directorio
                nombre,
                ruta_absoluta: path.to_string_lossy().to_string(),
                tamano_bytes,
                created_at,
                tipo_archivo,
            });
        }
    }
    
    // Sort documents alphabetically
    documentos.sort_by(|a, b| a.nombre.cmp(&b.nombre));
    
    Ok(documentos)
}

/// Elimina un archivo del disco de forma nativa
#[tauri::command]
pub fn remove_file_native(path: String) -> Result<(), String> {
    std::fs::remove_file(&path).map_err(|e| format!("Error eliminando archivo '{}': {}", path, e))
}

/// Copia un lote de archivos hacia el directorio destino y retorna sus nombres.
#[tauri::command]
pub fn copiar_archivos_actuacion(rutas: Vec<String>, carpeta_destino: String) -> Result<Vec<String>, String> {
    let mut nombres_copiados = Vec::new();
    let dir_destino = std::path::Path::new(&carpeta_destino);

    if !dir_destino.exists() {
        if let Err(e) = std::fs::create_dir_all(dir_destino) {
            return Err(format!("Error creando directorio de destino: {}", e));
        }
    }

    for ruta in rutas {
        let path = std::path::Path::new(&ruta);
        if !path.exists() || !path.is_file() { continue; }

        if let Some(file_name) = path.file_name() {
            let dest_path = dir_destino.join(file_name);
            
            if let Err(e) = std::fs::copy(path, &dest_path) {
                eprintln!("Error copiando archivo {:?}: {}", path, e);
            } else {
                if let Some(name_str) = file_name.to_str() {
                    nombres_copiados.push(name_str.to_string());
                }
            }
        }
    }

    Ok(nombres_copiados)
}
