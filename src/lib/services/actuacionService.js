import { getDb } from './db.js';

/**
 * Obtiene todas las actuaciones de un caso, ordenadas por fecha descendente.
 * @param {number} casoId
 * @returns {Promise<Array>}
 */
export async function getByCaso(casoId) {
  const db = await getDb();
  return await db.select(
    `SELECT * FROM actuaciones
     WHERE caso_id = $1
     ORDER BY fecha DESC, created_at DESC`,
    [casoId]
  );
}

/**
 * Crea una nueva actuación.
 * @param {Object} data
 * @returns {Promise<number>} El ID de la nueva actuación
 */
export async function create(data) {
  const db = await getDb();
  const archivosJson = data.archivos_vinculados || '[]';
  const result = await db.execute(
    `INSERT INTO actuaciones (
      caso_id, fecha, tipo, descripcion,
      fecha_notificacion, genera_termino, termino_vencimiento,
      dias_termino, tipo_dias, archivos_vinculados,
      created_at, updated_at
    ) VALUES (
      $1, $2, $3, $4,
      $5, $6, $7,
      $8, $9, $10,
      datetime('now'), datetime('now')
    )`,
    [
      data.caso_id,
      data.fecha || null,
      data.tipo || 'otro',
      data.descripcion || null,
      data.fecha_notificacion || null,
      data.genera_termino ? 1 : 0,
      data.termino_vencimiento || null,
      data.dias_termino || null,
      data.tipo_dias || null,
      archivosJson
    ]
  );
  return result.lastInsertId;
}

/**
 * Actualiza una actuación existente.
 * @param {number} id
 * @param {Object} data
 */
export async function update(id, data) {
  const db = await getDb();
  const fields = [];
  const params = [];
  let paramIndex = 1;

  const allowedFields = [
    'fecha', 'tipo', 'descripcion',
    'fecha_notificacion', 'genera_termino', 'termino_vencimiento',
    'dias_termino', 'tipo_dias', 'archivos_vinculados'
  ];

  for (const field of allowedFields) {
    if (field in data) {
      let value = data[field];
      // Convertir genera_termino a entero para SQLite
      if (field === 'genera_termino') {
        value = value ? 1 : 0;
      }
      fields.push(`${field} = $${paramIndex}`);
      params.push(value);
      paramIndex++;
    }
  }

  if (fields.length === 0) return;

  fields.push(`updated_at = datetime('now')`);
  params.push(id);

  await db.execute(
    `UPDATE actuaciones SET ${fields.join(', ')} WHERE id = $${paramIndex}`,
    params
  );
}

/**
 * Elimina una actuación permanentemente.
 * @param {number} id
 */
export async function remove(id) {
  const db = await getDb();
  await db.execute('DELETE FROM actuaciones WHERE id = $1', [id]);
}
