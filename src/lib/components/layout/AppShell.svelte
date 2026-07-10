<script>
  import Sidebar from './Sidebar.svelte';
  import Topbar from './Topbar.svelte';

  let { title = 'Inicio', children } = $props();
</script>

<div class="app-shell">
  <Sidebar />
  <Topbar {title} />
  <main class="main-content">
    {@render children()}
  </main>
</div>

<style>
  .app-shell {
    height: 100vh;
    width: 100vw;
    overflow: hidden;
  }

  .main-content {
    margin-left: var(--sidebar-width);
    margin-top: var(--topbar-height);
    padding: var(--sp-8);
    height: calc(100vh - var(--topbar-height));
    overflow-y: auto;
  }
</style>
