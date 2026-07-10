<script>
  let { variant = 'default', children } = $props();
</script>

<span class="badge badge-{variant}">
  {@render children()}
</span>

<style>
  .badge {
    display: inline-flex;
    align-items: center;
    font-size: 0.75rem;
    font-weight: 500;
    padding: 2px 8px;
    border-radius: var(--radius-full);
    white-space: nowrap;
    line-height: 1.4;
  }

  .badge-default {
    background: var(--bg-hover);
    color: var(--text-secondary);
  }

  .badge-blue {
    background: var(--accent-blue-soft);
    color: var(--accent-blue);
  }

  .badge-teal {
    background: var(--accent-teal-soft);
    color: var(--accent-teal);
  }

  .badge-amber {
    background: var(--accent-amber-soft);
    color: var(--accent-amber);
  }

  .badge-red {
    background: var(--accent-red-soft);
    color: var(--accent-red);
  }

  .badge-purple {
    background: var(--accent-purple-soft);
    color: var(--accent-purple);
  }

  .badge-green {
    background: var(--accent-green-soft);
    color: var(--accent-green);
  }
</style>
