/**
 * Formatea una fecha en formato corto colombiano (ej: "15 jun 2025").
 * @param {string|null} dateStr - Cadena de fecha (YYYY-MM-DD)
 * @returns {string}
 */
export function formatDate(dateStr) {
  if (!dateStr) return '—';
  
  // Si es un timestamp numérico (ms)
  if (typeof dateStr === 'number') {
    const d = new Date(dateStr);
    if (isNaN(d.getTime())) return 'Fecha desconocida';
    return d.toLocaleDateString('es-CO', { year: 'numeric', month: 'short', day: 'numeric' });
  }

  // Si es un string (ej. fecha SQL o ISO)
  const d = new Date(dateStr.includes && dateStr.includes('T') ? dateStr : dateStr + 'T00:00:00');
  if (isNaN(d.getTime())) return 'Fecha desconocida';
  return d.toLocaleDateString('es-CO', { year: 'numeric', month: 'short', day: 'numeric' });
}

/**
 * Formatea fecha y hora en formato colombiano.
 * @param {string|null} dateStr - Cadena de fecha con hora
 * @returns {string}
 */
export function formatDateTime(dateStr) {
  if (!dateStr) return '—';
  const d = new Date(dateStr);
  if (isNaN(d.getTime())) return 'Fecha desconocida';
  return d.toLocaleDateString('es-CO', { year: 'numeric', month: 'short', day: 'numeric', hour: '2-digit', minute: '2-digit' });
}

/**
 * Formatea un monto como moneda colombiana (COP).
 * @param {number|null} amount
 * @returns {string}
 */
export function formatCurrency(amount) {
  if (amount == null) return '$0';
  return new Intl.NumberFormat('es-CO', { style: 'currency', currency: 'COP', minimumFractionDigits: 0 }).format(amount);
}

/**
 * Formatea tamaño de archivo en unidades legibles.
 * @param {number|null} bytes
 * @returns {string}
 */
export function formatFileSize(bytes) {
  if (!bytes) return '—';
  const units = ['B', 'KB', 'MB', 'GB'];
  let i = 0;
  let size = bytes;
  while (size >= 1024 && i < units.length - 1) { size /= 1024; i++; }
  return `${size.toFixed(i > 0 ? 1 : 0)} ${units[i]}`;
}

/**
 * Retorna la variante de badge según el estado del caso.
 * @param {string} estado
 * @returns {string}
 */
export function getEstadoBadgeVariant(estado) {
  if (!estado) return 'default';
  const map = { activo: 'teal', archivado: 'default', suspendido: 'amber', terminado: 'purple', consulta: 'blue' };
  return map[estado.toLowerCase()] || 'default';
}

/**
 * Retorna la etiqueta legible para un tipo de proceso.
 * @param {string} tipo
 * @returns {string}
 */
export function getTipoProcesoLabel(tipo) {
  const map = { civil: 'Civil', penal: 'Penal', laboral: 'Laboral', administrativo: 'Administrativo', familia: 'Familia', comercial: 'Comercial', constitucional: 'Constitucional', disciplinario: 'Disciplinario', consulta: 'Consulta', otro: 'Otro' };
  return map[tipo] || tipo;
}

/**
 * Retorna la etiqueta legible para un tipo de actuación.
 * @param {string} tipo
 * @returns {string}
 */
export function getTipoActuacionLabel(tipo) {
  const map = { auto: 'Auto', sentencia: 'Sentencia', notificacion: 'Notificación', audiencia: 'Audiencia', memorial: 'Memorial', anotacion: 'Anotación', otro: 'Otro' };
  return map[tipo] || tipo;
}
