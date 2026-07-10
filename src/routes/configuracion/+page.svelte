<script>
  import { onMount } from 'svelte';
  import Card from '$lib/components/ui/Card.svelte';
  import Button from '$lib/components/ui/Button.svelte';
  import TextInput from '$lib/components/ui/TextInput.svelte';
  import * as configService from '$lib/services/configService.js';
  import { open as openDialog, save as saveDialog } from '@tauri-apps/plugin-dialog';
  import { invoke } from '@tauri-apps/api/core';
  import ConfirmDialog from '$lib/components/ui/ConfirmDialog.svelte';
  import Badge from '$lib/components/ui/Badge.svelte';
  import Select from '$lib/components/ui/Select.svelte';
  import Modal from '$lib/components/ui/Modal.svelte';
  import * as modelosService from '$lib/services/modelosService.js';

  let config = $state({});
  let loading = $state(true);
  let saving = $state(false);
  let successMsg = $state('');

  // Password change state
  let currentPassword = $state('');
  let newPassword = $state('');
  let confirmNewPassword = $state('');
  let pwdError = $state('');
  let pwdSuccess = $state('');
  let changingPwd = $state(false);

  // Backup state
  let backupMsg = $state('');
  let backupError = $state('');
  let showRestoreConfirm = $state(false);
  let pendingRestorePath = $state('');
  let generatingBackup = $state(false);
  let restoringBackup = $state(false);

  // Factory reset state
  let showFactoryResetModal = $state(false);
  let factoryResetPassword = $state('');
  let factoryResetError = $state('');
  let isFactoryResetting = $state(false);

  // Modelos state
  let modelos = $state([]);
  let newModeloNombre = $state('');
  let newModeloTipo = $state('Demanda');
  let modeloMsg = $state('');
  let loadingModelos = $state(false);

  const modeloTipos = [
    { value: 'Demanda', label: 'Demanda' },
    { value: 'Memorial', label: 'Memorial' },
    { value: 'Poder', label: 'Poder' },
    { value: 'Contrato', label: 'Contrato' },
    { value: 'Acta', label: 'Acta' },
    { value: 'Concepto', label: 'Concepto' },
    { value: 'Otro', label: 'Otro' }
  ];



  import { getAuth } from '$lib/stores/auth.svelte.js';
  const auth = getAuth();

  onMount(async () => {
    loading = true;
    try {
      config = await configService.getConfig();
    } catch (err) {
      console.error("Error cargando config:", err);
    }

    await loadModelos();

    loading = false;
  });

  async function loadModelos() {
    loadingModelos = true;
    try {
      modelos = await modelosService.getAll();
    } catch (e) {
      console.error(e);
    } finally {
      loadingModelos = false;
    }
  }

  async function handleSave(e) {
    e.preventDefault();
    saving = true;
    successMsg = '';
    try {
      await configService.updateConfig(config);
      successMsg = 'Configuración guardada exitosamente.';
      setTimeout(() => successMsg = '', 3000);
    } catch (e) {
      console.error('Error guardando config:', e);
    } finally {
      saving = false;
    }
  }



  async function handleChangePassword(e) {
    e.preventDefault();
    pwdError = ''; pwdSuccess = '';
    
    if (newPassword.length < 4) {
      pwdError = 'La nueva contraseña debe tener al menos 4 caracteres.'; return;
    }
    if (newPassword !== confirmNewPassword) {
      pwdError = 'Las nuevas contraseñas no coinciden.'; return;
    }

    changingPwd = true;
    try {
      const res = await auth.changePassword(newPassword);
      if (res.success) {
        pwdSuccess = 'Contraseña actualizada correctamente.';
        currentPassword = ''; newPassword = ''; confirmNewPassword = '';
        setTimeout(() => pwdSuccess = '', 4000);
      } else {
        pwdError = res.error || 'Error cambiando la contraseña.';
      }
    } catch (err) {
      pwdError = String(err);
    } finally {
      changingPwd = false;
    }
  }

  async function handleExportBackup() {
    backupMsg = ''; backupError = '';
    try {
      const destPath = await saveDialog({
        title: 'Guardar Copia de Seguridad (Solo Base de Datos)',
        defaultPath: 'directorio_casos_bd_backup.db',
        filters: [{ name: 'SQLite DB', extensions: ['db'] }]
      });
      if (!destPath) return;

      await invoke('db_export_backup', { destPath });
      backupMsg = 'Copia de seguridad de la base de datos exportada correctamente.';
      setTimeout(() => backupMsg = '', 5000);
    } catch (e) {
      backupError = String(e);
      setTimeout(() => backupError = '', 8000);
    }
  }

  async function handleFullBackup() {
    backupMsg = ''; backupError = '';
    
    // Obtener la fecha actual YYYYMMDD
    const dateStr = new Date().toISOString().slice(0,10).replace(/-/g,"");
    
    try {
      const destPath = await saveDialog({
        title: 'Guardar Copia de Seguridad Total',
        defaultPath: `Backup_Directorio_Casos_${dateStr}.zip`,
        filters: [{ name: 'Archivo ZIP', extensions: ['zip'] }]
      });
      if (!destPath) return;

      generatingBackup = true;
      await invoke('generar_backup_completo', { destPath });
      backupMsg = 'Copia de seguridad total (DB + Expedientes) exportada correctamente.';
      setTimeout(() => backupMsg = '', 5000);
    } catch (e) {
      backupError = String(e);
      setTimeout(() => backupError = '', 8000);
    } finally {
      generatingBackup = false;
    }
  }

  async function handleUploadModelo(e) {
    e.preventDefault();
    modeloMsg = '';
    if (!newModeloNombre.trim()) {
      modeloMsg = 'Debes escribir un nombre para el modelo.'; return;
    }
    
    try {
      const selected = await openDialog({
        multiple: false,
        title: 'Seleccionar Archivo de Modelo',
        filters: [{ name: 'Documentos', extensions: ['docx', 'doc', 'pages', 'pdf'] }]
      });
      if (!selected) return;
      
      const absolutePath = typeof selected === 'string' ? selected : selected.path;
      
      await modelosService.create({
        nombre: newModeloNombre,
        tipo: newModeloTipo,
        ruta_archivo: absolutePath
      });
      
      modeloMsg = 'Modelo guardado exitosamente en la bóveda.';
      newModeloNombre = '';
      setTimeout(() => modeloMsg = '', 4000);
      
      await loadModelos();
    } catch (e) {
      console.error(e);
      modeloMsg = 'Error al guardar el modelo: ' + String(e);
    }
  }

  async function handleRemoveModelo(id) {
    if (!confirm('¿Estás seguro de eliminar este modelo de la bóveda?')) return;
    try {
      await modelosService.remove(id);
      await loadModelos();
    } catch (e) { console.error(e); }
  }



  async function handleRestoreClick() {
    backupMsg = ''; backupError = '';
    try {
      const selected = await openDialog({
        multiple: false,
        title: 'Seleccionar Archivo de Respaldo Total',
        filters: [{ name: 'Archivos de Respaldo', extensions: ['zip'] }]
      });
      if (!selected) return;

      pendingRestorePath = typeof selected === 'string' ? selected : selected.path;
      showRestoreConfirm = true;
    } catch (e) {
      console.error(e);
      backupError = String(e);
    }
  }

  async function confirmRestore() {
    showRestoreConfirm = false;
    restoringBackup = true;
    backupMsg = 'Descomprimiendo archivos y restaurando base de datos...';
    try {
      await invoke('restaurar_backup_completo', { sourcePath: pendingRestorePath });
      window.location.reload();
    } catch (e) {
      console.error(e);
      backupError = 'Error restaurando: ' + String(e);
    } finally {
      restoringBackup = false;
    }
  }

  async function handleFactoryReset(e) {
    e.preventDefault();
    if (!factoryResetPassword) return;
    
    factoryResetError = '';
    isFactoryResetting = true;
    try {
      await invoke('eliminar_base_datos', { contrasenaIngresada: factoryResetPassword });
      showFactoryResetModal = false;
      window.location.reload();
    } catch (err) {
      factoryResetError = String(err);
    } finally {
      isFactoryResetting = false;
    }
  }
</script>

<div class="config-page fade-in">
  {#if loading}
    <div class="loading-center"><div class="spinner"></div></div>
  {:else}
    <Card>
      <div class="card-header">
        <h2>Configuración del Despacho</h2>
        <p class="subtitle">Datos que aparecerán en los reportes y cuentas de cobro generadas por el sistema.</p>
      </div>

      <form onsubmit={handleSave} class="config-form">
        <h3 class="section-title">Datos Profesionales</h3>
        <div class="form-row">
          <TextInput label="Nombre del Abogado / Firma" id="nombre_abogado" bind:value={config.nombre_abogado} placeholder="Nombre completo" />
          <TextInput label="Título Profesional" id="titulo_profesional" bind:value={config.titulo_profesional} placeholder="Ej: Abogado y Profesor" />
        </div>
        <div class="form-row">
          <TextInput label="Tarjeta Profesional" id="tarjeta_profesional" bind:value={config.tarjeta_profesional} placeholder="T.P. No." />
          <TextInput label="Dirección Oficina" id="direccion_oficina" bind:value={config.direccion_oficina} placeholder="Ej: Calle 100 # 10-20" />
        </div>
        <div class="form-row">
          <TextInput label="Teléfono / Celular" id="celular" bind:value={config.celular} />
          <TextInput label="Correo Electrónico" id="email" bind:value={config.email} />
        </div>


        {#if successMsg}
          <div class="success-msg">{successMsg}</div>
        {/if}

        <div class="form-actions">
          <Button variant="primary" type="submit" disabled={saving}>
            {saving ? 'Guardando...' : 'Guardar Configuración'}
          </Button>
        </div>
      </form>

      <div class="form-divider" style="margin-top: var(--sp-6)"></div>

      <div class="config-form">
        <h3 class="section-title">Bóveda de Modelos Legales</h3>
        <p class="help-text">Guarda tus plantillas maestras de Word o Pages. Al crear una actuación, podrás clonarlas e integrarlas al expediente automáticamente.</p>
        
        <form class="template-box" style="flex-direction: column; align-items: stretch; gap: var(--sp-3);" onsubmit={handleUploadModelo}>
          <div style="display: grid; grid-template-columns: 2fr 1fr; gap: var(--sp-3);">
            <TextInput label="Nombre del Modelo" id="modelo_nombre" bind:value={newModeloNombre} placeholder="Ej: Poder Especial Laboral" />
            <Select label="Categoría" id="modelo_tipo" bind:value={newModeloTipo} options={modeloTipos} />
          </div>
          <div style="display: flex; justify-content: flex-end;">
            <Button variant="secondary" type="submit">Seleccionar Archivo y Guardar</Button>
          </div>
        </form>
        
        {#if modeloMsg}
          <div class="success-msg" style="text-align: left; color: var(--accent-blue);">{modeloMsg}</div>
        {/if}

        {#if loadingModelos}
          <div class="loading-center" style="padding: var(--sp-4)"><div class="spinner"></div></div>
        {:else if modelos.length > 0}
          <div class="modelos-list">
            {#each modelos as mod}
              <div class="modelo-item">
                <div class="modelo-info">
                  <strong>{mod.nombre}</strong>
                  <Badge variant="blue">{mod.tipo}</Badge>
                </div>
                <button class="icon-btn delete" onclick={() => handleRemoveModelo(mod.id)} title="Eliminar modelo">
                  <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="3 6 5 6 21 6"/><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/></svg>
                </button>
              </div>
            {/each}
          </div>
        {:else}
          <div class="help-text" style="text-align: center; padding: var(--sp-4);">No hay modelos en la bóveda. Sube el primero.</div>
        {/if}
      </div>

      <div class="form-divider" style="margin-top: var(--sp-6)"></div>
      
      <form onsubmit={handleChangePassword} class="config-form">
        <h3 class="section-title">Seguridad y Acceso</h3>
        <p class="help-text">Cambia la contraseña maestra de encriptación de la base de datos.</p>

        <div class="form-row">
          <TextInput label="Nueva Contraseña" id="new_pwd" type="password" bind:value={newPassword} />
          <TextInput label="Confirmar Nueva Contraseña" id="confirm_pwd" type="password" bind:value={confirmNewPassword} />
        </div>

        {#if pwdError}
          <div class="error-msg">{pwdError}</div>
        {/if}
        {#if pwdSuccess}
          <div class="success-msg" style="text-align: left">{pwdSuccess}</div>
        {/if}

        <div class="form-actions">
          <Button variant="secondary" type="submit" disabled={changingPwd}>
            {changingPwd ? 'Actualizando...' : 'Cambiar Contraseña'}
          </Button>
        </div>
      </form>



      <div class="form-divider" style="margin-top: var(--sp-6)"></div>

      <div class="config-form">
        <h3 class="section-title">Respaldo de Datos y Seguridad</h3>
        <p class="help-text">Guarda una copia exacta de tu base de datos o restaura el sistema desde un punto anterior.</p>
        
        <div class="backup-actions">
          <Button variant="primary" type="button" onclick={handleFullBackup} disabled={generatingBackup}>
            {#if generatingBackup}
               <div class="spinner-small" style="margin-right: 8px;"></div> Comprimiendo archivos...
            {:else}
              <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" style="margin-right:8px;vertical-align:middle"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="7 10 12 15 17 10"/><line x1="12" y1="15" x2="12" y2="3"/></svg>
              Generar Copia de Seguridad Total
            {/if}
          </Button>
          <Button variant="secondary" type="button" onclick={handleExportBackup} disabled={generatingBackup}>
            Solo Base de Datos (.db)
          </Button>
          <Button variant="ghost" type="button" onclick={handleRestoreClick} style="color:var(--accent-red)" disabled={generatingBackup || restoringBackup}>
            {#if restoringBackup}
              <div class="spinner-small" style="margin-right: 8px;"></div> Restaurando...
            {:else}
              <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" style="margin-right:8px;vertical-align:middle"><path d="M3 12a9 9 0 1 0 9-9 9.75 9.75 0 0 0-6.74 2.74L3 8"/><path d="M3 3v5h5"/></svg>
              Restaurar Copia Total
            {/if}
          </Button>
        </div>

        {#if backupError}
          <div class="error-msg">{backupError}</div>
        {/if}
        {#if backupMsg}
          <div class="success-msg" style="text-align: left">{backupMsg}</div>
        {/if}
      </div>

      <div class="form-divider" style="margin-top: var(--sp-6)"></div>

      <div class="config-form" style="border: 1px solid var(--accent-red-soft); padding: var(--sp-4); border-radius: var(--radius-md); background: var(--bg-surface);">
        <h3 class="section-title" style="color: var(--accent-red);">Zona de Peligro</h3>
        <p class="help-text">Elimina por completo la base de datos actual y restablece el sistema de fábrica. Esta acción es irreversible.</p>
        
        <div class="form-actions" style="margin-top: var(--sp-2); justify-content: flex-start;">
          <Button variant="danger" type="button" onclick={() => { showFactoryResetModal = true; factoryResetPassword = ''; factoryResetError = ''; }}>
            Restablecer Sistema (Borrar Todo)
          </Button>
        </div>
      </div>
    </Card>
  {/if}
</div>

<ConfirmDialog 
  open={showRestoreConfirm} 
  title="Restaurar Copia de Seguridad"
  message="¿Estás seguro de que deseas sobrescribir la base de datos actual? Esta acción destruirá los datos actuales y los reemplazará con los del archivo seleccionado. Tendrás que reiniciar la aplicación luego."
  confirmText="Sí, Sobrescribir Base de Datos"
  cancelText="Cancelar"
  danger={true}
  onconfirm={confirmRestore}
  oncancel={() => showRestoreConfirm = false}
/>

<Modal open={showFactoryResetModal} title="⚠️ Peligro: Restablecimiento de Fábrica" onclose={() => !isFactoryResetting && (showFactoryResetModal = false)}>
  <div style="margin-bottom: var(--sp-4); color: var(--text-secondary); line-height: 1.5; font-size: 0.9375rem;">
    <p style="margin-bottom: var(--sp-2);">
      Estás a punto de <strong>eliminar permanentemente</strong> todos los registros de casos, clientes, actuaciones y eventos. 
      Esta acción es irreversible y tu base de datos actual se perderá.
    </p>
    <p>
      <em>Nota: Los archivos físicos en tu Expediente Digital no serán eliminados.</em>
    </p>
  </div>

  <form onsubmit={handleFactoryReset}>
    <TextInput 
      label="Para confirmar, ingresa tu contraseña maestra:" 
      type="password" 
      bind:value={factoryResetPassword} 
      required 
    />
    
    {#if factoryResetError}
      <div class="error-msg" style="margin-top: var(--sp-2);">{factoryResetError}</div>
    {/if}

    <div class="form-actions" style="margin-top: var(--sp-5); display: flex; justify-content: flex-end; gap: var(--sp-3);">
      <Button variant="ghost" type="button" onclick={() => showFactoryResetModal = false} disabled={isFactoryResetting}>
        Cancelar
      </Button>
      <Button variant="danger" type="submit" disabled={isFactoryResetting}>
        {isFactoryResetting ? 'Eliminando...' : 'Eliminar Base de Datos'}
      </Button>
    </div>
  </form>
</Modal>

<style>
  .config-page { max-width: 800px; margin: 0 auto; display: flex; flex-direction: column; gap: var(--sp-5); }
  .card-header { margin-bottom: var(--sp-6); }
  .card-header h2 { font-size: 1.25rem; font-weight: 600; color: var(--text-primary); }
  .subtitle { font-size: 0.875rem; color: var(--text-secondary); margin-top: var(--sp-1); }
  
  .config-form { display: flex; flex-direction: column; gap: var(--sp-4); }
  .form-row { display: grid; grid-template-columns: 1fr 1fr; gap: var(--sp-4); }
  .form-divider { height: 1px; background: var(--border-subtle); margin: var(--sp-2) 0; }
  .section-title { font-size: 0.875rem; font-weight: 600; color: var(--text-primary); text-transform: uppercase; letter-spacing: 0.05em; }
  .help-text { font-size: 0.8125rem; color: var(--text-muted); margin-top: -8px; margin-bottom: 8px; line-height: 1.5; }
  
  .template-box {
    display: flex; align-items: center; justify-content: space-between;
    padding: var(--sp-4); background: var(--bg-input);
    border: 1px dashed var(--border-focus); border-radius: var(--radius-md);
  }
  .template-info { display: flex; align-items: center; gap: var(--sp-3); }
  .template-info strong { font-size: 0.875rem; display: block; margin-bottom: 2px; }
  .path-text { font-size: 0.75rem; color: var(--text-muted); }
  
  .form-actions { display: flex; justify-content: flex-end; margin-top: var(--sp-4); }
  .success-msg { color: var(--accent-green); font-size: 0.875rem; text-align: right; }
  .error-msg { color: var(--accent-red); font-size: 0.875rem; }
  
  .loading-center { display: flex; justify-content: center; padding: var(--sp-10); }
  .spinner { width: 28px; height: 28px; border: 3px solid var(--border-default); border-top-color: var(--accent-blue); border-radius: 50%; animation: spin 0.8s linear infinite; }
  .spinner-small { width: 14px; height: 14px; border: 2px solid var(--bg-surface); border-top-color: transparent; border-radius: 50%; animation: spin 0.8s linear infinite; display: inline-block; vertical-align: middle; }


  /* API Setup Styles */
  .api-setup-toggle {
    display: flex; align-items: center; gap: var(--sp-2);
    width: 100%; padding: var(--sp-3); margin-top: var(--sp-4);
    background: transparent; border: 1px solid var(--border-subtle); border-radius: var(--radius-md);
    color: var(--text-secondary); font-size: 0.875rem; font-weight: 500; cursor: pointer;
    transition: all 0.2s ease; text-align: left;
  }
  .api-setup-toggle:hover { background: var(--bg-hover); color: var(--text-primary); border-color: var(--border-default); }
  .api-setup-panel {
    margin-top: var(--sp-2); padding: var(--sp-4);
    background: var(--bg-elevated); border: 1px solid var(--border-subtle); border-radius: var(--radius-md);
  }
  .cred-row { display: flex; align-items: center; gap: var(--sp-3); }
  .cred-label { width: 70px; font-weight: 600; font-size: 0.875rem; color: var(--text-primary); }

  .modelos-list {
    display: flex; flex-direction: column; gap: var(--sp-2);
    margin-top: var(--sp-2);
  }
  .modelo-item {
    display: flex; align-items: center; justify-content: space-between;
    padding: var(--sp-3); background: var(--bg-input);
    border: 1px solid var(--border-default); border-radius: var(--radius-md);
  }
  .modelo-info { display: flex; align-items: center; gap: var(--sp-3); }
  .modelo-info strong { font-size: 0.875rem; color: var(--text-primary); }
  .icon-btn { background: none; border: none; cursor: pointer; padding: 4px; border-radius: 4px; color: var(--text-muted); }
  .icon-btn:hover { color: var(--accent-red); background: var(--bg-hover); }
</style>
