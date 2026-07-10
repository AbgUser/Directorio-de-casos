import { getDb } from './db.js';
import { invoke } from '@tauri-apps/api/core';
import * as casoService from './casoService.js';
import { documentDir } from '@tauri-apps/api/path';

export async function getAll() {
  const db = await getDb();
  return await db.select(
    `SELECT * FROM modelos_legales ORDER BY nombre ASC`
  );
}

export async function create(data) {
  // Guardar físico en la bóveda
  const rutaBoveda = await invoke('guardar_modelo_boveda', { rutaOrigen: data.ruta_archivo });
  
  const db = await getDb();
  const result = await db.execute(
    `INSERT INTO modelos_legales (nombre, tipo, ruta_archivo, created_at)
     VALUES ($1, $2, $3, datetime('now'))`,
    [data.nombre, data.tipo, rutaBoveda]
  );
  return result.lastInsertId;
}

export async function remove(id) {
  const db = await getDb();
  await db.execute('DELETE FROM modelos_legales WHERE id = $1', [id]);
}

export async function clonarYabrir(modelo, casoId, tipoActuacion) {
  const caso = await casoService.getById(casoId);
  const carpetaRelativa = caso.carpeta_documentos;
  
  const basePath = await documentDir();
  const carpetaDestino = `${basePath}/Directorio_Casos/${carpetaRelativa}`;
  
  const today = new Date().toISOString().split('T')[0];
  const extension = modelo.ruta_archivo.split('.').pop();
  const nombreLimpio = tipoActuacion.replace(/[^a-zA-Z0-9]/g, '_');
  const nuevoNombre = `${today}_${nombreLimpio}.${extension}`;

  const result = await invoke('clonar_y_abrir_modelo', {
    rutaModelo: modelo.ruta_archivo,
    carpetaDestino,
    nuevoNombre
  });
  
  return result; 
}

export async function verificarYLimpiar(rutaArchivo, mtimeOriginal, sizeOriginal) {
  return await invoke('verificar_y_limpiar_clon', {
    rutaArchivo,
    mtimeOriginal,
    sizeOriginal
  });
}
