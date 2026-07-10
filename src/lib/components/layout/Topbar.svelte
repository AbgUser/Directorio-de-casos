<script>
  import { page } from '$app/stores';
  import { globalSearch } from '$lib/services/searchService.js';

  let { title = 'Inicio' } = $props();

  let searchQuery = $state('');
  let searchResults = $state({ clientes: [], casos: [] });
  let showDropdown = $state(false);
  let searchTimeout = null;

  let isDashboard = $derived($page.url.pathname === '/');

  function handleInput(e) {
    searchQuery = e.target.value;
    if (searchTimeout) clearTimeout(searchTimeout);

    if (searchQuery.trim().length < 2) {
      searchResults = { clientes: [], casos: [] };
      showDropdown = false;
      return;
    }

    searchTimeout = setTimeout(async () => {
      try {
        searchResults = await globalSearch(searchQuery);
        showDropdown = searchResults.clientes.length > 0 || searchResults.casos.length > 0;
      } catch (err) {
        console.error(err);
      }
    }, 300);
  }

  function handleBlur() {
    setTimeout(() => {
      showDropdown = false;
    }, 200);
  }
</script>
<header class="topbar">
  <div class="topbar-left">
    {#if title === 'Inicio'}
      <svg class="title-icon" width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="3" width="7" height="7" rx="1"/><rect x="14" y="3" width="7" height="7" rx="1"/><rect x="3" y="14" width="7" height="7" rx="1"/><rect x="14" y="14" width="7" height="7" rx="1"/></svg>
    {:else if title === 'Directorio de Clientes'}
      <svg class="title-icon" width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2"/><circle cx="9" cy="7" r="4"/><path d="M22 21v-2a4 4 0 0 0-3-3.87"/><path d="M16 3.13a4 4 0 0 1 0 7.75"/></svg>
    {:else if title === 'Directorio General de Casos'}
      <svg class="title-icon" width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/></svg>
    {:else if title === 'Calendario'}
      <svg class="title-icon" width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="4" width="18" height="18" rx="2" ry="2"/><line x1="16" y1="2" x2="16" y2="6"/><line x1="8" y1="2" x2="8" y2="6"/><line x1="3" y1="10" x2="21" y2="10"/></svg>
    {:else if title === 'Configuración del Despacho'}
      <svg class="title-icon" width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="3"/><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 2.83-2.83l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z"/></svg>
    {/if}
    <h1 class="topbar-title">{title}</h1>
  </div>

  <div class="topbar-right">
    {#if isDashboard}
      <div class="search-container">
        <svg class="search-icon" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <circle cx="11" cy="11" r="8"/>
          <line x1="21" y1="21" x2="16.65" y2="16.65"/>
        </svg>
        <input
          type="text"
          class="search-input"
          placeholder="Buscar clientes, casos, radicados..."
          value={searchQuery}
          oninput={handleInput}
          onblur={handleBlur}
          onfocus={() => { if (searchQuery.trim().length >= 2) showDropdown = true; }}
        />
        {#if showDropdown}
          <div class="search-dropdown">
            {#if searchResults.clientes.length > 0}
              <div class="dropdown-section">
                <div class="dropdown-header">Clientes</div>
                {#each searchResults.clientes as cliente}
                  <a href="/clientes/{cliente.id}" class="dropdown-item">
                    <span class="item-title">{cliente.nombre_completo}</span>
                    <span class="item-subtitle">ID: {cliente.identificacion || 'N/A'}</span>
                  </a>
                {/each}
              </div>
            {/if}
            {#if searchResults.casos.length > 0}
              <div class="dropdown-section">
                <div class="dropdown-header">Casos</div>
                {#each searchResults.casos as caso}
                  <a href="/casos/{caso.id}" class="dropdown-item">
                    <span class="item-title">{caso.radicado || caso.slug}</span>
                    <span class="item-subtitle">{caso.cliente_nombre || 'Sin cliente'} - {caso.tipo_proceso}</span>
                  </a>
                {/each}
              </div>
            {/if}
          </div>
        {/if}
      </div>
    {/if}

    <button class="notification-btn" aria-label="Notificaciones">
      <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <path d="M18 8A6 6 0 0 0 6 8c0 7-3 9-3 9h18s-3-2-3-9"/>
        <path d="M13.73 21a2 2 0 0 1-3.46 0"/>
      </svg>
      <span class="notification-dot"></span>
    </button>
  </div>
</header>

<style>
  .topbar {
    position: fixed;
    top: 0;
    left: var(--sidebar-width);
    right: 0;
    height: var(--topbar-height);
    background: var(--bg-primary);
    border-bottom: 1px solid var(--border-subtle);
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 var(--sp-8);
    z-index: 90;
  }

  .topbar-left {
    display: flex;
    align-items: center;
    gap: var(--sp-3);
  }

  .title-icon {
    color: var(--text-secondary);
  }

  .topbar-title {
    font-size: 1.125rem;
    font-weight: 600;
    color: var(--text-primary);
    letter-spacing: -0.01em;
  }

  .topbar-right {
    display: flex;
    align-items: center;
    gap: var(--sp-4);
  }

  .search-container {
    position: relative;
    display: flex;
    align-items: center;
  }

  .search-icon {
    position: absolute;
    left: var(--sp-3);
    color: var(--text-muted);
    pointer-events: none;
  }

  .search-input {
    width: 320px;
    padding: var(--sp-2) var(--sp-3) var(--sp-2) var(--sp-10);
    background: var(--bg-input);
    border: 1px solid var(--border-default);
    border-radius: var(--radius-full);
    color: var(--text-primary);
    font-family: var(--font-sans);
    font-size: 0.8125rem;
    transition: all var(--ease-fast);
    outline: none;
  }

  .search-input::placeholder {
    color: var(--text-muted);
  }

  .search-input:focus {
    border-color: var(--accent-blue);
    box-shadow: var(--shadow-glow-blue);
  }

  .notification-btn {
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 36px;
    height: 36px;
    background: transparent;
    border: 1px solid transparent;
    border-radius: var(--radius-md);
    color: var(--text-secondary);
    cursor: pointer;
    transition: all var(--ease-fast);
  }

  .notification-btn:hover {
    background: var(--bg-hover);
    color: var(--text-primary);
  }

  .notification-dot {
    position: absolute;
    top: 6px;
    right: 6px;
    width: 8px;
    height: 8px;
    background: var(--accent-red);
    border-radius: 50%;
    border: 2px solid var(--bg-primary);
  }

  .search-dropdown {
    position: absolute;
    top: calc(100% + var(--sp-2));
    left: 0;
    right: 0;
    background: var(--bg-primary);
    border: 1px solid var(--border-default);
    border-radius: var(--radius-md);
    box-shadow: var(--shadow-lg);
    max-height: 400px;
    overflow-y: auto;
    z-index: 100;
  }

  .dropdown-section {
    padding: var(--sp-2) 0;
  }

  .dropdown-section + .dropdown-section {
    border-top: 1px solid var(--border-subtle);
  }

  .dropdown-header {
    padding: var(--sp-1) var(--sp-4);
    font-size: 0.6875rem;
    font-weight: 600;
    text-transform: uppercase;
    color: var(--text-muted);
    letter-spacing: 0.05em;
  }

  .dropdown-item {
    display: flex;
    flex-direction: column;
    padding: var(--sp-2) var(--sp-4);
    text-decoration: none;
    color: var(--text-primary);
    transition: background var(--ease-fast);
  }

  .dropdown-item:hover {
    background: var(--bg-hover);
  }

  .item-title {
    font-size: 0.8125rem;
    font-weight: 500;
  }

  .item-subtitle {
    font-size: 0.75rem;
    color: var(--text-secondary);
    margin-top: 2px;
  }
</style>
