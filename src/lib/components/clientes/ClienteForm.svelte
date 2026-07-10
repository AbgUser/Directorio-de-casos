<script>
  import Modal from '$lib/components/ui/Modal.svelte';
  import TextInput from '$lib/components/ui/TextInput.svelte';
  import Select from '$lib/components/ui/Select.svelte';
  import Button from '$lib/components/ui/Button.svelte';
  import * as clienteService from '$lib/services/clienteService.js';

  let {
    open = false,
    cliente = null,
    onclose = () => {},
    onsave = () => {}
  } = $props();

  const isEdit = $derived(!!cliente);
  const title = $derived(isEdit ? 'Editar Cliente' : 'Nuevo Cliente');

  let tipo_persona = $state('natural');
  let nombre_completo = $state('');
  let tipo_identificacion = $state('CC');
  let identificacion = $state('');
  let telefono = $state('');
  let email = $state('');
  let direccion = $state('');
  let notas = $state('');
  let error = $state('');
  let saving = $state(false);

  const tipoIdOptions = [
    {value: 'CC', label: 'Cédula de Ciudadanía (CC)'},
    {value: 'NIT', label: 'NIT'},
    {value: 'CE', label: 'Cédula de Extranjería (CE)'},
    {value: 'Pasaporte', label: 'Pasaporte'},
    {value: 'Otro', label: 'Otro'}
  ];

  $effect(() => {
    if (open && cliente) {
      tipo_persona = cliente.tipo_persona || 'natural';
      nombre_completo = cliente.nombre_completo || '';
      tipo_identificacion = cliente.tipo_identificacion || 'CC';
      identificacion = cliente.identificacion || '';
      telefono = cliente.telefono || '';
      email = cliente.email || '';
      direccion = cliente.direccion || '';
      notas = cliente.notas || '';
    } else if (open && !cliente) {
      resetForm();
    }
  });

  function resetForm() {
    tipo_persona = 'natural';
    nombre_completo = ''; 
    tipo_identificacion = 'CC'; 
    identificacion = '';
    telefono = ''; 
    email = ''; 
    direccion = ''; 
    notas = '';
    error = '';
  }

  async function handleSubmit(e) {
    e.preventDefault();
    error = '';

    if (!nombre_completo.trim() || !identificacion.trim()) {
      error = 'El nombre/razón social y el número de identificación son obligatorios.';
      return;
    }

    saving = true;
    try {
      const data = {
        tipo_persona, 
        nombre_completo, 
        tipo_identificacion, 
        identificacion,
        telefono, 
        email, 
        direccion, 
        notas
      };

      if (isEdit) {
        await clienteService.update(cliente.id, data);
      } else {
        await clienteService.create(data);
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
      <div class="form-section">
        <Select
          label="Tipo de persona"
          id="tipo_persona"
          bind:value={tipo_persona}
          options={[{value:'natural',label:'Persona Natural'},{value:'juridica',label:'Persona Jurídica'}]}
        />
      </div>

      <TextInput 
        label={tipo_persona === 'natural' ? "Nombre Completo" : "Razón Social"} 
        id="nombre_completo" 
        bind:value={nombre_completo} 
        placeholder={tipo_persona === 'natural' ? "Nombres y Apellidos" : "Nombre de la empresa"} 
        required 
      />

      <div class="form-row">
        <Select 
          label="Tipo de Identificación" 
          id="tipo_identificacion" 
          bind:value={tipo_identificacion} 
          options={tipoIdOptions} 
        />
        <TextInput 
          label="Número de Identificación" 
          id="identificacion" 
          bind:value={identificacion} 
          placeholder="Ej: 123456789" 
          required 
        />
      </div>

      <div class="form-divider"></div>
      <h4 class="form-section-title">Datos de Contacto</h4>

      <div class="form-row">
        <TextInput label="Teléfono / WhatsApp" id="telefono" bind:value={telefono} placeholder="Número de contacto" />
        <TextInput label="Correo Electrónico" id="email" bind:value={email} placeholder="correo@ejemplo.com" />
      </div>
      <TextInput label="Dirección Física" id="direccion" bind:value={direccion} placeholder="Dirección completa" />

      <div class="form-divider"></div>
      <div class="form-field">
        <label class="field-label" for="notas">Notas / Perfil</label>
        <textarea id="notas" class="field-textarea" bind:value={notas} placeholder="Observaciones sobre el perfil del cliente..." rows="3"></textarea>
      </div>

      {#if error}
        <div class="error-msg">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><line x1="15" y1="9" x2="9" y2="15"/><line x1="9" y1="9" x2="15" y2="15"/></svg>
          {error}
        </div>
      {/if}

      <div class="modal-actions">
        <Button variant="ghost" type="button" onclick={onclose}>Cancelar</Button>
        <Button variant="primary" type="submit" disabled={saving}>
          {saving ? 'Guardando...' : (isEdit ? 'Guardar Cambios' : 'Crear Cliente')}
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
    width: 100%; max-width: 580px; max-height: 85vh;
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
  .form-row { display: grid; grid-template-columns: 1fr 1fr; gap: var(--sp-4); }
  .form-divider { height: 1px; background: var(--border-subtle); margin: var(--sp-2) 0; }
  .form-section-title { font-size: 0.8125rem; font-weight: 600; color: var(--text-secondary); text-transform: uppercase; letter-spacing: 0.05em; }
  .form-field { display: flex; flex-direction: column; gap: var(--sp-1); }
  .field-label { font-size: 0.8125rem; font-weight: 500; color: var(--text-secondary); }
  .field-textarea {
    width: 100%; padding: var(--sp-3); background: var(--bg-input);
    border: 1px solid var(--border-default); border-radius: var(--radius-md);
    color: var(--text-primary); font-family: var(--font-sans); font-size: 0.875rem;
    resize: vertical; outline: none; transition: border-color var(--ease-fast);
  }
  .field-textarea:focus { border-color: var(--accent-blue); box-shadow: var(--shadow-glow-blue); }
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
