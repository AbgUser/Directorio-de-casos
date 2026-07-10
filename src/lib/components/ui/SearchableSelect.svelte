<script>
  import { onMount } from 'svelte';

  let {
    label,
    id,
    options = [],
    value = $bindable(),
    placeholder = 'Buscar...',
    required = false
  } = $props();

  let search = $state('');
  let open = $state(false);
  let wrapperNode = $state(null);

  // Derivar la etiqueta de la opción seleccionada actualmente
  const selectedLabel = $derived(
    options.find(opt => String(opt.value) === String(value))?.label || ''
  );

  // Opciones filtradas en tiempo real
  const filteredOptions = $derived(
    options.filter(opt =>
      opt.label.toLowerCase().includes(search.toLowerCase()) || 
      (opt.description && opt.description.toLowerCase().includes(search.toLowerCase()))
    )
  );

  // Sincronizar el texto de búsqueda con el valor seleccionado cuando se cierra
  $effect(() => {
    if (!open && value) {
      search = selectedLabel;
    } else if (!open && !value) {
      search = '';
    }
  });

  function selectOption(opt) {
    value = String(opt.value);
    search = opt.label;
    open = false;
  }

  function handleInput(e) {
    search = e.target.value;
    if (!open) open = true;
    // Si el texto queda vacío, borrar el valor
    if (!search) {
      value = '';
    }
  }

  function handleClickOutside(event) {
    if (wrapperNode && !wrapperNode.contains(event.target)) {
      open = false;
      // Revertir la búsqueda al valor seleccionado si no completó la acción
      if (value) {
        search = selectedLabel;
      } else {
        search = '';
      }
    }
  }

  onMount(() => {
    document.addEventListener('click', handleClickOutside);
    return () => {
      document.removeEventListener('click', handleClickOutside);
    };
  });
</script>

<div class="searchable-select" bind:this={wrapperNode}>
  {#if label}
    <label class="field-label" for={id}>
      {label}
      {#if required}<span class="req">*</span>{/if}
    </label>
  {/if}
  
  <div class="input-wrapper">
    <input
      type="text"
      {id}
      class="field-input"
      {placeholder}
      bind:value={search}
      oninput={handleInput}
      onclick={() => open = true}
      onfocus={() => open = true}
      autocomplete="off"
    />
    <!-- Hidden input to enforce required validation if needed -->
    {#if required}
      <input type="text" style="display:none" bind:value={value} required />
    {/if}
    
    <div class="icon" class:open>
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <polyline points="6 9 12 15 18 9"></polyline>
      </svg>
    </div>
  </div>

  {#if open}
    <div class="dropdown">
      {#if filteredOptions.length === 0}
        <div class="empty">No se encontraron resultados</div>
      {:else}
        {#each filteredOptions as opt}
          <div 
            class="option" 
            class:selected={String(value) === String(opt.value)}
            onclick={() => selectOption(opt)}
          >
            {opt.label}
          </div>
        {/each}
      {/if}
    </div>
  {/if}
</div>

<style>
  .searchable-select {
    display: flex;
    flex-direction: column;
    gap: var(--sp-1);
    position: relative;
  }
  .field-label {
    font-size: 0.8125rem;
    font-weight: 500;
    color: var(--text-secondary);
  }
  .req {
    color: var(--accent-red);
    margin-left: 2px;
  }
  .input-wrapper {
    position: relative;
  }
  .field-input {
    width: 100%;
    padding: var(--sp-3);
    padding-right: var(--sp-8);
    background: var(--bg-input);
    border: 1px solid var(--border-default);
    border-radius: var(--radius-md);
    color: var(--text-primary);
    font-family: var(--font-sans);
    font-size: 0.875rem;
    outline: none;
    transition: all var(--ease-fast);
  }
  .field-input:focus {
    border-color: var(--accent-blue);
    box-shadow: var(--shadow-glow-blue);
  }
  .icon {
    position: absolute;
    right: 12px;
    top: 50%;
    transform: translateY(-50%);
    color: var(--text-muted);
    pointer-events: none;
    transition: transform var(--ease-fast);
  }
  .icon.open {
    transform: translateY(-50%) rotate(180deg);
  }
  .dropdown {
    position: absolute;
    top: calc(100% + 4px);
    left: 0;
    right: 0;
    max-height: 200px;
    overflow-y: auto;
    background: var(--bg-elevated);
    border: 1px solid var(--border-default);
    border-radius: var(--radius-md);
    box-shadow: var(--shadow-lg);
    z-index: 100;
  }
  .option {
    padding: var(--sp-2) var(--sp-3);
    font-size: 0.875rem;
    color: var(--text-primary);
    cursor: pointer;
    transition: background var(--ease-fast);
  }
  .option:hover {
    background: var(--bg-hover);
  }
  .option.selected {
    background: var(--accent-blue-transparent);
    color: var(--accent-blue);
  }
  .empty {
    padding: var(--sp-3);
    font-size: 0.875rem;
    color: var(--text-muted);
    text-align: center;
  }
</style>
