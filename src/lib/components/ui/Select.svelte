<script>
  let {
    label = '',
    value = $bindable(''),
    options = [],
    placeholder = 'Seleccionar...',
    required = false,
    error = '',
    id = ''
  } = $props();
</script>

<div class="select-group">
  {#if label}
    <label class="select-label" for={id}>
      {label}
      {#if required}
        <span class="required-mark">*</span>
      {/if}
    </label>
  {/if}

  <div class="select-wrapper">
    <select
      {id}
      class="select-input"
      class:has-error={!!error}
      bind:value
      {required}
    >
      {#if placeholder}
        <option value="" disabled>{placeholder}</option>
      {/if}
      {#each options as opt}
        <option value={opt.value}>{opt.label}</option>
      {/each}
    </select>
    <div class="select-chevron">
      <svg width="12" height="12" viewBox="0 0 12 12" fill="none">
        <path d="M2.5 4.5L6 8L9.5 4.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
      </svg>
    </div>
  </div>

  {#if error}
    <p class="select-error">{error}</p>
  {/if}
</div>

<style>
  .select-group {
    display: flex;
    flex-direction: column;
    gap: 0.375rem;
  }

  .select-label {
    font-size: 0.8125rem;
    font-weight: 500;
    color: var(--text-secondary, rgba(255, 255, 255, 0.7));
    letter-spacing: 0.01em;
  }

  .required-mark {
    color: var(--accent-red, #f87171);
    margin-left: 2px;
  }

  .select-wrapper {
    position: relative;
    display: flex;
    align-items: center;
  }

  .select-input {
    width: 100%;
    padding: 0.625rem 2.25rem 0.625rem 0.875rem;
    background: var(--bg-input, rgba(255, 255, 255, 0.04));
    border: 1px solid var(--border-default, rgba(255, 255, 255, 0.08));
    border-radius: 8px;
    color: var(--text-primary, #ffffff);
    font-size: 0.875rem;
    font-family: inherit;
    appearance: none;
    -webkit-appearance: none;
    cursor: pointer;
    transition: border-color 0.2s ease, box-shadow 0.2s ease;
  }

  .select-input:hover {
    border-color: var(--border-hover, rgba(255, 255, 255, 0.15));
  }

  .select-input:focus {
    outline: none;
    border-color: var(--accent-blue, #60a5fa);
    box-shadow: 0 0 0 3px rgba(96, 165, 250, 0.15);
  }

  .select-input.has-error {
    border-color: var(--accent-red, #f87171);
  }

  .select-input.has-error:focus {
    box-shadow: 0 0 0 3px rgba(248, 113, 113, 0.15);
  }

  .select-input option {
    background: var(--bg-card, #1a1a2e);
    color: var(--text-primary, #ffffff);
  }

  .select-chevron {
    position: absolute;
    right: 0.75rem;
    pointer-events: none;
    color: var(--text-muted, rgba(255, 255, 255, 0.45));
    display: flex;
    align-items: center;
  }

  .select-error {
    font-size: 0.75rem;
    color: var(--accent-red, #f87171);
    margin: 0;
  }
</style>
