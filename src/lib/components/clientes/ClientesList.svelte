<script>
  import Badge from '$lib/components/ui/Badge.svelte';
  import EmptyState from '$lib/components/ui/EmptyState.svelte';

  let {
    clientes = [],
    onnewclient = () => {},
    onselect = () => {}
  } = $props();

  let searchQuery = $state('');

  let filtered = $derived(
    searchQuery.trim()
      ? clientes.filter(c => {
          const q = searchQuery.toLowerCase();
          const name = (c.nombre_completo || '').toLowerCase();
          const doc = (c.identificacion || '').toLowerCase();
          const address = (c.direccion || '').toLowerCase();
          const mail = (c.email || '').toLowerCase();
          return name.includes(q) || doc.includes(q) || address.includes(q) || mail.includes(q);
        })
      : clientes
  );

  function displayName(c) {
    return c.nombre_completo || '—';
  }

  function displayDoc(c) {
    return c.identificacion ? `${c.tipo_identificacion} ${c.identificacion}` : '—';
  }
</script>

<div class="clientes-list fade-in">
  <div class="list-toolbar">
    <div class="search-box">
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/></svg>
      <input
        type="text"
        class="search-input"
        placeholder="Buscar por nombre, documento, ciudad..."
        bind:value={searchQuery}
      />
    </div>
    <button class="btn-new" onclick={onnewclient}>
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/></svg>
      Nuevo Cliente
    </button>
  </div>

  {#if filtered.length === 0}
    <EmptyState
      title={searchQuery ? 'Sin resultados' : 'No hay clientes registrados'}
      message={searchQuery ? 'Intente con otros términos de búsqueda.' : 'Registre su primer cliente para comenzar a gestionar sus casos.'}
    />
  {:else}
    <div class="table-container">
      <table class="data-table">
        <thead>
          <tr>
            <th>Nombre / Razón Social</th>
            <th>Tipo</th>
            <th>Identificación</th>
            <th>Dirección</th>
            <th>Teléfono</th>
            <th>Email</th>
          </tr>
        </thead>
        <tbody>
          {#each filtered as cliente}
            <tr class="table-row" onclick={() => onselect(cliente.id)}>
              <td class="cell-name">{displayName(cliente)}</td>
              <td>
                <Badge variant={cliente.tipo_persona === 'natural' ? 'blue' : 'purple'}>
                  {cliente.tipo_persona === 'natural' ? 'Natural' : 'Jurídica'}
                </Badge>
              </td>
              <td class="cell-mono">{displayDoc(cliente)}</td>
              <td class="cell-secondary">{cliente.direccion || '—'}</td>
              <td class="cell-secondary">{cliente.telefono || '—'}</td>
              <td class="cell-secondary">{cliente.email || '—'}</td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
    <div class="list-footer">
      <span class="count">{filtered.length} cliente{filtered.length !== 1 ? 's' : ''}</span>
    </div>
  {/if}
</div>

<style>
  .clientes-list { display: flex; flex-direction: column; gap: var(--sp-4); }
  .list-toolbar {
    display: flex; align-items: center; gap: var(--sp-3);
  }
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
  .cell-name { font-weight: 500; color: var(--text-primary); }
  .cell-mono { font-family: var(--font-mono); font-size: 0.8125rem; color: var(--text-secondary); }
  .cell-secondary { color: var(--text-secondary); }
  .list-footer {
    display: flex; justify-content: flex-end;
    padding: 0 var(--sp-2);
  }
  .count { font-size: 0.75rem; color: var(--text-muted); }
</style>
