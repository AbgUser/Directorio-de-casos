import { getDb } from './db.js';

export async function globalSearch(query) {
  if (!query || query.trim().length < 2) return { clientes: [], casos: [] };
  
  const db = await getDb();
  const searchPattern = `%${query.trim()}%`;
  
  // Buscar en clientes (nombre o identificación)
  const clientes = await db.select(`
    SELECT id, nombre_completo, identificacion, email
    FROM clientes
    WHERE nombre_completo LIKE ? OR identificacion LIKE ?
    LIMIT 5
  `, [searchPattern, searchPattern]);
  
  // Buscar en casos (radicado, título/descripción, slug)
  const casos = await db.select(`
    SELECT c.id, c.radicado, c.slug, c.tipo_proceso, cl.nombre_completo as cliente_nombre
    FROM casos c
    LEFT JOIN clientes cl ON c.cliente_id = cl.id
    WHERE c.radicado LIKE ? OR c.slug LIKE ? OR c.descripcion LIKE ? OR cl.nombre_completo LIKE ?
    LIMIT 5
  `, [searchPattern, searchPattern, searchPattern, searchPattern]);
  
  return { clientes, casos };
}
