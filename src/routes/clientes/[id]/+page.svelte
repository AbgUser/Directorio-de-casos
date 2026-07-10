<script>
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import Card from '$lib/components/ui/Card.svelte';
  import Badge from '$lib/components/ui/Badge.svelte';
  import Button from '$lib/components/ui/Button.svelte';
  import Tabs from '$lib/components/ui/Tabs.svelte';
  import ClienteForm from '$lib/components/clientes/ClienteForm.svelte';
  import CasoForm from '$lib/components/casos/CasoForm.svelte';
  import EmptyState from '$lib/components/ui/EmptyState.svelte';
  import ConfirmDialog from '$lib/components/ui/ConfirmDialog.svelte';
  import * as clienteService from '$lib/services/clienteService.js';
  import * as casoService from '$lib/services/casoService.js';
  import { formatDate, getEstadoBadgeVariant, getTipoProcesoLabel } from '$lib/utils/formatters.js';

  let id = $derived(Number($page.params.id));
  let cliente = $state(null);
  let casos = $state([]);
  let loading = $state(true);
  let showEditForm = $state(false);
  let showCasoForm = $state(false);
  let showDeleteConfirm = $state(false);
  let activeTab = $state('info');
  let allClientes = $state([]);

  const tabs = [
    { id: 'info', label: 'Información' },
    { id: 'casos', label: 'Casos Asociados' }
  ];

  $effect(() => { if (id) loadData(); });

  async function loadData() {
    loading = true;
    try {
      [cliente, casos, allClientes] = await Promise.all([
        clienteService.getById(id),
        casoService.getByCliente(id),
        clienteService.getAll()
      ]);
    } catch (e) { console.error(e); }
    finally { loading = false; }
  }

  function displayName() {
    if (!cliente) return '';
    return cliente.nombre_completo || '';
  }

  async function confirmDeleteCliente() {
    loading = true;
    try {
      await clienteService.remove(id);
      goto('/clientes');
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
  title="Eliminar Cliente Permanentemente"
  message="¿Estás seguro de que deseas eliminar permanentemente a este cliente? Esta acción no se puede deshacer."
  confirmText="Sí, Eliminar Cliente"
  cancelText="Cancelar"
  danger={true}
  onconfirm={confirmDeleteCliente}
  oncancel={() => showDeleteConfirm = false}
/>

<div class="detail-page fade-in">
  <div class="page-toolbar">
    <button class="back-btn" onclick={() => goto('/clientes')}>
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="19" y1="12" x2="5" y2="12"/><polyline points="12 19 5 12 12 5"/></svg>
      Volver al directorio
    </button>
    {#if cliente}
      <div style="display: flex; gap: var(--sp-2);">
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
  {:else if !cliente}
    <EmptyState title="Cliente no encontrado" message="El cliente solicitado no existe o ha sido eliminado." />
  {:else}
    <div class="client-header">
      <div class="client-avatar">{displayName().charAt(0).toUpperCase()}</div>
      <div>
        <h1 class="client-name">{displayName()}</h1>
        <Badge variant={cliente.tipo_persona === 'natural' ? 'blue' : 'purple'}>
          {cliente.tipo_persona === 'natural' ? 'Persona Natural' : 'Persona Jurídica'}
        </Badge>
      </div>
    </div>

    <Tabs {tabs} bind:activeTab />

    {#if activeTab === 'info'}
      <Card>
        <div class="info-grid">
          <div class="info-item"><span class="info-label">{cliente.tipo_persona === 'natural' ? 'Nombre Completo' : 'Razón Social'}</span><span class="info-value">{cliente.nombre_completo || '—'}</span></div>
          <div class="info-item"><span class="info-label">{cliente.tipo_identificacion || 'Identificación'}</span><span class="info-value mono">{cliente.identificacion || '—'}</span></div>
          <div class="info-divider"></div>
          <div class="info-item"><span class="info-label">Dirección</span><span class="info-value">{cliente.direccion || '—'}</span></div>
          <div class="info-item"><span class="info-label">Teléfono / WhatsApp</span><span class="info-value">{cliente.telefono || '—'}</span></div>
          <div class="info-item"><span class="info-label">Email</span><span class="info-value">{cliente.email || '—'}</span></div>
          {#if cliente.notas}
            <div class="info-divider"></div>
            <div class="info-item full"><span class="info-label">Notas / Perfil</span><span class="info-value">{cliente.notas}</span></div>
          {/if}
        </div>
      </Card>
    {:else}
      <div class="casos-section">
        <div class="section-header">
          <span class="section-count">{casos.length} caso{casos.length !== 1 ? 's' : ''}</span>
          <Button variant="primary" size="sm" onclick={() => showCasoForm = true}>
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/></svg>
            Nuevo Caso
          </Button>
        </div>
        {#if casos.length === 0}
          <EmptyState title="Sin casos asociados" message="Este cliente no tiene casos registrados." />
        {:else}
          <div class="casos-list">
            {#each casos as caso}
              <div class="caso-row" onclick={() => goto(`/casos/${caso.id}`)}>
                <div class="caso-main">
                  <span class="caso-radicado">{caso.radicado || caso.slug || '—'}</span>
                  <span class="caso-desc">{caso.descripcion || (caso.contraparte ? `Contra: ${caso.contraparte}` : 'Sin descripción')}</span>
                </div>
                <Badge variant="blue">{getTipoProcesoLabel(caso.tipo_proceso)}</Badge>
                <Badge variant={getEstadoBadgeVariant(caso.estado)}>{caso.estado}</Badge>
                <span class="caso-date">{formatDate(caso.fecha_inicio)}</span>
              </div>
            {/each}
          </div>
        {/if}
      </div>
    {/if}
  {/if}
</div>

<ClienteForm open={showEditForm} {cliente} onclose={() => showEditForm = false} onsave={() => { showEditForm = false; loadData(); }} />
<CasoForm open={showCasoForm} clienteId={id} clientes={allClientes} onclose={() => showCasoForm = false} onsave={() => { showCasoForm = false; loadData(); }} />

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
  .client-header { display: flex; align-items: center; gap: var(--sp-4); }
  .client-avatar {
    width: 48px; height: 48px; border-radius: var(--radius-lg);
    background: var(--accent-blue-soft); color: var(--accent-blue);
    display: flex; align-items: center; justify-content: center;
    font-size: 1.25rem; font-weight: 700; flex-shrink: 0;
  }
  .client-name { font-size: 1.375rem; font-weight: 700; margin-bottom: var(--sp-1); }
  .info-grid { display: grid; grid-template-columns: 1fr 1fr; gap: var(--sp-4); }
  .info-item { display: flex; flex-direction: column; gap: 2px; }
  .info-item.full { grid-column: 1 / -1; }
  .info-label { font-size: 0.75rem; font-weight: 500; color: var(--text-muted); text-transform: uppercase; letter-spacing: 0.05em; }
  .info-value { font-size: 0.9375rem; color: var(--text-primary); }
  .info-value.mono { font-family: var(--font-mono); }
  .info-divider { grid-column: 1 / -1; height: 1px; background: var(--border-subtle); }
  .casos-section { display: flex; flex-direction: column; gap: var(--sp-4); }
  .section-header { display: flex; align-items: center; justify-content: space-between; }
  .section-count { font-size: 0.8125rem; color: var(--text-muted); }
  .casos-list { display: flex; flex-direction: column; background: var(--bg-surface); border: 1px solid var(--border-default); border-radius: var(--radius-lg); overflow: hidden; }
  .caso-row {
    display: flex; align-items: center; gap: var(--sp-3);
    padding: var(--sp-3) var(--sp-4); border-bottom: 1px solid var(--border-subtle);
    cursor: pointer; transition: background var(--ease-fast);
  }
  .caso-row:hover { background: var(--bg-hover); }
  .caso-row:last-child { border-bottom: none; }
  .caso-main { flex: 1; display: flex; flex-direction: column; gap: 2px; min-width: 0; }
  .caso-radicado { font-family: var(--font-mono); font-size: 0.8125rem; font-weight: 500; color: var(--accent-blue); }
  .caso-desc { font-size: 0.75rem; color: var(--text-muted); overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
  .caso-date { font-size: 0.75rem; color: var(--text-muted); white-space: nowrap; }
</style>
