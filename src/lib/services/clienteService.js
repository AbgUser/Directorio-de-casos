import { getDb } from './db.js';

/**
 * Obtiene todos los clientes activos, ordenados por nombre/apellido (natural)
 * o razón social (jurídica).
 * @returns {Promise<Array>}
 */
export async function getAll() {
  const db = await getDb();
  return await db.select(
    `SELECT * FROM clientes
     WHERE activo = 1
     ORDER BY nombre_completo COLLATE NOCASE ASC`
  );
}

/**
 * Obtiene un cliente por su ID.
 * @param {number} id
 * @returns {Promise<Object|null>}
 */
export async function getById(id) {
  const db = await getDb();
  const rows = await db.select('SELECT * FROM clientes WHERE id = $1', [id]);
  return rows.length > 0 ? rows[0] : null;
}

/**
 * Crea un nuevo cliente.
 * @param {Object} data
 * @returns {Promise<number>} El ID del nuevo cliente
 */
export async function create(data) {
  const db = await getDb();
  const result = await db.execute(
    `INSERT INTO clientes (
      tipo_persona, nombre_completo, tipo_identificacion, identificacion,
      telefono, email, direccion, notas,
      activo, created_at, updated_at
    ) VALUES (
      $1, $2, $3, $4,
      $5, $6, $7, $8,
      1, datetime('now'), datetime('now')
    )`,
    [
      data.tipo_persona,
      data.nombre_completo || null,
      data.tipo_identificacion || null,
      data.identificacion || null,
      data.telefono || null,
      data.email || null,
      data.direccion || null,
      data.notas || null
    ]
  );
  return result.lastInsertId;
}

/**
 * Actualiza un cliente existente.
 * @param {number} id
 * @param {Object} data
 */
export async function update(id, data) {
  const db = await getDb();
  const fields = [];
  const params = [];
  let paramIndex = 1;

  const allowedFields = [
    'tipo_persona', 'nombre_completo', 'tipo_identificacion', 'identificacion',
    'telefono', 'email', 'direccion', 'notas'
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
    `UPDATE clientes SET ${fields.join(', ')} WHERE id = $${paramIndex}`,
    params
  );
}

/**
 * Elimina un cliente permanentemente. Lanza error si tiene casos asociados.
 * @param {number} id
 */
export async function remove(id) {
  const db = await getDb();

  // Verificar si tiene casos asociados
  const activeCases = await db.select(
    `SELECT COUNT(*) as count FROM casos WHERE cliente_id = $1`,
    [id]
  );

  if (activeCases[0].count > 0) {
    throw new Error('No se puede eliminar el cliente porque tiene casos asociados. Elimine los casos primero.');
  }

  await db.execute(
    `DELETE FROM clientes WHERE id = $1`,
    [id]
  );
}

/**
 * Busca clientes por nombre, apellido, cédula, razón social o NIT.
 * @param {string} query
 * @returns {Promise<Array>}
 */
export async function search(query) {
  const db = await getDb();
  const term = `%${query}%`;
  return await db.select(
    `SELECT * FROM clientes
     WHERE activo = 1
       AND (
         nombre_completo LIKE $1
         OR identificacion LIKE $2
       )
     ORDER BY nombre_completo COLLATE NOCASE ASC`,
    [term, term]
  );
}

/**
 * Retorna el nombre de visualización del cliente.
 * @param {Object} cliente
 * @returns {string}
 */
export function getDisplayName(cliente) {
  if (!cliente) return '';
  return cliente.nombre_completo || '';
}
