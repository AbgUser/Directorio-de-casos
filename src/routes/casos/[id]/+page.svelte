<script>
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { invoke } from '@tauri-apps/api/core';
  import { onDestroy } from 'svelte';
  import { remove } from '@tauri-apps/plugin-fs';
  import { openUrl } from '@tauri-apps/plugin-opener';
  import Card from '$lib/components/ui/Card.svelte';
  import Badge from '$lib/components/ui/Badge.svelte';
  import Button from '$lib/components/ui/Button.svelte';
  import Tabs from '$lib/components/ui/Tabs.svelte';
  import CasoForm from '$lib/components/casos/CasoForm.svelte';
  import CasoTimeline from '$lib/components/casos/CasoTimeline.svelte';
  import CasoDocumentos from '$lib/components/casos/CasoDocumentos.svelte';
  import CasoAgenda from '$lib/components/casos/CasoAgenda.svelte';
  import EmptyState from '$lib/components/ui/EmptyState.svelte';
  import ConfirmDialog from '$lib/components/ui/ConfirmDialog.svelte';
  import Modal from '$lib/components/ui/Modal.svelte';
  import TextInput from '$lib/components/ui/TextInput.svelte';
  import * as casoService from '$lib/services/casoService.js';
  import * as clienteService from '$lib/services/clienteService.js';
  import * as actuacionService from '$lib/services/actuacionService.js';
  import * as documentoService from '$lib/services/documentoService.js';
  import * as pdfService from '$lib/services/pdfService.js';
  import { formatDate, getEstadoBadgeVariant, getTipoProcesoLabel } from '$lib/utils/formatters.js';

  let id = $derived(Number($page.params.id));
  let caso = $state(null);
  let allClientes = $state([]);
  let loading = $state(true);
  let showEditForm = $state(false);
  let showDeleteConfirm = $state(false);
  let activeTab = $state('info');
  let showPortalDropdown = $state(false);
  let docsRef = $state(null);

  // Expediente Digital URL state
  let showExpedienteModal = $state(false);
  let expedienteUrlInput = $state('');
  let savingExpedienteUrl = $state(false);

  const tabs = [
    { id: 'info', label: 'Información' },
    { id: 'actuaciones', label: 'Actuaciones' },
    { id: 'documentos', label: 'Expediente' },
    { id: 'agenda', label: 'Agenda' }
  ];

  $effect(() => { if (id) loadData(); });

  $effect(() => {
    const tabParam = $page.url.searchParams.get('tab');
    if (tabParam && tabs.find(t => t.id === tabParam)) {
      activeTab = tabParam;
    }
  });

  async function loadData() {
    loading = true;
    try {
      [caso, allClientes] = await Promise.all([
        casoService.getById(id),
        clienteService.getAll()
      ]);
    } catch (e) { console.error(e); }
    finally { loading = false; }
  }

  let exporting = $state(false);
  let tempReportPath = null;

  onDestroy(() => {
    if (tempReportPath) {
      remove(tempReportPath).catch(e => console.log("Cleanup temporal no requerido o falló"));
    }
  });

  async function handleExportPDF() {
    if (!caso) return;
    exporting = true;
    try {
      const cliente = allClientes.find(c => c.id === caso.cliente_id) || {};
      const actuaciones = await actuacionService.getByCaso(caso.id);
      
      const outputPath = await pdfService.generateReportePDF(caso, cliente, actuaciones, []);
      
      // Intentar abrir el PDF generado
      import('@tauri-apps/api/core').then(core => {
        core.invoke('abrir_documento', { path: outputPath });
      }).catch(e => console.error("Could not auto-open PDF", e));

      tempReportPath = outputPath;

    } catch (e) {
      console.error('Error exportando PDF:', e);
      alert('Hubo un error al exportar el reporte: ' + String(e));
    } finally {
      exporting = false;
    }
  }

  async function abrirPortal(url) {
    showPortalDropdown = false;
    try {
      if (caso.radicado) {
        try {
          await navigator.clipboard.writeText(caso.radicado);
          import('@tauri-apps/plugin-notification').then(async ({ isPermissionGranted, requestPermission, sendNotification }) => {
            let granted = await isPermissionGranted();
            if (!granted) granted = (await requestPermission()) === 'granted';
            if (granted) {
              sendNotification({
                title: 'Radicado Copiado',
                body: `Listo para pegar en el navegador.`
              });
            }
          });
        } catch (err) {
          console.warn('No se pudo copiar automáticamente:', err);
        }
      }
      
      await openUrl(url);
    } catch (e) {
      alert("Error al abrir portal: " + String(e));
    }
  }

  function openExpedienteModal() {
    expedienteUrlInput = caso?.expediente_url || '';
    showExpedienteModal = true;
  }

  // Añade https:// si el enlace se pegó sin protocolo (ej. "drive.google.com/...")
  function normalizarUrl(url) {
    const limpio = url.trim();
    if (!limpio) return null;
    return /^https?:\/\//i.test(limpio) ? limpio : `https://${limpio}`;
  }

  async function saveExpedienteUrl() {
    savingExpedienteUrl = true;
    try {
      const url = normalizarUrl(expedienteUrlInput);
      await casoService.update(id, { expediente_url: url });
      caso.expediente_url = url;
      showExpedienteModal = false;
    } catch (e) {
      alert('Error al guardar el enlace: ' + String(e));
    } finally {
      savingExpedienteUrl = false;
    }
  }

  async function abrirExpedienteDigital() {
    if (!caso?.expediente_url) return;
    try {
      await openUrl(caso.expediente_url);
    } catch (e) {
      alert('Error al abrir el expediente: ' + String(e));
    }
  }

  async function confirmDeleteCaso() {
    loading = true;
    try {
      await casoService.remove(id);
      goto('/casos');
    } catch (e) {
      alert(e.message || String(e));
      loading = false;
    } finally {
      showDeleteConfirm = false;
    }
  }
</script>

<ConfirmDialog 
  open={showDeleteConfirm} 
  title="Eliminar Caso Permanentemente"
  message="¿Estás seguro de que deseas eliminar permanentemente este caso y todas sus actuaciones asociadas? Esta acción no se puede deshacer."
  confirmLabel="Sí, Eliminar Caso"
  cancelLabel="Cancelar"
  variant="danger"
  onconfirm={confirmDeleteCaso}
  oncancel={() => showDeleteConfirm = false}
/>

<div class="detail-page fade-in">
  <div class="page-toolbar">
    <button class="back-btn" onclick={() => goto('/casos')}>
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="19" y1="12" x2="5" y2="12"/><polyline points="12 19 5 12 12 5"/></svg>
      Volver al directorio
    </button>
    {#if caso}
      <div style="display: flex; gap: 8px;">
        <Button variant="ghost" size="sm" onclick={handleExportPDF} disabled={exporting}>
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8"/><line x1="12" y1="18" x2="12" y2="12"/><line x1="9" y1="15" x2="15" y2="15"/></svg>
          {exporting ? 'Generando...' : 'Exportar PDF'}
        </Button>
        <Button variant="ghost" size="sm" onclick={() => showEditForm = true}>
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"/><path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"/></svg>
          Editar
        </Button>
        <Button variant="ghost" size="sm" onclick={() => showDeleteConfirm = true} style="color: var(--accent-red);">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="3 6 5 6 21 6"></polyline><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path><line x1="10" y1="11" x2="10" y2="17"></line><line x1="14" y1="11" x2="14" y2="17"></line></svg>
          Eliminar
        </Button>
      </div>
    {/if}
  </div>

  {#if loading}
    <div class="loading-center"><div class="spinner"></div></div>
  {:else if !caso}
    <EmptyState title="Caso no encontrado" message="El expediente solicitado no existe o ha sido eliminado." />
  {:else}
    <!-- Case Header -->
    <div class="case-header">
      <div class="case-header-main">
        <div class="case-radicado-row">
          <span class="case-radicado">{caso.radicado || caso.slug || 'Sin radicado'}</span>
          <Badge variant={getEstadoBadgeVariant(caso.estado)}>{caso.estado}</Badge>
          <Badge variant="blue">{caso.tipo_proceso || '—'}</Badge>

          {#if (caso.categoria_expediente || 'proceso') === 'proceso' && caso.radicado}
            <div class="portal-dropdown-wrapper">
              <Button variant="ghost" size="sm" onclick={() => showPortalDropdown = !showPortalDropdown} style="color: var(--accent-blue); padding: 4px 8px; font-size: 0.75rem;">
                <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" style="margin-right: 4px;"><circle cx="12" cy="12" r="10"/><line x1="2" y1="12" x2="22" y2="12"/><path d="M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z"/></svg>
                Consultar Estado
              </Button>
              {#if showPortalDropdown}
                <div class="portal-dropdown-menu">
                  <button class="portal-item" onclick={() => abrirPortal('https://consultaprocesos.ramajudicial.gov.co/')}>Consulta Nacional Unificada (CNU)</button>
                  <button class="portal-item" onclick={() => abrirPortal('https://siugj.ramajudicial.gov.co/')}>SIUGJ / Rama Judicial - Consulta</button>
                  <button class="portal-item" onclick={() => abrirPortal('https://samai.consejodeestado.gov.co/')}>SAMAI (Consejo de Estado / Tribunales)</button>
                  <button class="portal-item" onclick={() => abrirPortal('https://procesojudicial.ramajudicial.gov.co/Justicia21/Administracion/Ciudadanos/frmConsulta')}>TYBA (Justicia XXI)</button>
                </div>
              {/if}
            </div>
          {/if}
        </div>
        {#if caso.contraparte}
          <p class="case-parties">Contra: {caso.contraparte}</p>
        {/if}
        <div class="expediente-row">
          {#if caso.expediente_url}
            <button class="expediente-btn active" onclick={abrirExpedienteDigital}>
              <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/></svg>
              Ver Expediente Digital
            </button>
            <button class="expediente-edit-btn" onclick={openExpedienteModal} title="Editar enlace">
              <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"/><path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"/></svg>
            </button>
          {:else}
            <button class="expediente-btn add" onclick={openExpedienteModal}>
              <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/><line x1="12" y1="11" x2="12" y2="17"/><line x1="9" y1="14" x2="15" y2="14"/></svg>
              Añadir expediente digital +
            </button>
          {/if}
        </div>
        <p class="case-client">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2"/><circle cx="9" cy="7" r="4"/></svg>
          <a class="client-link" onclick={() => goto(`/clientes/${caso.cliente_id}`)}>{caso.cliente_display_name || 'Sin cliente'}</a>
        </p>
      </div>
    </div>

    <Tabs {tabs} bind:activeTab />

    {#if activeTab === 'info'}
      <Card>
        <div class="info-grid">
          <div class="info-item"><span class="info-label">Radicado</span><span class="info-value mono">{caso.radicado || '—'}</span></div>
          <div class="info-item"><span class="info-label">Tipo de Proceso</span><span class="info-value">{caso.tipo_proceso || '—'}</span></div>
          <div class="info-item"><span class="info-label">Estado</span><span class="info-value">{caso.estado}</span></div>
          <div class="info-item"><span class="info-label">Fecha de Radicación / Inicio</span><span class="info-value">{formatDate(caso.fecha_inicio)}</span></div>
          <div class="info-divider"></div>
          <div class="info-item"><span class="info-label">Nombre de la Contraparte</span><span class="info-value">{caso.contraparte || '—'}</span></div>
          <div class="info-item"><span class="info-label">Juzgado / Despacho</span><span class="info-value">{caso.juzgado || '—'}</span></div>
          {#if caso.descripcion}
            <div class="info-divider"></div>
            <div class="info-item full"><span class="info-label">Descripción</span><span class="info-value">{caso.descripcion}</span></div>
          {/if}
          {#if caso.notas_internas}
            <div class="info-item full"><span class="info-label">Notas Internas</span><span class="info-value">{caso.notas_internas}</span></div>
          {/if}
          {#if caso.carpeta_documentos}
            <div class="info-divider"></div>
            <div class="info-item full">
              <span class="info-label">Carpeta de Documentos</span>
              <span class="info-value mono text-sm">Documentos/Directorio_Casos/{caso.carpeta_documentos}</span>
            </div>
          {/if}
        </div>
      </Card>
    {:else if activeTab === 'actuaciones'}
      <CasoTimeline casoId={id} />
    {:else if activeTab === 'documentos'}
      <CasoDocumentos casoId={id} carpetaDocumentos={caso.carpeta_documentos || ''} bind:this={docsRef} />
    {:else if activeTab === 'agenda'}
      <CasoAgenda casoId={id} />
    {/if}
  {/if}
</div>

<CasoForm open={showEditForm} {caso} clientes={allClientes} onclose={() => showEditForm = false} onsave={() => { showEditForm = false; loadData(); }} />

<Modal open={showExpedienteModal} title="Enlace al Expediente Digital" onclose={() => showExpedienteModal = false}>
  <div style="display: flex; flex-direction: column; gap: var(--sp-4);">
    <p style="font-size: 0.8125rem; color: var(--text-secondary); line-height: 1.5;">
      Pega la URL del expediente digital del proceso (Google Drive, OneDrive, SharePoint, etc.).
    </p>
    <TextInput
      label="URL del Expediente"
      id="expediente_url_input"
      bind:value={expedienteUrlInput}
      placeholder="https://drive.google.com/..."
    />
    <div style="display: flex; justify-content: flex-end; gap: var(--sp-3); margin-top: var(--sp-2);">
      <Button variant="ghost" onclick={() => showExpedienteModal = false}>Cancelar</Button>
      <Button variant="primary" onclick={saveExpedienteUrl} disabled={savingExpedienteUrl}>
        {savingExpedienteUrl ? 'Guardando...' : 'Guardar'}
      </Button>
    </div>
  </div>
</Modal>

<style>
  .detail-page { display: flex; flex-direction: column; gap: var(--sp-5); }
  .page-toolbar { display: flex; align-items: center; justify-content: space-between; }
  .back-btn {
    display: flex; align-items: center; gap: var(--sp-2);
    background: none; border: none; color: var(--text-secondary);
    font-family: var(--font-sans); font-size: 0.875rem; cursor: pointer;
    transition: color var(--ease-fast);
  }
  .back-btn:hover { color: var(--accent-blue); }
  .loading-center { display: flex; justify-content: center; padding: var(--sp-10); }
  .spinner { width: 28px; height: 28px; border: 3px solid var(--border-default); border-top-color: var(--accent-blue); border-radius: 50%; animation: spin 0.8s linear infinite; }

  .case-header {
    padding: var(--sp-5); background: var(--bg-surface);
    border: 1px solid var(--border-default); border-radius: var(--radius-lg);
  }
  .case-radicado-row { display: flex; align-items: center; gap: var(--sp-2); margin-bottom: var(--sp-2); flex-wrap: wrap; }
  .portal-dropdown-wrapper { position: relative; margin-left: auto; }
  .portal-dropdown-menu {
    position: absolute; top: 100%; right: 0; margin-top: var(--sp-1);
    background: var(--bg-elevated); border: 1px solid var(--border-default);
    border-radius: var(--radius-md); box-shadow: var(--shadow-lg);
    width: max-content; z-index: 100; overflow: hidden;
  }
  .portal-item {
    display: block; width: 100%; text-align: left; background: none; border: none;
    padding: var(--sp-2) var(--sp-4); font-size: 0.8125rem; color: var(--text-primary);
    cursor: pointer; transition: background var(--ease-fast);
  }
  .portal-item:hover { background: var(--bg-hover); }
  .case-radicado { font-family: var(--font-mono); font-size: 1.25rem; font-weight: 600; color: var(--accent-blue); }
  .case-parties { font-size: 0.9375rem; color: var(--text-secondary); margin-bottom: var(--sp-2); }
  .vs { font-style: italic; color: var(--text-muted); margin: 0 var(--sp-1); }
  .case-client { display: flex; align-items: center; gap: var(--sp-2); font-size: 0.8125rem; color: var(--text-muted); }
  .client-link { color: var(--accent-blue); cursor: pointer; transition: color var(--ease-fast); }
  .client-link:hover { color: var(--accent-blue-hover); }

  .info-grid { display: grid; grid-template-columns: 1fr 1fr; gap: var(--sp-4); }
  .info-item { display: flex; flex-direction: column; gap: 2px; }
  .info-item.full { grid-column: 1 / -1; }
  .info-label { font-size: 0.75rem; font-weight: 500; color: var(--text-muted); text-transform: uppercase; letter-spacing: 0.05em; }
  .info-value { font-size: 0.9375rem; color: var(--text-primary); line-height: 1.5; }
  .info-value.mono { font-family: var(--font-mono); }
  .info-value.text-sm { font-size: 0.8125rem; }
  .info-divider { grid-column: 1 / -1; height: 1px; background: var(--border-subtle); }

  /* Expediente Digital */
  .expediente-row {
    display: flex; align-items: center; gap: var(--sp-2);
    margin-bottom: var(--sp-2);
  }
  .expediente-btn {
    display: flex; align-items: center; gap: var(--sp-2);
    padding: 5px 12px; border-radius: var(--radius-md);
    font-size: 0.8125rem; font-weight: 500; cursor: pointer;
    border: 1px solid var(--border-subtle);
    transition: all var(--ease-fast); font-family: var(--font-sans);
  }
  .expediente-btn.add {
    background: transparent; color: var(--text-secondary);
  }
  .expediente-btn.add:hover {
    background: var(--bg-hover); color: var(--text-primary);
    border-color: var(--border-default);
  }
  .expediente-btn.active {
    background: var(--accent-blue-soft, rgba(56, 139, 253, 0.1));
    color: var(--accent-blue); border-color: var(--accent-blue-soft, rgba(56, 139, 253, 0.2));
  }
  .expediente-btn.active:hover {
    background: var(--accent-blue-soft, rgba(56, 139, 253, 0.18));
  }
  .expediente-edit-btn {
    display: flex; align-items: center; justify-content: center;
    width: 28px; height: 28px; border-radius: var(--radius-sm);
    background: transparent; border: 1px solid transparent;
    color: var(--text-muted); cursor: pointer;
    transition: all var(--ease-fast);
  }
  .expediente-edit-btn:hover {
    background: var(--bg-hover); color: var(--text-primary);
    border-color: var(--border-subtle);
  }
</style>
