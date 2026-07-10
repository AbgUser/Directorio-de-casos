<script>
  import Badge from '$lib/components/ui/Badge.svelte';
  import EmptyState from '$lib/components/ui/EmptyState.svelte';
  import { getEstadoBadgeVariant, getTipoProcesoLabel, formatDate } from '$lib/utils/formatters.js';

  let {
    casos = [],
    onnewcaso = () => {},
    onselect = () => {}
  } = $props();

  let searchQuery = $state('');
  let filterEstado = $state('Todos');
  let activeCategoria = $state('proceso');

  const estadoFilters = [
    { key: 'Todos', label: 'Todos' },
    { key: 'Activo', label: 'Activos' },
    { key: 'Suspendido', label: 'Suspendidos' },
    { key: 'Terminado', label: 'Terminados' },
    { key: 'Archivado', label: 'Archivados' }
  ];

  let filtered = $derived(() => {
    let list = casos.filter(c => (c.categoria_expediente || 'proceso') === activeCategoria);
    if (filterEstado !== 'Todos') {
      list = list.filter(c => c.estado === filterEstado);
    }
    if (searchQuery.trim()) {
      const q = searchQuery.toLowerCase();
      list = list.filter(c =>
        (c.radicado || '').toLowerCase().includes(q) ||
        (c.contraparte || '').toLowerCase().includes(q) ||
        (c.cliente_display_name || '').toLowerCase().includes(q) ||
        (c.juzgado || '').toLowerCase().includes(q)
      );
    }
    return list;
  });
</script>

<div class="casos-list fade-in">
  <div class="master-tabs">
    <button class="master-tab" class:active={activeCategoria === 'proceso'} onclick={() => activeCategoria = 'proceso'}>Procesos Judiciales</button>
    <button class="master-tab" class:active={activeCategoria === 'consulta'} onclick={() => activeCategoria = 'consulta'}>Consultas Jurídicas</button>
  </div>

  <div class="list-toolbar">
    <div class="search-box">
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/></svg>
      <input type="text" class="search-input" placeholder="Buscar por radicado, partes, cliente..." bind:value={searchQuery} />
    </div>
    <button class="btn-new" onclick={onnewcaso}>
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/></svg>
      Nuevo Caso
    </button>
  </div>

  <div class="filter-bar">
    {#each estadoFilters as f}
      <button
        class="filter-btn"
        class:active={filterEstado === f.key}
        onclick={() => filterEstado = f.key}
      >{f.label}</button>
    {/each}
  </div>

  {#if filtered().length === 0}
    <EmptyState
      title={searchQuery || filterEstado !== 'Todos' ? 'Sin resultados' : 'No hay casos registrados'}
      message={searchQuery ? 'Intente con otros términos de búsqueda.' : 'Cree su primer caso para comenzar la gestión de expedientes.'}
    />
  {:else}
    <div class="table-container">
      <table class="data-table">
        <thead>
          <tr>
            {#if activeCategoria === 'proceso'}
              <th>Radicado</th>
              <th>Cliente</th>
              <th>Tipo</th>
              <th>Estado</th>
              <th>Juzgado</th>
              <th>Inicio</th>
            {:else}
              <th>Referencia</th>
              <th>Cliente</th>
              <th>Asunto / Motivo</th>
              <th>Estado</th>
              <th>Fecha</th>
            {/if}
          </tr>
        </thead>
        <tbody>
          {#each filtered() as caso}
            <tr class="table-row" onclick={() => onselect(caso.id)}>
              {#if activeCategoria === 'proceso'}
                <td class="cell-radicado">{caso.radicado || '—'}</td>
                <td class="cell-name">{caso.cliente_display_name || '—'}</td>
                <td><Badge variant="blue">{caso.tipo_proceso || '—'}</Badge></td>
                <td><Badge variant={getEstadoBadgeVariant(caso.estado)}>{caso.estado}</Badge></td>
                <td class="cell-secondary">{caso.juzgado || '—'}</td>
                <td class="cell-secondary">{formatDate(caso.fecha_inicio)}</td>
              {:else}
                <td class="cell-radicado">CNS-{String(caso.id).padStart(3, '0')}</td>
                <td class="cell-name">{caso.cliente_display_name || '—'}</td>
                <td class="cell-secondary">{caso.tipo_proceso || '—'}</td>
                <td><Badge variant={getEstadoBadgeVariant(caso.estado)}>{caso.estado}</Badge></td>
                <td class="cell-secondary">{formatDate(caso.fecha_inicio)}</td>
              {/if}
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
    <div class="list-footer">
      <span class="count">{filtered().length} caso{filtered().length !== 1 ? 's' : ''}</span>
    </div>
  {/if}
</div>

<style>
  .casos-list { display: flex; flex-direction: column; gap: var(--sp-4); }
  .master-tabs { display: flex; gap: var(--sp-6); border-bottom: 1px solid var(--border-subtle); margin-bottom: var(--sp-1); }
  .master-tab {
    background: none; border: none; padding: var(--sp-2) 0;
    font-size: 0.9375rem; font-weight: 500; color: var(--text-muted);
    cursor: pointer; border-bottom: 2px solid transparent; transition: all var(--ease-fast);
  }
  .master-tab:hover { color: var(--text-primary); }
  .master-tab.active { color: var(--accent-blue); border-bottom-color: var(--accent-blue); font-weight: 600; }
  
  .list-toolbar { display: flex; align-items: center; gap: var(--sp-3); }
  .search-box {
    flex: 1; display: flex; align-items: center; gap: var(--sp-2);
    padding: var(--sp-2) var(--sp-3); background: var(--bg-surface);
    border: 1px solid var(--border-default); border-radius: var(--radius-md);
    transition: border-color var(--ease-fast);
  }
  .search-box:focus-within { border-color: var(--accent-blue); }
  .search-box svg { color: var(--text-muted); flex-shrink: 0; }
  .search-input {
    flex: 1; background: none; border: none; outline: none;
    color: var(--text-primary); font-family: var(--font-sans); font-size: 0.875rem;
  }
  .search-input::placeholder { color: var(--text-muted); }
  .btn-new {
    display: flex; align-items: center; gap: var(--sp-2);
    padding: var(--sp-2) var(--sp-4); background: var(--accent-blue);
    color: #fff; border: none; border-radius: var(--radius-md);
    font-family: var(--font-sans); font-size: 0.875rem; font-weight: 500;
    cursor: pointer; transition: background var(--ease-fast); white-space: nowrap;
  }
  .btn-new:hover { background: var(--accent-blue-hover); }
  .filter-bar { display: flex; gap: var(--sp-1); }
  .filter-btn {
    padding: var(--sp-1) var(--sp-3); background: none;
    border: 1px solid var(--border-default); border-radius: var(--radius-full);
    color: var(--text-secondary); font-family: var(--font-sans); font-size: 0.75rem;
    cursor: pointer; transition: all var(--ease-fast);
  }
  .filter-btn:hover { border-color: var(--accent-blue); color: var(--text-primary); }
  .filter-btn.active {
    background: var(--accent-blue-soft); border-color: var(--accent-blue);
    color: var(--accent-blue); font-weight: 500;
  }
  .table-container {
    background: var(--bg-surface); border: 1px solid var(--border-default);
    border-radius: var(--radius-lg); overflow: hidden;
  }
  .data-table { width: 100%; border-collapse: collapse; }
  .data-table thead { background: var(--bg-elevated); }
  .data-table th {
    padding: var(--sp-3) var(--sp-4); text-align: left;
    font-size: 0.75rem; font-weight: 600; color: var(--text-secondary);
    text-transform: uppercase; letter-spacing: 0.05em;
    border-bottom: 1px solid var(--border-default);
  }
  .data-table td {
    padding: var(--sp-3) var(--sp-4); font-size: 0.875rem;
    border-bottom: 1px solid var(--border-subtle);
  }
  .table-row { cursor: pointer; transition: background var(--ease-fast); }
  .table-row:hover { background: var(--bg-hover); }
  .table-row:last-child td { border-bottom: none; }
  .cell-radicado { font-family: var(--font-mono); font-size: 0.8125rem; font-weight: 500; color: var(--accent-blue); }
  .cell-name { font-weight: 500; color: var(--text-primary); }
  .cell-secondary { color: var(--text-secondary); }
  .list-footer { display: flex; justify-content: flex-end; padding: 0 var(--sp-2); }
  .count { font-size: 0.75rem; color: var(--text-muted); }
</style>
