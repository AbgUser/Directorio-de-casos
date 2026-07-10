<script>
  let {
    icon = '',
    title = '',
    message = '',
    children
  } = $props();
</script>

<div class="empty-state">
  {#if icon}
    <div class="empty-icon">
      {@html icon}
    </div>
  {/if}

  {#if title}
    <h3 class="empty-title">{title}</h3>
  {/if}

  {#if message}
    <p class="empty-message">{message}</p>
  {/if}

  {#if children}
    <div class="empty-actions">
      {@render children()}
    </div>
  {/if}
</div>

<style>
  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 3rem 2rem;
    text-align: center;
    min-height: 240px;
  }

  .empty-icon {
    width: 56px;
    height: 56px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 16px;
    background: var(--bg-elevated, rgba(255, 255, 255, 0.04));
    color: var(--text-muted, rgba(255, 255, 255, 0.45));
    margin-bottom: 1.25rem;
  }

  .empty-icon :global(svg) {
    width: 28px;
    height: 28px;
    opacity: 0.6;
  }

  .empty-title {
    font-size: 1rem;
    font-weight: 600;
    color: var(--text-secondary, rgba(255, 255, 255, 0.7));
    margin: 0 0 0.5rem 0;
    letter-spacing: -0.01em;
  }

  .empty-message {
    font-size: 0.875rem;
    color: var(--text-muted, rgba(255, 255, 255, 0.45));
    margin: 0 0 1.5rem 0;
    max-width: 320px;
    line-height: 1.5;
  }

  .empty-actions {
    display: flex;
    gap: 0.75rem;
    align-items: center;
  }
</style>
