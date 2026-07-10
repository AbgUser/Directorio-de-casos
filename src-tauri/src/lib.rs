// =============================================================================
// lib.rs
// Punto de entrada de la aplicación Tauri 2 para el
// Directorio General de Casos — CRM para Despacho de Abogado.
//
// Registra todos los plugins y comandos del backend.
// =============================================================================

mod commands;
mod db;

use commands::filesystem;
use commands::ensamblaje;
use db::core::{db_init, db_select, db_execute, db_exists, db_reset, db_change_password, db_close_connection, db_export_backup, db_restore_backup, generar_backup_completo, restaurar_backup_completo, eliminar_base_datos, marcar_termino_completado, obtener_agenda_caso, obtener_calendario_global, DbState};
use std::sync::Mutex;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(DbState {
            conn: Mutex::new(None),
        })
        // ── Plugins ──────────────────────────────────────────────────
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        // ── Comandos Tauri ───────────────────────────────────────────
        .invoke_handler(tauri::generate_handler![
            // Base de datos (SQLCipher)
            db_exists,
            db_init,
            db_close_connection,
            db_reset,
            db_change_password,

            db_export_backup,
            db_restore_backup,
            generar_backup_completo,
            restaurar_backup_completo,
            eliminar_base_datos,
            db_select,
            db_execute,
            marcar_termino_completado,
            obtener_agenda_caso,
            obtener_calendario_global,
            // Sistema de archivos
            filesystem::open_in_finder,
            filesystem::abrir_documento,
            filesystem::imprimir_documento,
            filesystem::create_directory,
            filesystem::copy_file_native,
            filesystem::read_file_bytes,
            filesystem::write_file_bytes,
            filesystem::generate_slug,
            filesystem::leer_directorio_expediente,
            filesystem::remove_file_native,
            filesystem::copiar_archivos_actuacion,
            // Ensamblaje
            ensamblaje::guardar_modelo_boveda,
            ensamblaje::clonar_y_abrir_modelo,
        ])
        .run(tauri::generate_context!())
        .expect("Error al iniciar la aplicación Directorio General de Casos");
}
