<script>
  let {
    message = '',
    type = 'info',
    visible = $bindable(false)
  } = $props();

  let timeout;

  $effect(() => {
    if (visible) {
      clearTimeout(timeout);
      timeout = setTimeout(() => {
        visible = false;
      }, 3000);
    }
    return () => clearTimeout(timeout);
  });

  const icons = {
    success: `<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/><polyline points="22 4 12 14.01 9 11.01"/></svg>`,
    error: `<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><line x1="15" y1="9" x2="9" y2="15"/><line x1="9" y1="9" x2="15" y2="15"/></svg>`,
    warning: `<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"/><line x1="12" y1="9" x2="12" y2="13"/><line x1="12" y1="17" x2="12.01" y2="17"/></svg>`,
    info: `<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><line x1="12" y1="16" x2="12" y2="12"/><line x1="12" y1="8" x2="12.01" y2="8"/></svg>`
  };
</script>

{#if visible}
  <div class="toast toast-{type}" role="alert">
    <span class="toast-icon">{@html icons[type]}</span>
    <span class="toast-message">{message}</span>
    <button class="toast-close" onclick={() => visible = false} aria-label="Cerrar">
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <line x1="18" y1="6" x2="6" y2="18"/>
        <line x1="6" y1="6" x2="18" y2="18"/>
      </svg>
    </button>
  </div>
{/if}

<style>
  .toast {
    position: fixed;
    bottom: var(--sp-6);
    right: var(--sp-6);
    display: flex;
    align-items: center;
    gap: var(--sp-3);
    padding: var(--sp-3) var(--sp-4);
    border-radius: var(--radius-lg);
    border: 1px solid;
    box-shadow: var(--shadow-lg);
    z-index: 2000;
    animation: toastSlideIn var(--ease-base) forwards;
    max-width: 400px;
  }

  @keyframes toastSlideIn {
    from {
      opacity: 0;
      transform: translateY(16px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  .toast-success {
    background: var(--bg-elevated);
    border-color: var(--accent-green);
    color: var(--accent-green);
  }

  .toast-error {
    background: var(--bg-elevated);
    border-color: var(--accent-red);
    color: var(--accent-red);
  }

  .toast-warning {
    background: var(--bg-elevated);
    border-color: var(--accent-amber);
    color: var(--accent-amber);
  }

  .toast-info {
    background: var(--bg-elevated);
    border-color: var(--accent-blue);
    color: var(--accent-blue);
  }

  .toast-icon {
    display: flex;
    align-items: center;
    flex-shrink: 0;
  }

  .toast-message {
    font-size: 0.8125rem;
    font-weight: 500;
    color: var(--text-primary);
    flex: 1;
  }

  .toast-close {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 24px;
    height: 24px;
    background: transparent;
    border: none;
    border-radius: var(--radius-sm);
    color: var(--text-muted);
    cursor: pointer;
    flex-shrink: 0;
    transition: color var(--ease-fast);
  }

  .toast-close:hover {
    color: var(--text-primary);
  }
</style>
