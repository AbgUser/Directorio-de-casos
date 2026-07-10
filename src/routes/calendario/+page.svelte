<script>
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import * as calendarioService from '$lib/services/calendarioService.js';
  import { getDb } from '$lib/services/db.js';
  import Button from '$lib/components/ui/Button.svelte';
  import Modal from '$lib/components/ui/Modal.svelte';
  import TextInput from '$lib/components/ui/TextInput.svelte';
  import Select from '$lib/components/ui/Select.svelte';
  import Badge from '$lib/components/ui/Badge.svelte';

  let eventos = $state([]);
  let loading = $state(true);

  // Form states
  let showForm = $state(false);
  let formTipo = $state('recordatorio');
  let formTitulo = $state('');
  let formFecha = $state('');
  let formHora = $state('');
  let formDescripcion = $state('');

  async function loadData() {
    loading = true;
    try {
      const terminos = await invoke('obtener_calendario_global');
      
      // Obtain normal events to combine them here.
      const db = await getDb();
      const normales = await db.select(`
        SELECT e.*, c.radicado, cl.nombre_completo as cliente_display_name 
        FROM eventos_terminos e
        LEFT JOIN casos c ON e.caso_id = c.id
        LEFT JOIN clientes cl ON c.cliente_id = cl.id
      `);

      eventos = [...normales, ...terminos].sort((a, b) => new Date(a.fecha_inicio) - new Date(b.fecha_inicio));
    } catch (e) {
      console.error('Error loading agenda:', e);
    } finally {
      loading = false;
    }
  }

  onMount(loadData);

  async function handleSave(e) {
    e.preventDefault();
    if (!formTitulo || !formFecha) return;
    try {
      await calendarioService.createEvento({
        caso_id: null, // Evento general sin caso
        titulo: formTitulo,
        tipo: formTipo,
        fecha_inicio: formFecha,
        hora_inicio: formHora,
        descripcion: formDescripcion,
        estado: 'pendiente'
      });
      showForm = false;
      formTitulo = ''; formFecha = ''; formHora = ''; formDescripcion = ''; formTipo = 'recordatorio';
      await loadData();
    } catch (error) {
      console.error(error);
      alert('Error al guardar el recordatorio.');
    }
  }

  async function handleDelete(ev) {
    if (ev.tipo === 'vencimiento' && ev.actuacion_id) {
       alert('Los vencimientos de términos no se pueden eliminar desde aquí. Vaya al detalle del caso y elimine la actuación respectiva.');
       return;
    }
    if (!confirm('¿Desea eliminar este evento?')) return;
    try {
      await calendarioService.deleteEvento(ev.id);
      await loadData();
    } catch (e) {
      console.error(e);
    }
  }

  async function handleToggleStatus(ev, currentStatus) {
    if (ev.tipo === 'vencimiento' && ev.actuacion_id) {
      if (currentStatus === 'pendiente') {
        try {
          await invoke('marcar_termino_completado', { id: ev.actuacion_id });
          await loadData();
        } catch(e) { console.error(e); }
      }
      return;
    }

    const newStatus = currentStatus === 'pendiente' ? 'cumplido' : 'pendiente';
    try {
      await calendarioService.updateEventoEstado(ev.id, newStatus);
      await loadData();
    } catch (e) {
      console.error(e);
    }
  }
</script>

<div class="calendario-page fade-in">
  <div class="page-header">
    <div>
      <h1 class="page-title">Calendario Global</h1>
      <p class="page-subtitle">Gestiona todos los vencimientos, audiencias y recordatorios del despacho.</p>
    </div>
    <div class="header-actions">
      <Button variant="primary" onclick={() => showForm = true}>
        <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" style="margin-right:8px;"><circle cx="12" cy="12" r="10"/><line x1="12" y1="8" x2="12" y2="16"/><line x1="8" y1="12" x2="16" y2="12"/></svg>
        Añadir Recordatorio
      </Button>
    </div>
  </div>

  <div class="calendario-content">
    {#if loading}
      <div class="loading-state"><div class="spinner"></div></div>
    {:else if eventos.length === 0}
      <div class="empty-state">
        <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" class="empty-icon"><rect x="3" y="4" width="18" height="18" rx="2" ry="2"/><line x1="16" y1="2" x2="16" y2="6"/><line x1="8" y1="2" x2="8" y2="6"/><line x1="3" y1="10" x2="21" y2="10"/></svg>
        <p class="empty-text">El calendario está vacío</p>
        <p class="empty-subtext">No tienes ningún compromiso, audiencia ni tarea pendiente registrada.</p>
      </div>
    {:else}
      <div class="agenda-list">
        {#each eventos as ev}
          <div class="agenda-item {ev.estado === 'cumplido' ? 'cumplido' : ''}">
            <div class="item-date">
              <span class="day">{new Date(ev.fecha_inicio + 'T00:00:00').getDate()}</span>
              <span class="month">{new Date(ev.fecha_inicio + 'T00:00:00').toLocaleString('es-CO', {month: 'short'}).toUpperCase()}</span>
            </div>
            <div class="item-content">
              <div class="item-header">
                <h4>{ev.titulo}</h4>
                <div class="badges">
                  <Badge variant={ev.estado === 'cumplido' ? 'default' : (ev.tipo === 'audiencia' ? 'amber' : ev.tipo === 'vencimiento' ? 'red' : ev.tipo === 'recordatorio' ? 'purple' : 'teal')}>
                    {ev.tipo.toUpperCase()}
                  </Badge>
                  {#if ev.radicado && ev.caso_id}
                    <a href="/casos/{ev.caso_id}" style="text-decoration: none;">
                      <Badge variant="default">Exp: {ev.radicado}</Badge>
                    </a>
                  {:else}
                    <Badge variant="default">General</Badge>
                  {/if}
                </div>
              </div>
              <div class="item-details">
                {#if ev.hora_inicio}
                  <span class="time-badge">
                    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><polyline points="12 6 12 12 16 14"/></svg>
                    {ev.hora_inicio}
                  </span>
                {/if}
                {#if ev.descripcion}
                  <p class="desc">{ev.descripcion}</p>
                {/if}
                {#if ev.cliente_display_name}
                  <p class="cliente-info">
                    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"/><circle cx="12" cy="7" r="4"/></svg>
                    {ev.cliente_display_name}
                  </p>
                {/if}
              </div>
            </div>
            <div class="item-actions">
              <button class="icon-btn check-btn" onclick={() => handleToggleStatus(ev, ev.estado)} title={ev.estado === 'pendiente' ? 'Marcar como Cumplido / Solventar' : 'Marcar como Pendiente'}>
                {#if ev.estado === 'pendiente'}
                  <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="20 6 9 17 4 12"/></svg>
                {:else}
                  <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M3 12a9 9 0 1 0 9-9 9.75 9.75 0 0 0-6.74 2.74L3 8"/><path d="M3 3v5h5"/></svg>
                {/if}
              </button>
              {#if ev.tipo !== 'vencimiento'}
                <button class="icon-btn delete-btn" onclick={() => handleDelete(ev)} title="Eliminar">
                  <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M3 6h18"/><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/></svg>
                </button>
              {/if}
            </div>
          </div>
        {/each}
      </div>
    {/if}
  </div>
</div>

<Modal open={showForm} title="Añadir Recordatorio" onclose={() => showForm = false}>
  <form onsubmit={handleSave} class="form-grid">
    <Select 
      label="Tipo" 
      bind:value={formTipo}
      options={[
        {value: 'recordatorio', label: 'Recordatorio General'},
        {value: 'audiencia', label: 'Reunión / Audiencia'},
        {value: 'vencimiento', label: 'Vencimiento'},
        {value: 'otro', label: 'Otra Tarea'}
      ]}
    />
    
    <div style="grid-column: 1 / -1;">
      <TextInput 
        label="Título del Recordatorio" 
        bind:value={formTitulo} 
        placeholder="Ej: Revisar correos de la fiscalía"
        required 
      />
    </div>

    <div style="display: flex; flex-direction: column; gap: var(--sp-1);">
      <label class="field-label">Fecha <span style="color: var(--accent-red); margin-left: 2px;">*</span></label>
      <input type="date" class="text-input" style="width: 100%; padding: var(--sp-2) var(--sp-3); background: var(--bg-input); border: 1px solid var(--border-default); border-radius: var(--radius-md); color: var(--text-primary); font-family: var(--font-sans); font-size: 0.875rem;" bind:value={formFecha} required />
    </div>
    
    <div style="display: flex; flex-direction: column; gap: var(--sp-1);">
      <label class="field-label">Hora (Opcional)</label>
      <input type="time" class="text-input" style="width: 100%; padding: var(--sp-2) var(--sp-3); background: var(--bg-input); border: 1px solid var(--border-default); border-radius: var(--radius-md); color: var(--text-primary); font-family: var(--font-sans); font-size: 0.875rem;" bind:value={formHora} />
    </div>

    <div style="grid-column: 1 / -1;">
      <label class="field-label" for="desc">Descripción (Opcional)</label>
      <textarea 
        id="desc"
        class="field-input" 
        bind:value={formDescripcion} 
        rows="3"
        placeholder="Detalles adicionales..."
      ></textarea>
    </div>

    <div class="form-actions" style="grid-column: 1 / -1; margin-top: var(--sp-4);">
      <Button variant="ghost" type="button" onclick={() => showForm = false}>Cancelar</Button>
      <Button variant="primary" type="submit">Guardar Recordatorio</Button>
    </div>
  </form>
</Modal>

<style>
  .calendario-page {
    padding: var(--sp-6);
    max-width: 1000px;
    margin: 0 auto;
    height: calc(100vh - var(--topbar-height));
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .page-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    margin-bottom: var(--sp-6);
    flex-shrink: 0;
  }

  .page-title {
    font-size: 1.75rem;
    font-weight: 700;
    color: var(--text-primary);
    margin-bottom: var(--sp-2);
  }

  .page-subtitle {
    color: var(--text-secondary);
    font-size: 0.9375rem;
  }

  .calendario-content {
    flex: 1;
    overflow-y: auto;
    background: var(--bg-surface);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-lg);
    padding: var(--sp-5);
  }

  .agenda-list {
    display: flex;
    flex-direction: column;
    gap: var(--sp-3);
  }

  .agenda-item {
    display: flex;
    gap: var(--sp-4);
    padding: var(--sp-4);
    background: var(--bg-elevated);
    border: 1px solid var(--border-default);
    border-radius: var(--radius-md);
    transition: all var(--ease-fast);
  }

  .agenda-item:hover {
    border-color: var(--border-focus);
    transform: translateY(-1px);
    box-shadow: var(--shadow-sm);
  }

  .agenda-item.cumplido {
    opacity: 0.6;
    border-style: dashed;
  }

  .agenda-item.cumplido .item-date {
    background: var(--bg-input);
    color: var(--text-muted);
  }

  .item-date {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    min-width: 60px;
    height: 60px;
    background: var(--accent-blue-soft);
    color: var(--accent-blue);
    border-radius: var(--radius-md);
    flex-shrink: 0;
  }

  .item-date .day {
    font-size: 1.25rem;
    font-weight: 700;
    line-height: 1;
  }

  .item-date .month {
    font-size: 0.7rem;
    font-weight: 600;
    margin-top: 2px;
  }

  .item-content {
    flex: 1;
    min-width: 0;
  }

  .item-header {
    display: flex;
    align-items: center;
    gap: var(--sp-3);
    margin-bottom: var(--sp-1);
    flex-wrap: wrap;
  }

  .item-header h4 {
    margin: 0;
    font-size: 1rem;
    color: var(--text-primary);
  }

  .badges {
    display: flex;
    gap: var(--sp-2);
  }

  .item-details {
    display: flex;
    flex-direction: column;
    gap: var(--sp-1);
  }

  .time-badge, .cliente-info {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    font-size: 0.8125rem;
    color: var(--text-secondary);
    font-family: var(--font-mono);
  }

  .desc {
    font-size: 0.875rem;
    color: var(--text-secondary);
    margin-top: var(--sp-1);
  }

  .item-actions {
    display: flex;
    gap: var(--sp-2);
    align-items: flex-start;
  }

  .icon-btn {
    width: 32px;
    height: 32px;
    display: flex;
    align-items: center;
    justify-content: center;
    border: none;
    border-radius: var(--radius-sm);
    background: var(--bg-input);
    color: var(--text-secondary);
    cursor: pointer;
    transition: all 0.2s;
  }

  .check-btn:hover {
    background: var(--accent-green-soft);
    color: var(--accent-green);
  }

  .delete-btn:hover {
    background: var(--accent-red-soft);
    color: var(--accent-red);
  }

  .form-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: var(--sp-4);
  }

  .field-label {
    display: block;
    margin-bottom: var(--sp-2);
    font-size: 0.875rem;
    font-weight: 500;
    color: var(--text-secondary);
  }

  .field-input {
    width: 100%;
    padding: 0.625rem 0.875rem;
    background: var(--bg-input);
    border: 1px solid var(--border-default);
    border-radius: var(--radius-md);
    color: var(--text-primary);
    font-family: inherit;
    font-size: 0.9375rem;
    transition: all var(--ease-fast);
  }

  .field-input:focus {
    outline: none;
    border-color: var(--accent-blue);
    box-shadow: var(--shadow-glow-blue);
  }

  textarea.field-input {
    resize: vertical;
    min-height: 80px;
  }

  .form-actions {
    display: flex;
    justify-content: flex-end;
    gap: var(--sp-3);
  }
</style>
