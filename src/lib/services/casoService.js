import { getDb } from './db.js';
import { getDisplayName } from './clienteService.js';
import { invoke } from '@tauri-apps/api/core';
import { documentDir } from '@tauri-apps/api/path';

/**
 * Obtiene todos los casos con información del cliente, ordenados por fecha de creación descendente.
 * @returns {Promise<Array>}
 */
export async function getAll() {
  const db = await getDb();
  return await db.select(
    `SELECT c.*,
       cl.tipo_persona,
       cl.nombre_completo AS cliente_display_name
     FROM casos c
     LEFT JOIN clientes cl ON c.cliente_id = cl.id
     ORDER BY c.created_at DESC`
  );
}

/**
 * Obtiene un caso por su ID con información del cliente.
 * @param {number} id
 * @returns {Promise<Object|null>}
 */
export async function getById(id) {
  const db = await getDb();
  const rows = await db.select(
    `SELECT c.*,
       cl.tipo_persona,
       cl.nombre_completo AS cliente_display_name,
       cl.identificacion AS cliente_identificacion,
       cl.email AS cliente_email,
       cl.telefono AS cliente_telefono
     FROM casos c
     LEFT JOIN clientes cl ON c.cliente_id = cl.id
     WHERE c.id = $1`,
    [id]
  );
  return rows.length > 0 ? rows[0] : null;
}

/**
 * Genera un slug seguro para carpetas a partir de un texto.
 * @param {string} text
 * @returns {Promise<string>}
 */
async function generateSlug(text) {
  try {
    return await invoke('generate_slug', { name: text });
  } catch {
    // Fallback: generar slug local básico
    return text
      .toLowerCase()
      .normalize('NFD')
      .replace(/[\u0300-\u036f]/g, '')
      .replace(/[^a-z0-9]+/g, '-')
      .replace(/^-+|-+$/g, '');
  }
}

/**
 * Crea un nuevo caso.
 * @param {Object} data
 * @returns {Promise<number>} El ID del nuevo caso
 */
export async function create(data) {
  const db = await getDb();

  // Generar slug del caso
  const slugSource = data.radicado || data.tipo_proceso || `caso-${Date.now()}`;
  const casoSlug = await generateSlug(slugSource);

  // Obtener información del cliente para la carpeta
  let clientSlug = 'sin-cliente';
  if (data.cliente_id) {
    const clientRows = await db.select('SELECT * FROM clientes WHERE id = $1', [data.cliente_id]);
    if (clientRows.length > 0) {
      const clientName = getDisplayName(clientRows[0]);
      clientSlug = await generateSlug(clientName);
    }
  }

  // Construir ruta relativa de la carpeta de documentos
  const carpetaRelativa = `${clientSlug}/${casoSlug}`;

  // Crear la carpeta de documentos
  try {
    const basePath = await documentDir();
    const fullPath = `${basePath}/Directorio_Casos/${carpetaRelativa}`;
    await invoke('create_directory', { path: fullPath });
  } catch (err) {
    console.warn('No se pudo crear la carpeta de documentos:', err);
  }

  const result = await db.execute(
    `INSERT INTO casos (
      cliente_id, radicado, slug, tipo_proceso,
      juzgado, contraparte, estado,
      descripcion, notas_internas,
      fecha_inicio, fecha_terminacion, carpeta_documentos,
      categoria_expediente,
      created_at, updated_at
    ) VALUES (
      $1, $2, $3, $4,
      $5, $6, $7,
      $8, $9,
      $10, $11, $12,
      $13,
      datetime('now'), datetime('now')
    )`,
    [
      data.cliente_id || null,
      data.radicado || null,
      casoSlug,
      data.tipo_proceso || null,
      data.juzgado || null,
      data.contraparte || null,
      data.estado || 'Activo',
      data.descripcion || null,
      data.notas_internas || null,
      data.fecha_inicio || null,
      data.fecha_terminacion || null,
      carpetaRelativa,
      data.categoria_expediente || 'proceso'
    ]
  );
  return result.lastInsertId;
}

/**
 * Actualiza un caso existente.
 * @param {number} id
 * @param {Object} data
 */
export async function update(id, data) {
  const db = await getDb();
  const fields = [];
  const params = [];
  let paramIndex = 1;

  const allowedFields = [
    'cliente_id', 'radicado', 'tipo_proceso',
    'juzgado', 'contraparte', 'estado',
    'descripcion', 'notas_internas',
    'fecha_inicio', 'fecha_terminacion',
    'categoria_expediente'
  ];

  for (const field of allowedFields) {
    if (field in data) {
      fields.push(`${field} = $${paramIndex}`);
      params.push(data[field]);
      paramIndex++;
    }
  }

  if (fields.length === 0) return;

  fields.push(`updated_at = datetime('now')`);
  params.push(id);

  await db.execute(
    `UPDATE casos SET ${fields.join(', ')} WHERE id = $${paramIndex}`,
    params
  );
}

/**
 * Elimina un caso permanentemente de la base de datos.
 * Nota: Los archivos físicos no se borran automáticamente.
 * @param {number} id
 */
export async function remove(id) {
  const db = await getDb();
  await db.execute(
    `DELETE FROM casos WHERE id = $1`,
    [id]
  );
}

/**
 * Busca casos por radicado, demandante, demandado o slug.
 * @param {string} query
 * @returns {Promise<Array>}
 */
export async function search(query) {
  const db = await getDb();
  const term = `%${query}%`;
  return await db.select(
    `SELECT c.*,
       cl.nombre_completo AS cliente_display_name
     FROM casos c
     LEFT JOIN clientes cl ON c.cliente_id = cl.id
     WHERE c.radicado LIKE $1
       OR c.contraparte LIKE $2
       OR c.slug LIKE $3
     ORDER BY c.created_at DESC`,
    [term, term, term]
  );
}

/**
 * Obtiene todos los casos de un cliente específico.
 * @param {number} clienteId
 * @returns {Promise<Array>}
 */
export async function getByCliente(clienteId) {
  const db = await getDb();
  return await db.select(
    `SELECT c.*,
       cl.nombre_completo AS cliente_display_name
     FROM casos c
     LEFT JOIN clientes cl ON c.cliente_id = cl.id
     WHERE c.cliente_id = $1
     ORDER BY c.created_at DESC`,
    [clienteId]
  );
}
