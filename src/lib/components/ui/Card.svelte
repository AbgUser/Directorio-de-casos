<script>
  let { padding = 'md', children } = $props();
</script>

<div class="card card-pad-{padding}">
  {@render children()}
</div>

<style>
  .card {
    background: var(--bg-surface);
    border: 1px solid var(--border-default);
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-sm);
    transition: box-shadow var(--ease-base);
  }

  .card:hover {
    box-shadow: var(--shadow-md);
  }

  .card-pad-sm {
    padding: var(--sp-3);
  }

  .card-pad-md {
    padding: var(--sp-5);
  }

  .card-pad-lg {
    padding: var(--sp-8);
  }

  .card-pad-none {
    padding: 0;
  }
</style>
