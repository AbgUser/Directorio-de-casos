<script>
  let { tabs = [], activeTab = $bindable('') } = $props();
</script>

<div class="tabs-container" role="tablist">
  {#each tabs as tab}
    <button
      class="tab-button"
      class:active={activeTab === tab.id}
      role="tab"
      aria-selected={activeTab === tab.id}
      id="tab-{tab.id}"
      onclick={() => { activeTab = tab.id; }}
    >
      {tab.label}
    </button>
  {/each}
</div>

<style>
  .tabs-container {
    display: flex;
    gap: 0;
    border-bottom: 1px solid var(--border-subtle, rgba(255, 255, 255, 0.06));
    margin-bottom: 1.5rem;
    overflow-x: auto;
    scrollbar-width: none;
  }

  .tabs-container::-webkit-scrollbar {
    display: none;
  }

  .tab-button {
    position: relative;
    padding: 0.75rem 1.25rem;
    background: none;
    border: none;
    border-bottom: 2px solid transparent;
    color: var(--text-muted, rgba(255, 255, 255, 0.45));
    font-size: 0.875rem;
    font-weight: 500;
    cursor: pointer;
    white-space: nowrap;
    transition: color 0.2s ease, border-color 0.2s ease;
    font-family: inherit;
    letter-spacing: 0.01em;
  }

  .tab-button:hover {
    color: var(--text-secondary, rgba(255, 255, 255, 0.7));
  }

  .tab-button.active {
    color: var(--accent-blue, #60a5fa);
    border-bottom-color: var(--accent-blue, #60a5fa);
  }

  .tab-button:focus-visible {
    outline: 2px solid var(--accent-blue, #60a5fa);
    outline-offset: -2px;
    border-radius: 4px 4px 0 0;
  }
</style>
