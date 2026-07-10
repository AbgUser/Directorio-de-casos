<script>
  let {
    variant = 'primary',
    size = 'md',
    disabled = false,
    type = 'button',
    onclick = undefined,
    children
  } = $props();
</script>

<button
  class="btn btn-{variant} btn-{size}"
  {type}
  {disabled}
  {onclick}
>
  {@render children()}
</button>

<style>
  .btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: var(--sp-2);
    font-family: var(--font-sans);
    font-weight: 500;
    border: 1px solid transparent;
    border-radius: var(--radius-md);
    cursor: pointer;
    transition: all var(--ease-fast);
    white-space: nowrap;
    line-height: 1;
  }

  .btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  /* ── Variants ── */
  .btn-primary {
    background: var(--accent-blue);
    color: #fff;
    border-color: var(--accent-blue);
  }
  .btn-primary:hover:not(:disabled) {
    background: var(--accent-blue-hover);
    border-color: var(--accent-blue-hover);
  }
  .btn-primary:active:not(:disabled) {
    transform: scale(0.98);
  }

  .btn-secondary {
    background: var(--bg-surface);
    color: var(--text-primary);
    border-color: var(--border-default);
  }
  .btn-secondary:hover:not(:disabled) {
    background: var(--bg-hover);
  }

  .btn-ghost {
    background: transparent;
    color: var(--text-secondary);
    border-color: transparent;
  }
  .btn-ghost:hover:not(:disabled) {
    background: var(--bg-hover);
    color: var(--text-primary);
  }

  .btn-danger {
    background: var(--accent-red);
    color: #fff;
    border-color: var(--accent-red);
  }
  .btn-danger:hover:not(:disabled) {
    background: #DC2626;
    border-color: #DC2626;
  }

  /* ── Sizes ── */
  .btn-sm {
    padding: 4px 12px;
    font-size: 0.8125rem;
  }

  .btn-md {
    padding: 8px 16px;
    font-size: 0.875rem;
  }

  .btn-lg {
    padding: 12px 24px;
    font-size: 0.9375rem;
  }
</style>
