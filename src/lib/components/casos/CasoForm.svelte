<script>
  import Modal from '$lib/components/ui/Modal.svelte';
  import TextInput from '$lib/components/ui/TextInput.svelte';
  import Select from '$lib/components/ui/Select.svelte';
  import SearchableSelect from '$lib/components/ui/SearchableSelect.svelte';
  import Button from '$lib/components/ui/Button.svelte';
  import * as casoService from '$lib/services/casoService.js';

  let {
    open = false,
    caso = null,
    clienteId = null,
    clientes = [],
    onclose = () => {},
    onsave = () => {}
  } = $props();

  const isEdit = $derived(!!caso);
  const title = $derived(isEdit ? 'Editar Caso' : 'Nuevo Caso');

  let cliente_id = $state('');
  let categoria_expediente = $state('proceso');
  let radicado = $state('');
  let tipo_proceso = $state('');
  let juzgado = $state('');
  let contraparte = $state('');
  let estado = $state('Activo');
  let descripcion = $state('');
  let fecha_inicio = $state('');
  let error = $state('');
  let saving = $state(false);

  const estadoOptions = [
    {value:'Activo',label:'Activo'},
    {value:'Suspendido',label:'Suspendido'},
    {value:'Terminado',label:'Terminado'},
    {value:'Archivado',label:'Archivado'}
  ];

  function displayName(c) {
    return c.nombre_completo || '';
  }

  let clienteOptions = $derived(
    clientes.map(c => ({ value: String(c.id), label: displayName(c) }))
  );

  $effect(() => {
    if (open && caso) {
      cliente_id = String(caso.cliente_id || '');
      categoria_expediente = caso.categoria_expediente || 'proceso';
      radicado = caso.radicado || '';
      tipo_proceso = caso.tipo_proceso || '';
      juzgado = caso.juzgado || '';
      contraparte = caso.contraparte || '';
      estado = caso.estado || 'Activo';
      descripcion = caso.descripcion || '';
      fecha_inicio = caso.fecha_inicio || '';
    } else if (open && !caso) {
      resetForm();
      if (clienteId) cliente_id = String(clienteId);
    }
  });

  function resetForm() {
    cliente_id = clienteId ? String(clienteId) : '';
    categoria_expediente = 'proceso';
    radicado = ''; 
    tipo_proceso = ''; 
    juzgado = ''; 
    contraparte = ''; 
    estado = 'Activo'; 
    descripcion = '';
    fecha_inicio = new Date().toISOString().split('T')[0];
    error = '';
  }

  async function handleSubmit(e) {
    e.preventDefault();
    error = '';

    if (!cliente_id) { error = 'Debe seleccionar un cliente asociado al caso.'; return; }
    
    if (categoria_expediente === 'proceso') {
      if (!radicado || radicado.length < 5) { error = 'El número de radicado es inválido o muy corto.'; return; }
      if (!tipo_proceso) { error = 'El tipo de proceso es obligatorio.'; return; }
    } else {
      if (!tipo_proceso) { error = 'El asunto o tema es obligatorio.'; return; }
    }

    saving = true;
    try {
      const data = {
        cliente_id: Number(cliente_id), 
        categoria_expediente,
        radicado: categoria_expediente === 'proceso' ? radicado : null, 
        tipo_proceso, 
        juzgado: categoria_expediente === 'proceso' ? juzgado : null, 
        contraparte: categoria_expediente === 'proceso' ? contraparte : null, 
        estado, 
        descripcion, 
        fecha_inicio
      };
      if (isEdit) {
        await casoService.update(caso.id, data);
      } else {
        await casoService.create(data);
      }
      onsave();
      onclose();
    } catch (err) {
      error = String(err.message || err);
    } finally {
      saving = false;
    }
  }
</script>

{#if open}
<div class="modal-backdrop" onclick={onclose}>
  <div class="modal-content" onclick={(e) => e.stopPropagation()}>
    <div class="modal-header">
      <h2>{title}</h2>
      <button class="close-btn" onclick={onclose} aria-label="Cerrar">
        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/></svg>
      </button>
    </div>

    <form class="modal-body" onsubmit={handleSubmit}>
      
      <!-- TABS SELECTOR -->
      <div class="tabs-container">
        <button type="button" class="tab-btn {categoria_expediente === 'proceso' ? 'active' : ''}" onclick={() => categoria_expediente = 'proceso'}>Proceso Judicial</button>
        <button type="button" class="tab-btn {categoria_expediente === 'consulta' ? 'active' : ''}" onclick={() => categoria_expediente = 'consulta'}>Consulta Jurídica</button>
      </div>

      <SearchableSelect 
        label="Cliente Asociado" 
        id="cliente_caso" 
        bind:value={cliente_id} 
        options={clienteOptions} 
        placeholder="Buscar y seleccionar cliente..." 
        required 
      />

      {#if categoria_expediente === 'proceso'}
        <TextInput 
          label="Número de Radicado (21 dígitos)" 
          id="radicado" 
          bind:value={radicado} 
          placeholder="Ej: 11001310500120230012300" 
          required
        />

        <div class="form-row">
          <TextInput 
            label="Tipo de Proceso" 
            id="tipo_proceso" 
            bind:value={tipo_proceso} 
            placeholder="Ej: Ordinario Laboral, Ejecutivo..." 
            required
          />
          <TextInput 
            label="Nombre de la Contraparte" 
            id="contraparte" 
            bind:value={contraparte} 
            placeholder="Nombre o Razón Social" 
          />
        </div>

        <div class="form-divider"></div>
        <h4 class="section-title">Despacho y Estado</h4>
        
        <div class="form-row">
          <TextInput 
            label="Juzgado / Despacho de Conocimiento" 
            id="juzgado" 
            bind:value={juzgado} 
            placeholder="Ej: Juzgado 1 Civil del Circuito" 
          />
          <Select 
            label="Estado Actual del Proceso" 
            id="estado" 
            bind:value={estado} 
            options={estadoOptions} 
          />
        </div>

        <div class="form-row">
          <div class="form-field">
            <label class="field-label" for="fecha_inicio">Fecha de Radicación / Inicio</label>
            <input type="date" id="fecha_inicio" class="field-date" bind:value={fecha_inicio} />
          </div>
        </div>

        <div class="form-field">
          <label class="field-label" for="desc_caso">Descripción o Notas</label>
          <textarea id="desc_caso" class="field-textarea" bind:value={descripcion} placeholder="Breve descripción del objeto del proceso..." rows="2"></textarea>
        </div>

      {:else}
        <!-- CAMPOS PARA CONSULTA JURÍDICA -->
        <TextInput 
          label="Asunto / Tema de Consulta" 
          id="tipo_proceso_consulta" 
          bind:value={tipo_proceso} 
          placeholder="Ej: Asesoría Contrato Laboral" 
          required
        />
        
        <div class="form-row">
          <div class="form-field">
            <label class="field-label" for="fecha_inicio_consulta">Fecha de Consulta</label>
            <input type="date" id="fecha_inicio_consulta" class="field-date" bind:value={fecha_inicio} required />
          </div>
          <Select 
            label="Estado de la Consulta" 
            id="estado_consulta" 
            bind:value={estado} 
            options={estadoOptions} 
          />
        </div>

        <div class="form-field">
          <label class="field-label" for="desc_caso_consulta">Descripción / Resumen de la Asesoría</label>
          <textarea id="desc_caso_consulta" class="field-textarea" bind:value={descripcion} placeholder="Detalles principales de la consulta..." rows="3" required></textarea>
        </div>
      {/if}

      {#if error}
        <div class="error-msg">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><line x1="15" y1="9" x2="9" y2="15"/><line x1="9" y1="9" x2="15" y2="15"/></svg>
          {error}
        </div>
      {/if}

      <div class="modal-actions">
        <Button variant="ghost" type="button" onclick={onclose}>Cancelar</Button>
        <Button variant="primary" type="submit" disabled={saving}>
          {saving ? 'Guardando...' : (isEdit ? 'Guardar Cambios' : 'Crear Caso')}
        </Button>
      </div>
    </form>
  </div>
</div>
{/if}

<style>
  .modal-backdrop {
    position: fixed; inset: 0; z-index: 1000;
    background: rgba(0,0,0,0.6); backdrop-filter: blur(4px);
    display: flex; align-items: center; justify-content: center;
    animation: fadeIn 200ms ease;
  }
  .modal-content {
    width: 100%; max-width: 620px; max-height: 85vh;
    background: var(--bg-secondary); border: 1px solid var(--border-default);
    border-radius: var(--radius-xl); box-shadow: var(--shadow-lg);
    display: flex; flex-direction: column; overflow: hidden;
  }
  .modal-header {
    display: flex; align-items: center; justify-content: space-between;
    padding: var(--sp-5) var(--sp-6); border-bottom: 1px solid var(--border-subtle);
  }
  .modal-header h2 { font-size: 1.125rem; font-weight: 600; }
  .close-btn {
    background: none; border: none; color: var(--text-muted);
    cursor: pointer; padding: var(--sp-1); border-radius: var(--radius-sm);
    display: flex; align-items: center; transition: color var(--ease-fast);
  }
  .close-btn:hover { color: var(--text-primary); }
  .modal-body {
    padding: var(--sp-5) var(--sp-6); overflow-y: auto;
    display: flex; flex-direction: column; gap: var(--sp-4);
  }
  .tabs-container {
    display: flex; gap: var(--sp-2); margin-bottom: var(--sp-2);
    border-bottom: 1px solid var(--border-subtle); padding-bottom: var(--sp-3);
  }
  .tab-btn {
    flex: 1; background: none; border: 1px solid var(--border-default); 
    padding: var(--sp-2) var(--sp-4); border-radius: var(--radius-md);
    font-size: 0.875rem; font-weight: 500; color: var(--text-muted);
    cursor: pointer; transition: all var(--ease-fast);
  }
  .tab-btn:hover { color: var(--text-primary); border-color: var(--border-focus); }
  .tab-btn.active { 
    color: var(--accent-blue); background: var(--accent-blue-soft); 
    border-color: var(--accent-blue); font-weight: 600;
  }
  .form-row { display: grid; grid-template-columns: 1fr 1fr; gap: var(--sp-4); }
  .form-divider { height: 1px; background: var(--border-subtle); margin: var(--sp-1) 0; }
  .section-title { font-size: 0.8125rem; font-weight: 600; color: var(--text-secondary); text-transform: uppercase; letter-spacing: 0.05em; }
  .form-field { display: flex; flex-direction: column; gap: var(--sp-1); }
  .field-label { font-size: 0.8125rem; font-weight: 500; color: var(--text-secondary); }
  .field-date, .field-textarea {
    width: 100%; padding: var(--sp-3); background: var(--bg-input);
    border: 1px solid var(--border-default); border-radius: var(--radius-md);
    color: var(--text-primary); font-family: var(--font-sans); font-size: 0.875rem;
    outline: none; transition: border-color var(--ease-fast);
  }
  .field-date:focus, .field-textarea:focus { border-color: var(--accent-blue); box-shadow: var(--shadow-glow-blue); }
  .field-textarea { resize: vertical; }
  .field-textarea::placeholder { color: var(--text-muted); }
  .error-msg {
    display: flex; align-items: center; gap: var(--sp-2);
    padding: var(--sp-2) var(--sp-3); background: var(--accent-red-soft);
    border-radius: var(--radius-md); color: var(--accent-red); font-size: 0.8125rem;
  }
  .modal-actions {
    display: flex; justify-content: flex-end; gap: var(--sp-3);
    padding-top: var(--sp-4); border-top: 1px solid var(--border-subtle); margin-top: var(--sp-2);
  }
  @keyframes fadeIn { from { opacity: 0; } to { opacity: 1; } }
</style>
