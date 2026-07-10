<script>
  import CasosList from '$lib/components/casos/CasosList.svelte';
  import CasoForm from '$lib/components/casos/CasoForm.svelte';
  import * as casoService from '$lib/services/casoService.js';
  import * as clienteService from '$lib/services/clienteService.js';
  import { goto } from '$app/navigation';

  let casos = $state([]);
  let clientes = $state([]);
  let showForm = $state(false);
  let loading = $state(true);

  $effect(() => { loadData(); });

  async function loadData() {
    loading = true;
    try {
      [casos, clientes] = await Promise.all([
        casoService.getAll(),
        clienteService.getAll()
      ]);
    } catch (e) { console.error('Error cargando datos:', e); }
    finally { loading = false; }
  }

  function handleNewCaso() { showForm = true; }
  function handleSelect(id) { goto(`/casos/${id}`); }
  async function handleSaved() {
    showForm = false;
    await loadData();
  }
</script>

<div class="page fade-in">
  {#if loading}
    <div class="loading-center"><div class="spinner"></div></div>
  {:else}
    <CasosList
      {casos}
      onnewcaso={handleNewCaso}
      onselect={handleSelect}
    />
  {/if}
</div>

<CasoForm
  open={showForm}
  {clientes}
  onclose={() => showForm = false}
  onsave={handleSaved}
/>

<style>
  .page { height: 100%; }
  .loading-center { display: flex; justify-content: center; align-items: center; height: 200px; }
  .spinner { width: 28px; height: 28px; border: 3px solid var(--border-default); border-top-color: var(--accent-blue); border-radius: 50%; animation: spin 0.8s linear infinite; }
</style>
