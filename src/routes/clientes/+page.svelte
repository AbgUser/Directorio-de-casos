<script>
  import ClientesList from '$lib/components/clientes/ClientesList.svelte';
  import ClienteForm from '$lib/components/clientes/ClienteForm.svelte';
  import * as clienteService from '$lib/services/clienteService.js';
  import { goto } from '$app/navigation';

  let clientes = $state([]);
  let showForm = $state(false);
  let loading = $state(true);

  $effect(() => { loadClientes(); });

  async function loadClientes() {
    loading = true;
    try { clientes = await clienteService.getAll(); }
    catch (e) { console.error('Error cargando clientes:', e); }
    finally { loading = false; }
  }

  function handleNewClient() { showForm = true; }
  function handleSelect(id) { goto(`/clientes/${id}`); }
  async function handleSaved() {
    showForm = false;
    await loadClientes();
  }
</script>

<div class="page fade-in">
  {#if loading}
    <div class="loading-center"><div class="spinner"></div></div>
  {:else}
    <ClientesList
      {clientes}
      onnewclient={handleNewClient}
      onselect={handleSelect}
    />
  {/if}
</div>

<ClienteForm
  open={showForm}
  onclose={() => showForm = false}
  onsave={handleSaved}
/>

<style>
  .page { height: 100%; }
  .loading-center { display: flex; justify-content: center; align-items: center; height: 200px; }
  .spinner { width: 28px; height: 28px; border: 3px solid var(--border-default); border-top-color: var(--accent-blue); border-radius: 50%; animation: spin 0.8s linear infinite; }
</style>
