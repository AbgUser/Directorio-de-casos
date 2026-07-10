use rusqlite::{params_from_iter, Connection, OpenFlags, types::Value as SqlValue};
use serde_json::{json, Map, Value as JsonValue};
use std::sync::Mutex;
use tauri::{AppHandle, Manager, State};
use std::io::{Read, Write};
use zip::write::FileOptions;
use walkdir::WalkDir;

use crate::db::migrations;

pub struct DbState {
    pub conn: Mutex<Option<Connection>>,
}

/// Cierra la conexión de la base de datos
#[tauri::command]
pub fn db_close_connection(state: State<'_, DbState>) -> Result<bool, String> {
    *state.conn.lock().unwrap() = None;
    Ok(true)
}

/// Elimina la base de datos completamente (Hard Reset)
#[tauri::command]
pub fn db_reset(app: AppHandle, state: State<'_, DbState>) -> Result<bool, String> {
    // Cerrar conexión actual si existe
    *state.conn.lock().unwrap() = None;

    let app_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let db_path = app_dir.join("directorio_casos.db");
    
    if db_path.exists() {
        std::fs::remove_file(&db_path).map_err(|e| e.to_string())?;
    }
    
    // Eliminar también los archivos wal y shm si existen
    let wal_path = app_dir.join("directorio_casos.db-wal");
    if wal_path.exists() { let _ = std::fs::remove_file(&wal_path); }
    
    let shm_path = app_dir.join("directorio_casos.db-shm");
    if shm_path.exists() { let _ = std::fs::remove_file(&shm_path); }

    Ok(true)
}

/// Cambia la contraseña (clave de encriptación) de la base de datos abierta
#[tauri::command]
pub fn db_change_password(state: State<'_, DbState>, new_password: &str) -> Result<bool, String> {
    let mut conn_guard = state.conn.lock().unwrap();
    let conn = conn_guard.as_mut().ok_or("Base de datos no inicializada. Debe iniciar sesión primero.")?;

    let rekey_query = format!("PRAGMA rekey = '{}';", new_password.replace("'", "''"));
    conn.execute_batch(&rekey_query).map_err(|e| e.to_string())?;

    Ok(true)
}



/// Exporta una copia de seguridad de la base de datos a un destino específico
#[tauri::command]
pub fn db_export_backup(state: State<'_, DbState>, dest_path: &str) -> Result<bool, String> {
    let mut conn_guard = state.conn.lock().unwrap();
    let conn = conn_guard.as_mut().ok_or("Base de datos no inicializada")?;

    // Asegurarse de que el WAL se escriba completamente en el archivo principal (.db)
    // Usamos TRUNCATE para forzar el checkpoint y truncar el archivo WAL a 0 bytes.
    conn.execute_batch("PRAGMA wal_checkpoint(TRUNCATE);")
        .map_err(|e| format!("Error en WAL checkpoint: {}", e))?;

    // Obtenemos la ruta actual del archivo principal
    let db_path_str = conn.path().ok_or("No se pudo obtener la ruta de la base de datos")?;
    let db_path = std::path::PathBuf::from(db_path_str);
    
    // Copiar el archivo principal .db al destino seleccionado por el usuario
    std::fs::copy(&db_path, dest_path).map_err(|e| format!("Error copiando el archivo DB: {}", e))?;

    Ok(true)
}

/// Restaura una copia de seguridad sobreescribiendo la base de datos actual
#[tauri::command]
pub fn db_restore_backup(state: State<'_, DbState>, source_path: &str, app_handle: tauri::AppHandle) -> Result<bool, String> {
    let mut conn_guard = state.conn.lock().unwrap();
    
    // Si la DB está abierta, obtenemos su ruta y la cerramos soltando el guard
    let db_path = if let Some(conn) = conn_guard.as_ref() {
        let path_str = conn.path().unwrap_or("");
        std::path::PathBuf::from(path_str)
    } else {
        // Si no estaba inicializada, resolvemos la ruta por defecto
        let app_dir = app_handle.path().document_dir().unwrap().join("Directorio_Casos");
        app_dir.join("directorio_casos.db")
    };

    // Forzar el cierre de la conexión actual
    *conn_guard = None;

    // Sobrescribir el archivo .db principal con el backup
    std::fs::copy(source_path, &db_path).map_err(|e| format!("Error restaurando el archivo DB: {}", e))?;

    // Borrar el .db-wal y .db-shm si existen para forzar que el nuevo DB se lea limpio
    let mut wal_path = db_path.clone();
    wal_path.set_extension("db-wal");
    if wal_path.exists() { let _ = std::fs::remove_file(wal_path); }

    let mut shm_path = db_path.clone();
    shm_path.set_extension("db-shm");
    if shm_path.exists() { let _ = std::fs::remove_file(shm_path); }

    Ok(true)
}

/// Restablecimiento de fábrica: valida contraseña, elimina físicamente y recrea en blanco.
#[tauri::command]
pub fn eliminar_base_datos(app: AppHandle, state: State<'_, DbState>, contrasena_ingresada: String) -> Result<bool, String> {
    let app_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let db_path = app_dir.join("directorio_casos.db");

    // 1. Validación de la contraseña (intentando abrir una conexión nueva y leyendo)
    let test_conn = Connection::open(&db_path).map_err(|e| e.to_string())?;
    let pragma_query = format!("PRAGMA key = '{}';", contrasena_ingresada.replace("'", "''"));
    test_conn.execute_batch(&pragma_query).map_err(|e| e.to_string())?;
    
    if test_conn.query_row("SELECT count(*) FROM sqlite_master", [], |_| Ok(())).is_err() {
        return Err("Contraseña incorrecta".into());
    }
    
    // Cerrar explícitamente la conexión de prueba
    drop(test_conn);

    // 2. Cierre de conexiones principales
    *state.conn.lock().unwrap() = None;

    // 3. Eliminación Física
    let _ = std::fs::remove_file(&db_path);
    let _ = std::fs::remove_file(app_dir.join("directorio_casos.db-wal"));
    let _ = std::fs::remove_file(app_dir.join("directorio_casos.db-shm"));

    // 4. Recreación
    db_init(app, state, &contrasena_ingresada)?;

    Ok(true)
}

/// Genera un respaldo completo (DB + Directorio_Casos) en un archivo ZIP
#[tauri::command]
pub fn generar_backup_completo(state: State<'_, DbState>, app_handle: AppHandle, dest_path: String) -> Result<bool, String> {
    let mut conn_guard = state.conn.lock().unwrap();
    let conn = conn_guard.as_mut().ok_or("Base de datos no inicializada")?;

    // Forzar checkpoint WAL para asegurar integridad
    conn.execute_batch("PRAGMA wal_checkpoint(TRUNCATE);")
        .map_err(|e| format!("Error en WAL checkpoint: {}", e))?;

    let db_path_str = conn.path().ok_or("No se pudo obtener la ruta de la base de datos")?;
    let db_path = std::path::PathBuf::from(db_path_str);

    let zip_file = std::fs::File::create(&dest_path).map_err(|e| format!("No se pudo crear el archivo ZIP: {}", e))?;
    let mut zip = zip::ZipWriter::new(zip_file);
    
    // Opción por defecto (compresión standard)
    let options = FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    // 1. Añadir la BD al zip
    zip.start_file("base_de_datos/directorio_casos.db", options).map_err(|e| format!("Error iniciando archivo en zip: {}", e))?;
    let mut db_file = std::fs::File::open(&db_path).map_err(|e| format!("Error leyendo BD: {}", e))?;
    let mut buffer = Vec::new();
    db_file.read_to_end(&mut buffer).map_err(|e| format!("Error leyendo BD a buffer: {}", e))?;
    zip.write_all(&buffer).map_err(|e| format!("Error escribiendo BD en zip: {}", e))?;

    // 2. Añadir Expedientes Físicos (recursivamente)
    if let Ok(docs_dir) = app_handle.path().document_dir() {
        let expedientes_dir = docs_dir.join("Directorio_Casos");
        if expedientes_dir.exists() {
            // Recorrer la carpeta
            for entry in WalkDir::new(&expedientes_dir) {
                let entry = match entry {
                    Ok(e) => e,
                    Err(e) => {
                        eprintln!("Omitiendo archivo por error: {}", e);
                        continue;
                    }
                };
                
                let path = entry.path();
                let name = path.strip_prefix(&expedientes_dir).unwrap_or(path);
                
                if path.is_file() {
                    let zip_name = format!("expedientes/{}", name.to_string_lossy().replace("\\", "/"));
                    // Omitir si no se puede leer
                    if let Ok(mut f) = std::fs::File::open(path) {
                        if zip.start_file(zip_name, options).is_ok() {
                            let mut f_buffer = Vec::new();
                            if f.read_to_end(&mut f_buffer).is_ok() {
                                let _ = zip.write_all(&f_buffer);
                            }
                        }
                    }
                } else if path.is_dir() && name.to_string_lossy() != "" {
                    let zip_name = format!("expedientes/{}/", name.to_string_lossy().replace("\\", "/"));
                    let _ = zip.add_directory(zip_name, options);
                }
            }
        }
    }

    // Cerrar el stream zip adecuadamente
    zip.finish().map_err(|e| format!("Error cerrando archivo ZIP: {}", e))?;

    Ok(true)
}

/// Restaura un respaldo completo desde un archivo ZIP
#[tauri::command]
pub fn restaurar_backup_completo(app: AppHandle, state: State<'_, DbState>, source_path: String) -> Result<bool, String> {
    let app_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let db_path = app_dir.join("directorio_casos.db");

    let docs_dir = app.path().document_dir().map_err(|e| e.to_string())?;
    let expedientes_dir = docs_dir.join("Directorio_Casos");

    // 1. Cierre de Conexiones
    *state.conn.lock().unwrap() = None;

    let _ = std::fs::remove_file(app_dir.join("directorio_casos.db-wal"));
    let _ = std::fs::remove_file(app_dir.join("directorio_casos.db-shm"));

    // 2. Descompresión en Directorio Temporal
    let temp_dir = std::env::temp_dir().join(format!("restore_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()));
    std::fs::create_dir_all(&temp_dir).map_err(|e| format!("Error creando dir temp: {}", e))?;

    let file = std::fs::File::open(&source_path).map_err(|e| format!("Error abriendo ZIP: {}", e))?;
    let mut archive = zip::ZipArchive::new(file).map_err(|e| format!("Formato ZIP inválido: {}", e))?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(|e| e.to_string())?;
        let outpath = match file.enclosed_name() {
            Some(path) => temp_dir.join(path),
            None => continue,
        };

        if file.name().ends_with('/') {
            std::fs::create_dir_all(&outpath).map_err(|e| e.to_string())?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    std::fs::create_dir_all(&p).map_err(|e| e.to_string())?;
                }
            }
            let mut outfile = std::fs::File::create(&outpath).map_err(|e| e.to_string())?;
            std::io::copy(&mut file, &mut outfile).map_err(|e| e.to_string())?;
        }
    }

    // 3. Reemplazo de Base de Datos
    let extracted_db_path = temp_dir.join("base_de_datos").join("directorio_casos.db");
    if extracted_db_path.exists() {
        std::fs::copy(&extracted_db_path, &db_path).map_err(|e| format!("Error copiando BD: {}", e))?;
    } else {
        let _ = std::fs::remove_dir_all(&temp_dir);
        return Err("El archivo ZIP no contiene una base de datos válida (base_de_datos/directorio_casos.db)".to_string());
    }

    // 4. Reemplazo de Expedientes
    let extracted_expedientes = temp_dir.join("expedientes");
    if extracted_expedientes.exists() {
        std::fs::create_dir_all(&expedientes_dir).map_err(|e| e.to_string())?;
        for entry in walkdir::WalkDir::new(&extracted_expedientes) {
            let entry = entry.map_err(|e| e.to_string())?;
            let path = entry.path();
            if path.is_file() {
                let rel_path = path.strip_prefix(&extracted_expedientes).unwrap();
                let target_path = expedientes_dir.join(rel_path);
                if let Some(p) = target_path.parent() {
                    std::fs::create_dir_all(p).map_err(|e| e.to_string())?;
                }
                std::fs::copy(path, &target_path).map_err(|e| e.to_string())?;
            }
        }
    }

    // 5. Limpieza
    let _ = std::fs::remove_dir_all(&temp_dir);

    // 6. El frontend re-inicializará (reconexión) mediante recarga.
    Ok(true)
}


#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteResult {
    pub rows_affected: usize,
    pub last_insert_id: i64,
}

/// Convierte un JsonValue a un SqlValue
fn json_to_sql(val: &JsonValue) -> SqlValue {
    match val {
        JsonValue::Null => SqlValue::Null,
        JsonValue::Bool(b) => SqlValue::Integer(if *b { 1 } else { 0 }),
        JsonValue::Number(n) => {
            if let Some(i) = n.as_i64() {
                SqlValue::Integer(i)
            } else if let Some(f) = n.as_f64() {
                SqlValue::Real(f)
            } else {
                SqlValue::Null
            }
        }
        JsonValue::String(s) => SqlValue::Text(s.clone()),
        _ => SqlValue::Text(val.to_string()),
    }
}

/// Verifica si el archivo de la base de datos existe
#[tauri::command]
pub fn db_exists(app: AppHandle) -> Result<bool, String> {
    let app_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let db_path = app_dir.join("directorio_casos.db");
    Ok(db_path.exists())
}

/// Inicializa y desencripta la base de datos
#[tauri::command]
pub fn db_init(app: AppHandle, state: State<'_, DbState>, password: &str) -> Result<bool, String> {
    let app_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&app_dir).map_err(|e| e.to_string())?;
    
    let db_path = app_dir.join("directorio_casos.db");
    
    // Abrir o crear la base de datos
    let conn = Connection::open_with_flags(
        &db_path,
        OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE | OpenFlags::SQLITE_OPEN_NO_MUTEX,
    ).map_err(|e| e.to_string())?;

    // Aplicar la clave de encriptación (SQLCipher)
    let pragma_query = format!("PRAGMA key = '{}';", password.replace("'", "''"));
    conn.execute_batch(&pragma_query).map_err(|e| e.to_string())?;

    // Probar si la clave es correcta intentando leer
    if let Err(e) = conn.query_row("SELECT count(*) FROM sqlite_master", [], |_| Ok(())) {
        return Err(format!("Contraseña incorrecta o base de datos corrupta: {}", e));
    }

    // Configurar settings recomendados para SQLCipher/SQLite
    conn.execute_batch("
        PRAGMA foreign_keys = ON;
        PRAGMA journal_mode = WAL;
        PRAGMA synchronous = NORMAL;
    ").map_err(|e| e.to_string())?;

    // Ejecutar migraciones y seeds
    conn.execute_batch(migrations::get_schema()).map_err(|e| e.to_string())?;
    
    // Verificar si ya hay feriados para evitar errores
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM feriados", [], |row| row.get(0)).unwrap_or(0);
    if count == 0 {
        conn.execute_batch(migrations::get_seed_feriados()).map_err(|e| e.to_string())?;
    }
    
    // Verificar si ya hay config
    let config_count: i64 = conn.query_row("SELECT COUNT(*) FROM despacho_config", [], |row| row.get(0)).unwrap_or(0);
    if config_count == 0 {
        conn.execute_batch(migrations::get_seed_config()).map_err(|e| e.to_string())?;
    }



    // Upgrades de columnas — cada ALTER se ejecuta individualmente porque
    // falla si la columna ya existe, y no debe bloquear las demás.
    let _ = conn.execute_batch("ALTER TABLE honorarios_cuentas ADD COLUMN tarifa_hora REAL DEFAULT 0;");
    let _ = conn.execute_batch("ALTER TABLE honorarios_cuentas ADD COLUMN es_estatal INTEGER NOT NULL DEFAULT 0;");
    let _ = conn.execute_batch("ALTER TABLE honorarios_cuentas ADD COLUMN numero_contrato TEXT;");
    let _ = conn.execute_batch("ALTER TABLE honorarios_cuentas ADD COLUMN entidad_contratante TEXT;");
    let _ = conn.execute_batch("ALTER TABLE despacho_config ADD COLUMN plantilla_factura_path TEXT;");
    let _ = conn.execute_batch("ALTER TABLE actuaciones ADD COLUMN termino_vencimiento TEXT;");
    let _ = conn.execute_batch("ALTER TABLE actuaciones ADD COLUMN termino_completado INTEGER DEFAULT 0;");

    // Guardar la conexión en el estado
    *state.conn.lock().unwrap() = Some(conn);

    Ok(true)
}

/// Ejecuta una consulta de modificación (INSERT, UPDATE, DELETE)
#[tauri::command]
pub fn db_execute(state: State<'_, DbState>, query: &str, params: Vec<JsonValue>) -> Result<ExecuteResult, String> {
    let mut conn_guard = state.conn.lock().unwrap();
    let conn = conn_guard.as_mut().ok_or("Base de datos no inicializada")?;

    let sql_params: Vec<SqlValue> = params.iter().map(json_to_sql).collect();
    
    let mut stmt = conn.prepare(query).map_err(|e| e.to_string())?;
    let rows_affected = stmt.execute(params_from_iter(sql_params.iter())).map_err(|e| e.to_string())?;
    let last_insert_id = conn.last_insert_rowid();

    Ok(ExecuteResult {
        rows_affected,
        last_insert_id,
    })
}

/// Ejecuta una consulta de selección (SELECT)
#[tauri::command]
pub fn db_select(state: State<'_, DbState>, query: &str, params: Vec<JsonValue>) -> Result<Vec<JsonValue>, String> {
    let mut conn_guard = state.conn.lock().unwrap();
    let conn = conn_guard.as_mut().ok_or("Base de datos no inicializada")?;

    let sql_params: Vec<SqlValue> = params.iter().map(json_to_sql).collect();
    
    let mut stmt = conn.prepare(query).map_err(|e| e.to_string())?;
    
    let column_names: Vec<String> = stmt.column_names().into_iter().map(|s| s.to_string()).collect();
    
    let rows = stmt.query_map(params_from_iter(sql_params.iter()), |row| {
        let mut map = Map::new();
        for (i, col_name) in column_names.iter().enumerate() {
            let val = match row.get_ref(i)? {
                rusqlite::types::ValueRef::Null => JsonValue::Null,
                rusqlite::types::ValueRef::Integer(i) => json!(i),
                rusqlite::types::ValueRef::Real(f) => json!(f),
                rusqlite::types::ValueRef::Text(t) => {
                    let s = std::str::from_utf8(t).unwrap_or("");
                    json!(s)
                },
                rusqlite::types::ValueRef::Blob(b) => {
                    // Blob as base64 string or just string if it's text. We'll assume string for this CRM
                    let s = std::str::from_utf8(b).unwrap_or("");
                    json!(s)
                }
            };
            map.insert(col_name.clone(), val);
        }
        Ok(JsonValue::Object(map))
    }).map_err(|e| e.to_string())?;

    let mut result = Vec::new();
    for row in rows {
        result.push(row.map_err(|e| e.to_string())?);
    }

    Ok(result)
}

/// Marca un término judicial como completado
#[tauri::command]
pub fn marcar_termino_completado(state: State<'_, DbState>, id: i64) -> Result<bool, String> {
    let mut conn_guard = state.conn.lock().unwrap();
    let conn = conn_guard.as_mut().ok_or("Base de datos no inicializada")?;

    let query = "UPDATE actuaciones SET termino_completado = 1 WHERE id = ?";
    let mut stmt = conn.prepare(query).map_err(|e| e.to_string())?;
    stmt.execute([id]).map_err(|e| e.to_string())?;

    Ok(true)
}

/// Obtiene los términos judiciales activos de un caso específico
#[tauri::command]
pub fn obtener_agenda_caso(state: State<'_, DbState>, id_caso: i64) -> Result<Vec<JsonValue>, String> {
    let query = "
        SELECT 
            a.id, 
            a.caso_id,
            a.id as actuacion_id,
            'vencimiento' as tipo,
            'Vencimiento: ' || substr(a.descripcion, 1, 50) || '...' as titulo,
            a.descripcion,
            a.termino_vencimiento as fecha_inicio,
            'pendiente' as estado,
            c.radicado,
            cl.nombre_completo as cliente_display_name
        FROM actuaciones a
        JOIN casos c ON a.caso_id = c.id
        LEFT JOIN clientes cl ON c.cliente_id = cl.id
        WHERE a.caso_id = ? AND a.termino_vencimiento IS NOT NULL AND a.termino_completado = 0
        ORDER BY a.termino_vencimiento ASC
    ";
    db_select(state, query, vec![json!(id_caso)])
}

/// Obtiene todos los términos judiciales activos a nivel global
#[tauri::command]
pub fn obtener_calendario_global(state: State<'_, DbState>) -> Result<Vec<JsonValue>, String> {
    let query = "
        SELECT 
            a.id, 
            a.caso_id,
            a.id as actuacion_id,
            'vencimiento' as tipo,
            'Vencimiento: ' || substr(a.descripcion, 1, 50) || '...' as titulo,
            a.descripcion,
            a.termino_vencimiento as fecha_inicio,
            'pendiente' as estado,
            c.radicado,
            cl.nombre_completo as cliente_display_name
        FROM actuaciones a
        JOIN casos c ON a.caso_id = c.id
        LEFT JOIN clientes cl ON c.cliente_id = cl.id
        WHERE a.termino_vencimiento IS NOT NULL AND a.termino_completado = 0
        ORDER BY a.termino_vencimiento ASC
    ";
    db_select(state, query, vec![])
}
