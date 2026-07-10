<script>
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import * as calendarioService from '$lib/services/calendarioService.js';
  import Button from '$lib/components/ui/Button.svelte';
  import Modal from '$lib/components/ui/Modal.svelte';
  import TextInput from '$lib/components/ui/TextInput.svelte';
  import Select from '$lib/components/ui/Select.svelte';
  import Badge from '$lib/components/ui/Badge.svelte';

  let { casoId } = $props();

  let eventos = $state([]);
  let loading = $state(true);

  // Form states
  let showForm = $state(false);
  let formTipo = $state('audiencia');
  let formTitulo = $state('');
  let formFecha = $state('');
  let formHora = $state('');
  let formDescripcion = $state('');

  async function loadData() {
    loading = true;
    try {
      const eventosNormales = await calendarioService.getEventosByCaso(casoId);
      const terminosActuaciones = await invoke('obtener_agenda_caso', { idCaso: casoId });
      
      eventos = [...eventosNormales, ...terminosActuaciones]
        .sort((a, b) => new Date(a.fecha_inicio) - new Date(b.fecha_inicio));
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
        caso_id: casoId,
        titulo: formTitulo,
        tipo: formTipo,
        fecha_inicio: formFecha,
        hora_inicio: formHora,
        descripcion: formDescripcion,
        estado: 'pendiente'
      });
      showForm = false;
      formTitulo = ''; formFecha = ''; formHora = ''; formDescripcion = ''; formTipo = 'audiencia';
      await loadData();
    } catch (error) {
      console.error(error);
      alert('Error al guardar el evento en la agenda.');
    }
  }

  async function handleDelete(ev) {
    if (ev.tipo === 'vencimiento' && ev.actuacion_id) {
       alert('Los vencimientos de términos no se pueden eliminar desde aquí. Vaya a la actuación y elimínela.');
       return;
    }
    if (!confirm('¿Desea eliminar este evento de la agenda?')) return;
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

  const formatFecha = (d) => new Date(d.includes('T') ? d : d + 'T00:00:00').toLocaleDateString('es-CO', { year: 'numeric', month: 'long', day: 'numeric' });
</script>

<div class="agenda-container">
  <div class="header-actions">
    <h3>Próximos Compromisos y Audiencias</h3>
    <Button variant="primary" onclick={() => showForm = true}>
      Programar Evento
    </Button>
  </div>

  {#if loading}
    <div class="loading"><div class="spinner"></div></div>
  {:else if eventos.length === 0}
    <div class="empty-state">
      No hay audiencias ni compromisos programados para este caso.
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
              <Badge variant={ev.estado === 'cumplido' ? 'default' : (ev.tipo === 'audiencia' ? 'amber' : ev.tipo === 'vencimiento' ? 'red' : 'teal')}>
                {ev.tipo.toUpperCase()}
              </Badge>
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

<Modal open={showForm} title="Programar Nuevo Evento" onclose={() => showForm = false}>
  <form onsubmit={handleSave} class="form-grid">
    <Select 
      label="Tipo de Evento" 
      bind:value={formTipo}
      options={[
        {value: 'audiencia', label: 'Audiencia'},
        {value: 'otro', label: 'Reunión con Cliente'},
        {value: 'termino', label: 'Vencimiento de Término'},
        {value: 'recordatorio', label: 'Recordatorio General'}
      ]}
    />
    <TextInput label="Título Corto" bind:value={formTitulo} placeholder="Ej. Audiencia Inicial" required />
    
    <div style="display: grid; grid-template-columns: 1fr 1fr; gap: var(--sp-4);">
      <TextInput label="Fecha" type="date" bind:value={formFecha} required />
      <TextInput label="Hora (Opcional)" type="time" bind:value={formHora} />
    </div>

    <TextInput label="Descripción / Objetivo (Opcional)" bind:value={formDescripcion} multiline rows={3} placeholder="Detalles de preparación, enlace a sala virtual, etc." />

    <div class="form-actions" style="margin-top: var(--sp-4); display: flex; justify-content: flex-end; gap: var(--sp-3);">
      <Button variant="ghost" onclick={() => showForm = false}>Cancelar</Button>
      <Button variant="primary" type="submit">Guardar Evento</Button>
    </div>
  </form>
</Modal>

<style>
  .agenda-container {
    display: flex; flex-direction: column; gap: var(--sp-6);
  }
  .header-actions {
    display: flex; align-items: center; justify-content: space-between;
  }
  .header-actions h3 {
    margin: 0; color: var(--text-primary); font-size: 1.125rem; font-weight: 600;
  }
  .loading { display: flex; justify-content: center; padding: var(--sp-10); }
  .spinner { width: 24px; height: 24px; border: 2px solid var(--border-default); border-top-color: var(--accent-blue); border-radius: 50%; animation: spin 0.8s linear infinite; }
  .empty-state { text-align: center; color: var(--text-secondary); padding: var(--sp-10); border: 1px dashed var(--border-default); border-radius: var(--radius-lg); font-size: 0.875rem; }

  .agenda-list {
    display: flex; flex-direction: column; gap: var(--sp-4);
  }
  .agenda-item {
    display: flex; gap: var(--sp-4); background: var(--bg-surface);
    border: 1px solid var(--border-subtle); border-radius: var(--radius-md);
    padding: var(--sp-4); transition: all var(--ease-base);
  }
  .agenda-item:hover { border-color: var(--border-focus); }
  .agenda-item.cumplido { opacity: 0.6; }
  .agenda-item.cumplido .item-header h4 { text-decoration: line-through; color: var(--text-muted); }

  .item-date {
    display: flex; flex-direction: column; align-items: center; justify-content: center;
    min-width: 60px; padding-right: var(--sp-4); border-right: 1px solid var(--border-default);
  }
  .item-date .day { font-size: 1.5rem; font-weight: 700; color: var(--accent-blue); line-height: 1; margin-bottom: 2px; }
  .agenda-item.cumplido .item-date .day { color: var(--text-muted); }
  .item-date .month { font-size: 0.75rem; font-weight: 600; color: var(--text-secondary); letter-spacing: 0.05em; }

  .item-content { flex: 1; display: flex; flex-direction: column; justify-content: center; gap: var(--sp-1); }
  .item-header { display: flex; align-items: center; gap: var(--sp-3); }
  .item-header h4 { margin: 0; font-size: 1rem; color: var(--text-primary); font-weight: 600; }
  .item-details { display: flex; flex-direction: column; gap: var(--sp-1); margin-top: 4px; }
  
  .time-badge {
    display: inline-flex; align-items: center; gap: 6px; font-size: 0.8125rem; font-weight: 500;
    color: var(--text-primary); background: var(--bg-elevated); padding: 4px 8px; border-radius: var(--radius-sm);
    width: fit-content;
  }
  .desc { margin: 0; font-size: 0.875rem; color: var(--text-secondary); line-height: 1.4; margin-top: 4px; }

  .item-actions {
    display: flex; flex-direction: column; gap: var(--sp-2); justify-content: center;
  }
  .icon-btn { background: none; border: none; cursor: pointer; padding: 6px; border-radius: var(--radius-sm); transition: all var(--ease-fast); color: var(--text-muted); display: flex; align-items: center; justify-content: center; }
  .check-btn:hover { background: var(--accent-green-soft); color: var(--accent-green); }
  .delete-btn:hover { background: var(--accent-red-soft); color: var(--accent-red); }
</style>
