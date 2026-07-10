import { getDb } from './db.js';

export async function getDashboardStats() {
  const db = await getDb();
  
  // Casos Activos
  const casosActivosResult = await db.select("SELECT COUNT(*) as count FROM casos WHERE estado = 'Activo'");
  const casosActivos = casosActivosResult[0]?.count || 0;

  // Casos Cerrados
  const casosCerradosResult = await db.select("SELECT COUNT(*) as count FROM casos WHERE estado = 'Cerrado' OR estado = 'Archivado'");
  const casosCerrados = casosCerradosResult[0]?.count || 0;

  // Clientes Totales
  const clientesResult = await db.select("SELECT COUNT(*) as count FROM clientes");
  const clientesTotales = clientesResult[0]?.count || 0;

  // Términos Pendientes (eventos_terminos)
  const terminosResult = await db.select("SELECT COUNT(*) as count FROM eventos_terminos WHERE estado = 'pendiente'");
  const terminosPendientes = terminosResult[0]?.count || 0;

  return {
    casosActivos,
    casosCerrados,
    clientesTotales,
    terminosPendientes
  };
}

export async function getAlertas() {
  const db = await getDb();
  
  // Obtenemos todos los eventos para el calendario, pero procesaremos alertas solo para los pendientes o próximos.
  const eventos = await db.select(`
    SELECT e.*, c.radicado, cl.nombre_completo as cliente_display_name 
    FROM eventos_terminos e
    LEFT JOIN casos c ON e.caso_id = c.id
    LEFT JOIN clientes cl ON c.cliente_id = cl.id
  `);

  const actuacionesConTermino = await db.select(`
    SELECT 
      a.id, 
      a.caso_id, 
      a.id as actuacion_id,
      'Vencimiento: ' || substr(a.descripcion, 1, 50) || '...' as titulo, 
      'vencimiento' as tipo, 
      a.termino_vencimiento as fecha_inicio, 
      NULL as hora_inicio, 
      a.descripcion, 
      'pendiente' as estado, 
      c.radicado, 
      cl.nombre_completo as cliente_display_name 
    FROM actuaciones a
    LEFT JOIN casos c ON a.caso_id = c.id
    LEFT JOIN clientes cl ON c.cliente_id = cl.id
    WHERE a.termino_vencimiento IS NOT NULL AND a.termino_completado = 0
  `);

  const eventosCombinados = [...eventos, ...actuacionesConTermino];
  
  // Obtenemos los feriados para cálculo
  const feriadosRows = await db.select('SELECT fecha FROM feriados');
  const feriadosSet = new Set(feriadosRows.map(r => r.fecha));

  const hoy = new Date();
  hoy.setHours(0,0,0,0);

  const alertas = [];

  for (const ev of eventosCombinados) {
    // Extraer solo la parte YYYY-MM-DD, manejando si viene con T o espacio
    if (!ev.fecha_inicio) continue;
    ev.fecha_inicio = ev.fecha_inicio.split('T')[0].split(' ')[0];
    
    // Solo generamos alertas para los eventos en estado pendiente
    if (ev.estado !== 'pendiente') continue;

    const fechaEv = new Date(ev.fecha_inicio + 'T00:00:00');
    
    // Si ya pasó, es un vencido (esto debería manejarlo un cron o al cargar, pero lo mostramos como alerta)
    if (fechaEv < hoy) {
      alertas.push({ ...ev, urgencia: 'vencido', dias: -1 });
      continue;
    }

    if (ev.tipo === 'termino' || ev.tipo === 'vencimiento') {
      // Calcular cuántos días hábiles faltan
      let diasHabiles = 0;
      let current = new Date(hoy);
      
      // Contamos días hábiles desde mañana hasta la fecha del evento
      current.setDate(current.getDate() + 1);
      while (current <= fechaEv) {
        const dayOfWeek = current.getDay();
        const dateStr = current.toISOString().split('T')[0];
        const isWeekend = dayOfWeek === 0 || dayOfWeek === 6;
        const isFeriado = feriadosSet.has(dateStr);
        
        if (!isWeekend && !isFeriado) {
          diasHabiles++;
        }
        current.setDate(current.getDate() + 1);
      }

      if (diasHabiles <= 3) {
        alertas.push({ ...ev, urgencia: diasHabiles <= 1 ? 'alta' : 'media', dias: diasHabiles });
      }
    } else {
      // Audiencias, reuniones, etc (días calendario)
      const diffTime = fechaEv - hoy;
      const diffDays = Math.ceil(diffTime / (1000 * 60 * 60 * 24)); 
      
      if (diffDays <= 5) {
        alertas.push({ ...ev, urgencia: diffDays <= 1 ? 'alta' : 'media', dias: diffDays });
      }
    }
  }

  // Ordenar por cercanía
  alertas.sort((a, b) => new Date(a.fecha_inicio) - new Date(b.fecha_inicio));
  
  return { alertas, todosEventos: eventosCombinados };
}
