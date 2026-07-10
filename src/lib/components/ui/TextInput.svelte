<script>
  let {
    label = '',
    value = $bindable(''),
    placeholder = '',
    type = 'text',
    required = false,
    error = '',
    id = undefined
  } = $props();

  let inputId = $derived(id || `input-${label.toLowerCase().replace(/\s+/g, '-')}`);
</script>

<div class="text-input-container" class:has-error={!!error}>
  {#if label}
    <label class="text-input-label" for={inputId}>
      {label}
      {#if required}<span class="required-mark">*</span>{/if}
    </label>
  {/if}
  <input
    {type}
    id={inputId}
    class="text-input"
    {placeholder}
    {required}
    bind:value
  />
  {#if error}
    <span class="text-input-error">{error}</span>
  {/if}
</div>

<style>
  .text-input-container {
    display: flex;
    flex-direction: column;
    gap: var(--sp-1);
  }

  .text-input-label {
    font-size: 0.8125rem;
    font-weight: 500;
    color: var(--text-secondary);
  }

  .required-mark {
    color: var(--accent-red);
    margin-left: 2px;
  }

  .text-input {
    width: 100%;
    padding: var(--sp-2) var(--sp-3);
    background: var(--bg-input);
    border: 1px solid var(--border-default);
    border-radius: var(--radius-md);
    color: var(--text-primary);
    font-family: var(--font-sans);
    font-size: 0.875rem;
    line-height: 1.5;
    transition: all var(--ease-fast);
    outline: none;
  }

  .text-input::placeholder {
    color: var(--text-muted);
  }

  .text-input:focus {
    border-color: var(--accent-blue);
    box-shadow: var(--shadow-glow-blue);
  }

  .has-error .text-input {
    border-color: var(--accent-red);
  }

  .has-error .text-input:focus {
    box-shadow: 0 0 0 3px rgba(239, 68, 68, 0.3);
  }

  .text-input-error {
    font-size: 0.75rem;
    color: var(--accent-red);
  }
</style>
