import { getDb } from './db.js';
import { copyFile, stat, exists, mkdir } from '@tauri-apps/plugin-fs';
import { invoke } from '@tauri-apps/api/core';
import { documentDir } from '@tauri-apps/api/path';

/**
 * Obtiene la ruta base de documentos: <DocumentDir>/Directorio_Casos/
 * @returns {Promise<string>}
 */
export async function getBasePath() {
  const base = await documentDir();
  return `${base}/Directorio_Casos/`;
}

/**
 * Obtiene todos los documentos de un caso directamente de la carpeta física.
 * @param {number} casoId
 * @returns {Promise<Array>}
 */
export async function getByCaso(casoId) {
  try {
    const db = await getDb();
    const casoRows = await db.select(
      'SELECT carpeta_documentos FROM casos WHERE id = $1',
      [casoId]
    );
    
    if (casoRows.length === 0) return [];
    
    const carpetaDocumentos = casoRows[0].carpeta_documentos;
    const basePath = await getBasePath();
    const rutaAbsoluta = `${basePath}${carpetaDocumentos}`;
    
    // Llamar al comando Rust para leer el directorio físico
    return await invoke('leer_directorio_expediente', { rutaAbsoluta });
  } catch (error) {
    console.error("Error leyendo expediente:", error);
    return []; // En caso de error (ej. comando no existe o IPC falla), devolvemos lista vacía
  }
}

/**
 * Sube archivos al caso, copiándolos a la carpeta de documentos y registrándolos en BD.
 * @param {number} casoId
 * @param {string[]} filePaths - Rutas absolutas de los archivos a copiar
 * @returns {Promise<number[]>} IDs de los documentos creados
 */
export async function upload(casoId, filePaths) {
  const db = await getDb();
  const basePath = await getBasePath();

  // Obtener la carpeta de documentos del caso
  const casoRows = await db.select(
    'SELECT carpeta_documentos FROM casos WHERE id = $1',
    [casoId]
  );

  if (casoRows.length === 0) {
    throw new Error('Caso no encontrado');
  }

  const carpetaDocumentos = casoRows[0].carpeta_documentos;
  const destDir = `${basePath}${carpetaDocumentos}`;

  // Asegurar que la carpeta destino existe
  const dirExists = await exists(destDir);
  if (!dirExists) {
    await mkdir(destDir, { recursive: true });
  }

  const insertedIds = [];

  for (const filePath of filePaths) {
    // Extraer nombre del archivo de la ruta
    const fileName = filePath.split('/').pop() || filePath.split('\\').pop();
    const destPath = `${destDir}/${fileName}`;

    // Copiar archivo físicamente
    await copyFile(filePath, destPath);
    insertedIds.push(1); // Mantenemos el retorno array para Svelte
  }

  return insertedIds;
}

/**
 * Elimina un documento (elimina el archivo físico).
 * @param {string} rutaAbsoluta
 */
export async function removeFile(rutaAbsoluta) {
  await invoke('remove_file_native', { path: rutaAbsoluta });
}

/**
 * Abre un archivo con la aplicación predeterminada del sistema.
 * @param {string} rutaAbsoluta
 */
export async function openFile(rutaAbsoluta) {
  await invoke('abrir_documento', { path: rutaAbsoluta });
}

/**
 * Abre la ubicación del archivo en Finder.
 * @param {string} rutaAbsoluta
 */
export async function openInFinder(rutaAbsoluta) {
  await invoke('open_in_finder', { path: rutaAbsoluta });
}

/**
 * Envía el archivo a imprimir.
 * @param {string} rutaAbsoluta
 */
export async function printFile(rutaAbsoluta) {
  await invoke('imprimir_documento', { path: rutaAbsoluta });
}

/**
 * Determina el tipo de archivo basado en su extensión.
 * @param {string} extension
 * @returns {string}
 */
function getTipoArchivo(extension) {
  const tipos = {
    pdf: 'application/pdf',
    doc: 'application/msword',
    docx: 'application/vnd.openxmlformats-officedocument.wordprocessingml.document',
    xls: 'application/vnd.ms-excel',
    xlsx: 'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet',
    jpg: 'image/jpeg',
    jpeg: 'image/jpeg',
    png: 'image/png',
    gif: 'image/gif',
    txt: 'text/plain',
    html: 'text/html',
    zip: 'application/zip',
    rar: 'application/x-rar-compressed',
    mp3: 'audio/mpeg',
    mp4: 'video/mp4'
  };
  return tipos[extension] || 'application/octet-stream';
}
