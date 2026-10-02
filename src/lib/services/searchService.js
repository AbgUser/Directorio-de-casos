import { getDb } from './db.js';

export async function globalSearch(query) {
  if (!query || query.trim().length < 2) return { clientes: [], casos: [] };
  
  const db = await getDb();
  const searchPattern = `%${query.trim()}%`;
  
  let clientes = [];
  let casos = [];

  try {
    // Buscar en clientes (nombre o identificación)
    clientes = await db.select(
      `SELECT id, nombre_completo, identificacion, email
       FROM clientes
       WHERE activo = 1 AND (nombre_completo LIKE $1 OR identificacion LIKE $2)
       LIMIT 5`,
      [searchPattern, searchPattern]
    );
  } catch (e) {
    console.error('Error buscando clientes:', e);
  }

  try {
    // Buscar en casos (radicado, descripción, slug, cliente)
    casos = await db.select(
      `SELECT c.id, c.radicado, c.slug, c.tipo_proceso, cl.nombre_completo as cliente_nombre
       FROM casos c
       LEFT JOIN clientes cl ON c.cliente_id = cl.id
       WHERE c.radicado LIKE $1 OR c.slug LIKE $2 OR c.descripcion LIKE $3 OR cl.nombre_completo LIKE $4
       LIMIT 5`,
      [searchPattern, searchPattern, searchPattern, searchPattern]
    );
  } catch (e) {
    console.error('Error buscando casos:', e);
  }

  return { clientes, casos };
}
