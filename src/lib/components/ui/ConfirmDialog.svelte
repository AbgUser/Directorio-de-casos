<script>
  let {
    open = false,
    title = 'Confirmar acción',
    message = '¿Está seguro de que desea continuar?',
    confirmLabel = 'Confirmar',
    cancelLabel = 'Cancelar',
    variant = 'danger',
    onconfirm = () => {},
    oncancel = () => {}
  } = $props();

  function handleBackdropClick(e) {
    if (e.target === e.currentTarget) {
      oncancel();
    }
  }

  function handleKeydown(e) {
    if (e.key === 'Escape') {
      oncancel();
    }
  }
</script>

{#if open}
  <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
  <div
    class="dialog-backdrop"
    role="dialog"
    aria-modal="true"
    aria-labelledby="confirm-dialog-title"
    onclick={handleBackdropClick}
    onkeydown={handleKeydown}
  >
    <div class="dialog-panel">
      <div class="dialog-header">
        <div class="dialog-icon {variant}">
          {#if variant === 'danger'}
            <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"/>
              <line x1="12" y1="9" x2="12" y2="13"/>
              <line x1="12" y1="17" x2="12.01" y2="17"/>
            </svg>
          {:else}
            <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <circle cx="12" cy="12" r="10"/>
              <line x1="12" y1="8" x2="12" y2="12"/>
              <line x1="12" y1="16" x2="12.01" y2="16"/>
            </svg>
          {/if}
        </div>
        <h3 class="dialog-title" id="confirm-dialog-title">{title}</h3>
      </div>

      <p class="dialog-message">{message}</p>

      <div class="dialog-actions">
        <button
          class="btn btn-ghost"
          onclick={oncancel}
        >
          {cancelLabel}
        </button>
        <button
          class="btn btn-{variant}"
          onclick={onconfirm}
        >
          {confirmLabel}
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .dialog-backdrop {
    position: fixed;
    inset: 0;
    z-index: 1000;
    display: flex;
    align-items: center;
    justify-content: center;
    background: rgba(0, 0, 0, 0.6);
    backdrop-filter: blur(4px);
    animation: fadeIn 0.15s ease-out;
  }

  .dialog-panel {
    background: var(--bg-card, #1a1a2e);
    border: 1px solid var(--border-default, rgba(255, 255, 255, 0.08));
    border-radius: 16px;
    padding: 1.75rem;
    max-width: 420px;
    width: calc(100% - 2rem);
    box-shadow: 0 24px 48px rgba(0, 0, 0, 0.4);
    animation: scaleIn 0.2s ease-out;
  }

  .dialog-header {
    display: flex;
    align-items: center;
    gap: 0.875rem;
    margin-bottom: 0.875rem;
  }

  .dialog-icon {
    width: 40px;
    height: 40px;
    border-radius: 12px;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
  }

  .dialog-icon.danger {
    background: rgba(248, 113, 113, 0.12);
    color: var(--accent-red, #f87171);
  }

  .dialog-icon.warning {
    background: rgba(251, 191, 36, 0.12);
    color: var(--accent-amber, #fbbf24);
  }

  .dialog-icon.info {
    background: rgba(96, 165, 250, 0.12);
    color: var(--accent-blue, #60a5fa);
  }

  .dialog-title {
    font-size: 1.0625rem;
    font-weight: 600;
    color: var(--text-primary, #ffffff);
    margin: 0;
    letter-spacing: -0.01em;
  }

  .dialog-message {
    font-size: 0.875rem;
    color: var(--text-muted, rgba(255, 255, 255, 0.45));
    line-height: 1.6;
    margin: 0 0 1.5rem 0;
    padding-left: calc(40px + 0.875rem);
  }

  .dialog-actions {
    display: flex;
    justify-content: flex-end;
    gap: 0.625rem;
  }

  .btn {
    padding: 0.5rem 1.125rem;
    border-radius: 8px;
    font-size: 0.8125rem;
    font-weight: 500;
    font-family: inherit;
    cursor: pointer;
    border: none;
    transition: all 0.2s ease;
    letter-spacing: 0.01em;
  }

  .btn-ghost {
    background: transparent;
    color: var(--text-secondary, rgba(255, 255, 255, 0.7));
    border: 1px solid var(--border-default, rgba(255, 255, 255, 0.08));
  }

  .btn-ghost:hover {
    background: rgba(255, 255, 255, 0.04);
    border-color: var(--border-hover, rgba(255, 255, 255, 0.15));
  }

  .btn-danger {
    background: var(--accent-red, #f87171);
    color: #fff;
  }

  .btn-danger:hover {
    background: #ef4444;
    box-shadow: 0 4px 12px rgba(248, 113, 113, 0.3);
  }

  .btn-warning {
    background: var(--accent-amber, #fbbf24);
    color: #1a1a2e;
  }

  .btn-warning:hover {
    background: #f59e0b;
  }

  .btn-info {
    background: var(--accent-blue, #60a5fa);
    color: #fff;
  }

  .btn-info:hover {
    background: #3b82f6;
  }

  .btn:focus-visible {
    outline: 2px solid var(--accent-blue, #60a5fa);
    outline-offset: 2px;
  }

  @keyframes fadeIn {
    from { opacity: 0; }
    to { opacity: 1; }
  }

  @keyframes scaleIn {
    from { opacity: 0; transform: scale(0.95); }
    to { opacity: 1; transform: scale(1); }
  }
</style>
