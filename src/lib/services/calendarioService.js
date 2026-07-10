import { getDb } from './db.js';

/**
 * Obtiene todos los feriados almacenados en SQLite.
 * @returns {Promise<Array>}
 */
export async function getFeriados() {
  const db = await getDb();
  return await db.select(`SELECT * FROM feriados ORDER BY fecha ASC`);
}

/**
 * Calcula un término judicial sumando días hábiles a una fecha de inicio.
 * Descarta sábados, domingos y días festivos registrados.
 * @param {string} fechaInicio - Fecha de inicio en formato YYYY-MM-DD
 * @param {number} dias - Cantidad de días a sumar
 * @param {string} tipoDias - 'habiles' o 'calendario'
 * @returns {Promise<string>} Fecha resultante en formato YYYY-MM-DD
 */
export async function calcularTermino(fechaInicio, dias, tipoDias = 'habiles') {
  if (tipoDias === 'calendario') {
    const d = new Date(fechaInicio + 'T00:00:00');
    d.setDate(d.getDate() + dias);
    return d.toISOString().split('T')[0];
  }

  // Cargar feriados de la BD
  const db = await getDb();
  const feriadosRows = await db.select('SELECT fecha FROM feriados');
  const feriadosSet = new Set(feriadosRows.map(r => r.fecha));

  const current = new Date(fechaInicio + 'T00:00:00');
  let diasSumados = 0;

  // Empezar a contar desde el día siguiente a la fecha de inicio,
  // como dicta el Código General del Proceso para los términos en días.
  current.setDate(current.getDate() + 1);

  while (diasSumados < dias) {
    const dayOfWeek = current.getDay(); // 0 = Domingo, 6 = Sábado
    const dateStr = current.toISOString().split('T')[0];

    const isWeekend = dayOfWeek === 0 || dayOfWeek === 6;
    const isFeriado = feriadosSet.has(dateStr);

    if (!isWeekend && !isFeriado) {
      diasSumados++;
    }

    if (diasSumados < dias) {
      current.setDate(current.getDate() + 1);
    }
  }

  return current.toISOString().split('T')[0];
}

export async function getEventosByCaso(casoId) {
  const db = await getDb();
  return await db.select(
    `SELECT * FROM eventos_terminos WHERE caso_id = $1 ORDER BY fecha_inicio ASC, hora_inicio ASC`,
    [casoId]
  );
}

export async function createEvento(data) {
  const db = await getDb();
  const res = await db.execute(
    `INSERT INTO eventos_terminos (
      caso_id, titulo, tipo, fecha_inicio, hora_inicio, descripcion, estado
    ) VALUES ($1, $2, $3, $4, $5, $6, $7)`,
    [
      data.caso_id || null,
      data.titulo,
      data.tipo,
      data.fecha_inicio,
      data.hora_inicio || null,
      data.descripcion || null,
      data.estado || 'pendiente'
    ]
  );
}

export async function deleteEvento(id) {
  const db = await getDb();
  await db.execute(`DELETE FROM eventos_terminos WHERE id = $1`, [id]);
}

export async function updateEventoEstado(id, estado) {
  const db = await getDb();
  await db.execute(
    `UPDATE eventos_terminos SET estado = $1, updated_at = datetime('now', 'localtime') WHERE id = $2`,
    [estado, id]
  );
}
