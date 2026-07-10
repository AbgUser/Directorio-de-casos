<script>
  import Badge from '$lib/components/ui/Badge.svelte';
  import Button from '$lib/components/ui/Button.svelte';
  import TextInput from '$lib/components/ui/TextInput.svelte';
  import Select from '$lib/components/ui/Select.svelte';
  import ConfirmDialog from '$lib/components/ui/ConfirmDialog.svelte';
  import EmptyState from '$lib/components/ui/EmptyState.svelte';
  import * as actuacionService from '$lib/services/actuacionService.js';
  import * as calendarioService from '$lib/services/calendarioService.js';
  import * as modelosService from '$lib/services/modelosService.js';
  import * as documentoService from '$lib/services/documentoService.js';
  import * as casoService from '$lib/services/casoService.js';
  import { formatDate, getTipoActuacionLabel } from '$lib/utils/formatters.js';
  import { listen } from '@tauri-apps/api/event';
  import { invoke } from '@tauri-apps/api/core';
  import { documentDir } from '@tauri-apps/api/path';

  let { casoId } = $props();

  let actuaciones = $state([]);
  let loading = $state(true);
  let showForm = $state(false);
  let editingActuacion = $state(null);
  let deleteTarget = $state(null);

  // Modelos state
  let modelos = $state([]);
  let selectedModeloId = $state('');
  let cloning = $state(false);

  // Archivos vinculados state
  let archivosTemporales = $state([]);
  let isDraggingOverForm = $state(false);

  // Form fields
  let fecha = $state('');
  let tipo = $state('anotacion');
  let descripcion = $state('');
  let fecha_notificacion = $state('');
  
  // Termino fields
  let genera_termino = $state(false);
  let termino_vencimiento = $state('');

  const tipoOptions = [
    {value:'auto',label:'Auto'},{value:'sentencia',label:'Sentencia'},
    {value:'notificacion',label:'Notificación'},{value:'audiencia',label:'Audiencia'},
    {value:'memorial',label:'Memorial'},{value:'anotacion',label:'Anotación'},
    {value:'otro',label:'Otro'}
  ];

  const tipoDotColor = {
    auto: 'var(--accent-blue)', sentencia: 'var(--accent-purple)',
    notificacion: 'var(--accent-amber)', audiencia: 'var(--accent-teal)',
    memorial: 'var(--accent-green)', anotacion: 'var(--text-secondary)',
    otro: 'var(--text-muted)'
  };

  $effect(() => { if (casoId) loadActuaciones(); });

  $effect(() => {
    let unlistenEnter, unlistenLeave, unlistenDrop;

    async function setupDragAndDrop() {
      unlistenEnter = await listen('tauri://drag-enter', () => {
        if (showForm) isDraggingOverForm = true;
      });
      unlistenLeave = await listen('tauri://drag-leave', () => {
        isDraggingOverForm = false;
      });
      unlistenDrop = await listen('tauri://drag-drop', (event) => {
        isDraggingOverForm = false;
        if (showForm) {
           const filePaths = event.payload?.paths || event.payload;
           if (Array.isArray(filePaths) && filePaths.length > 0) {
             archivosTemporales = [...archivosTemporales, ...filePaths.map(p => ({
               nombre: p.split('/').pop() || p.split('\\').pop(),
               ruta_absoluta: p
             }))];
           }
        }
      });
    }

    setupDragAndDrop();

    return () => {
      if (unlistenEnter) unlistenEnter();
      if (unlistenLeave) unlistenLeave();
      if (unlistenDrop) unlistenDrop();
    };
  });

  async function loadActuaciones() {
    loading = true;
    try { 
      actuaciones = await actuacionService.getByCaso(casoId); 
      modelos = await modelosService.getAll();
    }
    catch (e) { console.error(e); }
    finally { loading = false; }
  }

  function openNewForm() {
    editingActuacion = null;
    fecha = new Date().toISOString().split('T')[0];
    tipo = 'anotacion'; descripcion = ''; fecha_notificacion = '';
    genera_termino = false; termino_vencimiento = '';
    selectedModeloId = '';
    archivosTemporales = [];
    showForm = true;
  }

  function openEditForm(act) {
    editingActuacion = act;
    fecha = act.fecha || ''; tipo = act.tipo || 'anotacion';
    descripcion = act.descripcion || ''; fecha_notificacion = act.fecha_notificacion || '';
    genera_termino = !!act.genera_termino;
    termino_vencimiento = act.termino_vencimiento || '';
    archivosTemporales = act.archivos_vinculados && act.archivos_vinculados !== '[]' 
      ? JSON.parse(act.archivos_vinculados).map(n => ({ nombre: n, ruta_absoluta: null }))
      : [];
    showForm = true;
  }

  async function handleSave(e) {
    e.preventDefault();
    if (!descripcion.trim()) return;
    try {
      // 1. Copiar archivos arrastrados al expediente si hay nuevos
      let nombresFinales = [];
      const archivosNuevos = archivosTemporales.filter(a => a.ruta_absoluta);
      
      if (archivosNuevos.length > 0) {
        const caso = await casoService.getById(casoId);
        const basePath = await documentDir();
        const destFolder = `${basePath}/Directorio_Casos/${caso.carpeta_documentos}`;
        const rutasNuevas = archivosNuevos.map(a => a.ruta_absoluta);
        
        const nombresCopiados = await invoke('copiar_archivos_actuacion', { 
           rutas: rutasNuevas, 
           carpetaDestino: destFolder 
        });
        
        nombresFinales = archivosTemporales
          .filter(a => !a.ruta_absoluta)
          .map(a => a.nombre)
          .concat(nombresCopiados);
      } else {
        nombresFinales = archivosTemporales.map(a => a.nombre);
      }

      const data = { 
        caso_id: casoId, fecha, tipo, descripcion, 
        fecha_notificacion: fecha_notificacion || null,
        genera_termino: genera_termino ? 1 : 0,
        termino_vencimiento: genera_termino ? termino_vencimiento : null,
        dias_termino: null,
        tipo_dias: null,
        archivos_vinculados: JSON.stringify(nombresFinales)
      };
      
      // Guardar actuación
      let actuacionId;
      if (editingActuacion) {
        await actuacionService.update(editingActuacion.id, data);
        actuacionId = editingActuacion.id;
      } else {
        actuacionId = await actuacionService.create(data);
      }



      showForm = false;
      await loadActuaciones();
    } catch (e) { console.error(e); }
  }

  async function handleClonarModelo() {
    if (!selectedModeloId) return;
    const modelo = modelos.find(m => m.id == selectedModeloId);
    if (!modelo) return;
    
    cloning = true;
    try {
      const tipoLabel = tipoOptions.find(t => t.value === tipo)?.label || tipo;
      
      // Pre-calcular el nombre que Svelte/Rust generará
      const today = new Date().toISOString().split('T')[0];
      const extension = modelo.ruta_archivo.split('.').pop();
      const nombreLimpio = tipoLabel.replace(/[^a-zA-Z0-9]/g, '_');
      const nuevoNombre = `${today}_${nombreLimpio}.${extension}`;

      await modelosService.clonarYabrir(modelo, casoId, tipoLabel);
      
      // Vincular el modelo a la lista de archivos pendientes de la actuación
      archivosTemporales = [...archivosTemporales, { nombre: nuevoNombre, ruta_absoluta: null }];
      
      alert('Modelo generado y vinculado exitosamente a la actuación.');
    } catch (e) {
      console.error(e);
      alert('Error clonando modelo: ' + String(e));
    } finally {
      cloning = false;
      selectedModeloId = ''; // Reset selection
    }
  }

  async function handleDelete() {
    if (!deleteTarget) return;
    try {
      await actuacionService.remove(deleteTarget.id);
      deleteTarget = null;
      await loadActuaciones();
    } catch (e) { console.error(e); }
  }

  async function openLinkedFile(nombre) {
    try {
      const caso = await casoService.getById(casoId);
      const basePath = await documentDir();
      const rutaAbsoluta = `${basePath}/Directorio_Casos/${caso.carpeta_documentos}/${nombre}`;
      await documentoService.openFile(rutaAbsoluta);
    } catch (e) {
      console.error(e);
      alert('No se pudo abrir el archivo. ¿Fue eliminado del directorio?');
    }
  }

  async function solventarTermino(e, act) {
    e.stopPropagation();
    try {
      await invoke('marcar_termino_completado', { id: act.id });
      act.termino_completado = 1;
    } catch (err) {
      console.error(err);
      alert('Error al completar el término.');
    }
  }
</script>

<div class="timeline-container">
  <div class="timeline-header">
    <h3>Actuaciones Procesales</h3>
    <Button variant="primary" size="sm" onclick={openNewForm}>
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/></svg>
      Nueva Actuación
    </Button>
  </div>

  {#if loading}
    <div class="loading-state"><div class="spinner"></div></div>
  {:else if actuaciones.length === 0}
    <EmptyState title="Sin actuaciones" message="Registre la primera actuación procesal de este caso." />
  {:else}
    <div class="timeline">
      {#each actuaciones as act}
        <div class="timeline-item">
          <div class="timeline-dot" style="background: {tipoDotColor[act.tipo] || 'var(--text-muted)'}"></div>
          <div class="timeline-line"></div>
          <div class="timeline-card" onclick={() => openEditForm(act)}>
            <div class="card-header">
              <span class="card-date">{formatDate(act.fecha)}</span>
              <Badge variant={act.tipo === 'sentencia' ? 'purple' : act.tipo === 'audiencia' ? 'teal' : act.tipo === 'notificacion' ? 'amber' : 'default'}>
                {getTipoActuacionLabel(act.tipo)}
              </Badge>
              <div style="margin-left: auto; display: flex; gap: var(--sp-2);">
                {#if act.termino_vencimiento && !act.termino_completado}
                  <button class="solventar-btn" title="Solventar Término" onclick={(e) => solventarTermino(e, act)}>
                    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="20 6 9 17 4 12"/></svg>
                  </button>
                {/if}
                <button class="delete-btn" onclick={(e) => { e.stopPropagation(); deleteTarget = act; }} aria-label="Eliminar">
                  <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="3 6 5 6 21 6"/><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/></svg>
                </button>
              </div>
            </div>
            <p class="card-desc">{act.descripcion}</p>
            {#if act.archivos_vinculados && act.archivos_vinculados !== '[]'}
              <div class="card-files">
                {#each JSON.parse(act.archivos_vinculados) as fName}
                  <button class="file-badge" type="button" onclick={(e) => { e.stopPropagation(); openLinkedFile(fName); }}>
                    <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M21.44 11.05l-9.19 9.19a6 6 0 0 1-8.49-8.49l9.19-9.19a4 4 0 0 1 5.66 5.66l-9.2 9.19a2 2 0 0 1-2.83-2.83l8.49-8.48"/></svg>
                    {fName}
                  </button>
                {/each}
              </div>
            {/if}
            {#if act.fecha_notificacion}
              <span class="card-notif">Notificado: {formatDate(act.fecha_notificacion)}</span>
            {/if}
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>

<!-- Form Modal -->
{#if showForm}
<div class="modal-backdrop" onclick={() => showForm = false}>
  <div class="modal-content" onclick={(e) => e.stopPropagation()}>
    <div class="modal-header">
      <h2>{editingActuacion ? 'Editar Actuación' : 'Nueva Actuación'}</h2>
      <button class="close-btn" onclick={() => showForm = false} aria-label="Cerrar">
        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/></svg>
      </button>
    </div>
    <form class="modal-body" onsubmit={handleSave}>
      <div class="form-row">
        <div class="form-field">
          <label class="field-label" for="act_fecha">Fecha</label>
          <input type="date" id="act_fecha" class="field-date" bind:value={fecha} required />
        </div>
        <Select label="Tipo" id="act_tipo" bind:value={tipo} options={tipoOptions} />
      </div>
      <div class="form-field">
        <label class="field-label" for="act_desc">Descripción</label>
        <textarea id="act_desc" class="field-textarea" bind:value={descripcion} placeholder="Detalle de la actuación procesal..." rows="3" required></textarea>
      </div>

      {#if !editingActuacion}
      <div class="form-divider" style="margin: var(--sp-1) 0; border-top: 1px dashed var(--border-subtle)"></div>
      <div class="form-field">
        <label class="field-label">Ensamblar Documento (Modelo Base)</label>
        <div style="display: flex; gap: var(--sp-2);">
          <select class="field-date" bind:value={selectedModeloId}>
            <option value="">-- No usar plantilla --</option>
            {#each modelos as mod}
              <option value={mod.id}>{mod.nombre} ({mod.tipo})</option>
            {/each}
          </select>
          {#if selectedModeloId}
            <Button variant="secondary" type="button" onclick={handleClonarModelo} disabled={cloning}>
              {cloning ? 'Clonando...' : 'Generar y Abrir'}
            </Button>
          {/if}
        </div>
      </div>
      {/if}

      <div class="form-divider" style="margin: var(--sp-1) 0; border-top: 1px dashed var(--border-subtle)"></div>
      <div class="form-field">
        <label class="field-label">Documentos Vinculados</label>
        <div class="drop-zone {isDraggingOverForm ? 'drag-over' : ''}">
          <p>Arrastra archivos PDF o Word aquí para vincularlos a la actuación</p>
        </div>
        {#if archivosTemporales.length > 0}
          <div class="temp-files-list">
            {#each archivosTemporales as arc, i}
              <div class="temp-file-item">
                <span class="file-name">{arc.nombre}</span>
                <button type="button" class="remove-file-btn" aria-label="Eliminar archivo" onclick={() => archivosTemporales = archivosTemporales.filter((_, idx) => idx !== i)}>×</button>
              </div>
            {/each}
          </div>
        {/if}
      </div>

      <div class="form-divider" style="margin: var(--sp-1) 0; border-top: 1px dashed var(--border-subtle)"></div>

      <div class="form-field checkbox-field">
        <label class="field-label" style="display:flex; align-items:center; gap:8px; cursor:pointer;">
          <input type="checkbox" bind:checked={genera_termino} />
          Genera término judicial
        </label>
      </div>

      {#if genera_termino}
      <div class="form-row terminos-row">
        <div class="form-field">
          <label class="field-label" for="act_vencimiento">Fecha de Vencimiento <span style="color: var(--accent-red); margin-left: 2px;">*</span></label>
          <input type="date" id="act_vencimiento" class="field-date" bind:value={termino_vencimiento} required />
        </div>
      </div>
      {/if}
      <div class="modal-actions">
        <Button variant="ghost" onclick={() => showForm = false}>Cancelar</Button>
        <Button variant="primary" type="submit">{editingActuacion ? 'Guardar' : 'Registrar'}</Button>
      </div>
    </form>
  </div>
</div>
{/if}

<ConfirmDialog open={!!deleteTarget} title="Eliminar Actuación" message="¿Está seguro de eliminar esta actuación? Esta acción no se puede deshacer." onconfirm={handleDelete} oncancel={() => deleteTarget = null} />

<style>
  .timeline-container { display: flex; flex-direction: column; gap: var(--sp-4); }
  .timeline-header { display: flex; align-items: center; justify-content: space-between; }
  .timeline-header h3 { font-size: 1rem; font-weight: 600; }
  .loading-state { display: flex; justify-content: center; padding: var(--sp-10); }
  .spinner { width: 24px; height: 24px; border: 3px solid var(--border-default); border-top-color: var(--accent-blue); border-radius: 50%; animation: spin 0.8s linear infinite; }

  .timeline { display: flex; flex-direction: column; padding-left: var(--sp-6); }
  .timeline-item { position: relative; padding-bottom: var(--sp-5); padding-left: var(--sp-6); }
  .timeline-dot {
    position: absolute; left: -6px; top: 6px;
    width: 12px; height: 12px; border-radius: 50%;
    border: 2px solid var(--bg-surface); z-index: 1;
  }
  .timeline-line {
    position: absolute; left: -1px; top: 18px; bottom: 0;
    width: 2px; background: var(--border-default);
  }
  .timeline-item:last-child .timeline-line { display: none; }

  .timeline-card {
    background: var(--bg-surface); border: 1px solid var(--border-default);
    border-radius: var(--radius-lg); padding: var(--sp-4);
    cursor: pointer; transition: all var(--ease-fast);
  }
  .timeline-card:hover { border-color: var(--accent-blue); box-shadow: var(--shadow-sm); }
  .card-header { display: flex; align-items: center; gap: var(--sp-3); margin-bottom: var(--sp-2); }
  .card-date { font-size: 0.8125rem; font-weight: 500; color: var(--text-primary); }
  .delete-btn {
    margin-left: auto; background: none; border: none; color: var(--text-muted);
    cursor: pointer; padding: var(--sp-1); border-radius: var(--radius-sm);
    transition: color var(--ease-fast); display: flex;
  }
  .delete-btn:hover { color: var(--accent-red); }
  .card-desc { font-size: 0.875rem; color: var(--text-secondary); line-height: 1.5; }
  .card-notif { display: inline-block; margin-top: var(--sp-2); font-size: 0.75rem; color: var(--text-muted); }

  .card-files { display: flex; flex-wrap: wrap; gap: var(--sp-2); margin-top: var(--sp-3); }
  .file-badge { 
    display: inline-flex; align-items: center; gap: 6px; 
    padding: 4px 10px; background: var(--bg-secondary);
    border: 1px solid var(--border-subtle); border-radius: var(--radius-full);
    font-size: 0.75rem; color: var(--text-secondary); cursor: pointer;
    transition: all var(--ease-fast);
  }
  .file-badge:hover { background: var(--bg-hover); color: var(--accent-blue); border-color: var(--accent-blue); }

  /* Modal styles */
  .modal-backdrop {
    position: fixed; inset: 0; z-index: 1000;
    background: rgba(0,0,0,0.6); backdrop-filter: blur(4px);
    display: flex; align-items: center; justify-content: center;
  }
  .modal-content {
    width: 100%; max-width: 540px; background: var(--bg-secondary);
    border: 1px solid var(--border-default); border-radius: var(--radius-xl);
    box-shadow: var(--shadow-lg); overflow: hidden;
  }
  .modal-header {
    display: flex; align-items: center; justify-content: space-between;
    padding: var(--sp-4) var(--sp-5); border-bottom: 1px solid var(--border-subtle);
  }
  .modal-header h2 { font-size: 1.125rem; font-weight: 600; }
  .close-btn { background: none; border: none; color: var(--text-muted); cursor: pointer; display: flex; padding: var(--sp-1); border-radius: var(--radius-sm); }
  .close-btn:hover { color: var(--text-primary); }
  .modal-body { padding: var(--sp-4) var(--sp-5); display: flex; flex-direction: column; gap: var(--sp-3); max-height: 80vh; overflow-y: auto; }
  .form-row { display: grid; grid-template-columns: 1fr 1fr; gap: var(--sp-4); }
  .form-field { display: flex; flex-direction: column; gap: var(--sp-1); }
  .field-label { font-size: 0.8125rem; font-weight: 500; color: var(--text-secondary); }
  .field-date, .field-textarea {
    width: 100%; padding: var(--sp-2) var(--sp-3); background: var(--bg-input);
    border: 1px solid var(--border-default); border-radius: var(--radius-md);
    color: var(--text-primary); font-family: var(--font-sans); font-size: 0.875rem;
    outline: none; transition: border-color var(--ease-fast);
  }
  .field-date:focus, .field-textarea:focus { border-color: var(--accent-blue); box-shadow: var(--shadow-glow-blue); }
  .field-textarea { resize: vertical; }
  .field-textarea::placeholder { color: var(--text-muted); }
  .modal-actions { display: flex; justify-content: flex-end; gap: var(--sp-3); padding-top: var(--sp-4); border-top: 1px solid var(--border-subtle); }

  .drop-zone {
    border: 2px dashed var(--border-default); border-radius: var(--radius-md);
    padding: var(--sp-3); text-align: center; color: var(--text-muted);
    font-size: 0.8125rem; transition: all var(--ease-fast);
  }
  .drop-zone.drag-over { border-color: var(--accent-blue); background: rgba(59, 130, 246, 0.05); color: var(--accent-blue); }
  
  .temp-files-list { display: flex; flex-direction: column; gap: var(--sp-2); margin-top: var(--sp-2); }
  .temp-file-item {
    display: flex; align-items: center; justify-content: space-between;
    background: var(--bg-surface); padding: 6px 12px;
    border-radius: var(--radius-sm); border: 1px solid var(--border-subtle);
  }
  .file-name { font-size: 0.8125rem; color: var(--text-primary); white-space: nowrap; overflow: hidden; text-overflow: ellipsis; max-width: 90%; }
  .remove-file-btn { background: none; border: none; color: var(--accent-red); cursor: pointer; font-weight: bold; font-size: 1rem; padding: 0 4px; }
  
  .solventar-btn {
    background: var(--bg-secondary); border: 1px solid var(--border-subtle);
    color: var(--text-secondary); width: 24px; height: 24px;
    border-radius: var(--radius-sm); display: flex; align-items: center; justify-content: center;
    cursor: pointer; transition: all var(--ease-fast);
  }
  .solventar-btn:hover {
    background: var(--accent-green-soft); color: var(--accent-green); border-color: var(--accent-green-soft);
  }
</style>
