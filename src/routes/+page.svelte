<script>
  import Card from '$lib/components/ui/Card.svelte';
  import Badge from '$lib/components/ui/Badge.svelte';
  import Button from '$lib/components/ui/Button.svelte';
  import MiniCalendar from '$lib/components/dashboard/MiniCalendar.svelte';
  import { getDashboardStats, getAlertas } from '$lib/services/dashboardService.js';
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';

  let stats = $state([
    {
      label: 'Casos Activos',
      value: '0',
      accent: 'teal',
      accentColor: 'var(--accent-teal)',
      icon: `<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/></svg>`
    },
    {
      label: 'Casos Cerrados',
      value: '0',
      accent: 'default',
      accentColor: 'var(--text-secondary)',
      icon: `<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/><polyline points="22 4 12 14.01 9 11.01"/></svg>`
    },
    {
      label: 'Términos Pendientes',
      value: '0',
      accent: 'amber',
      accentColor: 'var(--accent-amber)',
      icon: `<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><polyline points="12 6 12 12 16 14"/></svg>`
    },
    {
      label: 'Clientes Registrados',
      value: '0',
      accent: 'purple',
      accentColor: 'var(--accent-purple)',
      icon: `<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2"/><circle cx="9" cy="7" r="4"/><path d="M22 21v-2a4 4 0 0 0-3-3.87"/><path d="M16 3.13a4 4 0 0 1 0 7.75"/></svg>`
    }
  ]);

  let isLoading = $state(true);
  let alertas = $state([]);
  let todosEventos = $state([]);
  let selectedDate = $state(null);

  // Computed array for display
  let displayedAlertas = $derived(
    selectedDate 
      ? todosEventos.filter(e => e.fecha_inicio === selectedDate) 
      : alertas
  );

  onMount(async () => {
    try {
      const data = await getDashboardStats();
      stats[0].value = data.casosActivos.toString();
      stats[1].value = data.casosCerrados.toString();
      stats[2].value = data.terminosPendientes.toString();
      stats[3].value = data.clientesTotales.toString();

      const res = await getAlertas();
      alertas = res.alertas;
      todosEventos = res.todosEventos;
    } catch (e) {
      console.error("Error loading dashboard stats:", e);
    } finally {
      isLoading = false;
    }
  });
  const getAlertaColor = (urgencia) => {
    if (urgencia === 'alta' || urgencia === 'vencido') return 'var(--accent-red)';
    if (urgencia === 'media') return 'var(--accent-amber)';
    return 'var(--accent-blue)';
  };

  async function solventarTermino(e, alerta) {
    e.preventDefault();
    e.stopPropagation();
    if (alerta.tipo !== 'vencimiento' || !alerta.actuacion_id) return;
    try {
      await invoke('marcar_termino_completado', { id: alerta.actuacion_id });
      alertas = alertas.filter(a => a !== alerta);
      todosEventos = todosEventos.filter(a => a !== alerta);
    } catch (err) {
      console.error(err);
      alert('Error al completar el término.');
    }
  }
</script>

<div class="dashboard fade-in">
  <!-- Stats Grid -->
  <div class="stats-grid">
    {#each stats as stat}
      <div class="stat-card" style="--stat-accent: {stat.accentColor}">
        <div class="stat-icon">
          {@html stat.icon}
        </div>
        <div class="stat-info">
          <span class="stat-label">{stat.label}</span>
          <span class="stat-value">{stat.value}</span>
        </div>
        <Badge variant={stat.accent}>
          {stat.accent === 'default' ? 'Archivados' : stat.accent === 'teal' ? 'Activos' : stat.accent === 'amber' ? 'Pendientes' : 'Total'}
        </Badge>
      </div>
    {/each}
  </div>

  <!-- Content Grid -->
  <div class="content-grid">
    <!-- Términos por Vencer -->
    <div class="content-main">
      <Card>
        <div class="section-header">
          <h3>
            {selectedDate 
              ? `Eventos del ${new Date(selectedDate + 'T00:00:00').toLocaleDateString('es-CO')}` 
              : 'Próximos Vencimientos y Audiencias'}
          </h3>
          {#if selectedDate}
            <Button variant="ghost" onclick={() => selectedDate = null} style="font-size:0.75rem; padding: 4px 8px;">
              Ver Alertas
            </Button>
          {:else}
            <Badge variant="amber">Alertas Proactivas</Badge>
          {/if}
        </div>
        
        {#if displayedAlertas.length === 0}
          <div class="empty-state">
            <svg width="40" height="40" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" class="empty-icon">
              <rect x="3" y="4" width="18" height="18" rx="2" ry="2"/>
              <line x1="16" y1="2" x2="16" y2="6"/>
              <line x1="8" y1="2" x2="8" y2="6"/>
              <line x1="3" y1="10" x2="21" y2="10"/>
            </svg>
            <p class="empty-text">
              {selectedDate ? 'No hay eventos en esta fecha' : 'No hay alertas inminentes'}
            </p>
            {#if !selectedDate}
              <p class="empty-subtext">No tienes términos a vencer en 3 días hábiles ni audiencias en 5 días.</p>
            {/if}
          </div>
        {:else}
          <div class="alertas-list">
            {#each displayedAlertas as alerta}
              <a href="/casos/{alerta.caso_id}" class="alerta-card" style="border-left-color: {getAlertaColor(alerta.urgencia)}">
                <div class="alerta-header">
                  <strong>{alerta.titulo}</strong>
                  <div style="display: flex; gap: var(--sp-2); align-items: center;">
                    {#if alerta.urgencia}
                      <Badge variant={alerta.urgencia === 'alta' || alerta.urgencia === 'vencido' ? 'red' : 'amber'}>
                        {alerta.urgencia === 'vencido' ? 'VENCIDO' : (alerta.dias + (alerta.tipo === 'termino' ? ' días hábiles' : ' días'))}
                      </Badge>
                    {:else}
                      <Badge variant={alerta.tipo === 'audiencia' ? 'amber' : 'teal'}>
                        {alerta.tipo.toUpperCase()}
                      </Badge>
                    {/if}
                    {#if alerta.tipo === 'vencimiento' && alerta.actuacion_id}
                      <button class="solventar-btn" title="Marcar como solventado" onclick={(e) => solventarTermino(e, alerta)}>
                        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="20 6 9 17 4 12"/></svg>
                      </button>
                    {/if}
                  </div>
                </div>
                <div class="alerta-meta">
                  <span>Radicado: {alerta.radicado || 'Sin radicado'}</span>
                  <span>|</span>
                  <span>{alerta.cliente_display_name || 'Sin cliente'}</span>
                </div>
                <div class="alerta-tipo">
                  {alerta.tipo.toUpperCase()} - {new Date(alerta.fecha_inicio + 'T00:00:00').toLocaleDateString('es-CO')}
                  {#if alerta.hora_inicio}
                    a las {alerta.hora_inicio}
                  {/if}
                </div>
              </a>
            {/each}
          </div>
        {/if}
      </Card>
    </div>

    <!-- Accesos Rápidos y Calendario -->
    <div class="content-aside">
      <MiniCalendar 
        alertas={todosEventos} 
        onSelectDate={(date) => selectedDate = date} 
      />

      <Card style="margin-top: var(--sp-4);">
        <div class="section-header">
          <h3>Accesos Rápidos</h3>
        </div>
        <div class="quick-actions">
          <a href="/clientes" class="quick-action-btn">
            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <line x1="12" y1="5" x2="12" y2="19"/>
              <line x1="5" y1="12" x2="19" y2="12"/>
            </svg>
            Nuevo Cliente
          </a>
          <a href="/casos" class="quick-action-btn">
            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <line x1="12" y1="5" x2="12" y2="19"/>
              <line x1="5" y1="12" x2="19" y2="12"/>
            </svg>
            Nuevo Caso
          </a>
        </div>
      </Card>
    </div>
  </div>
</div>

<style>
  .dashboard {
    display: flex;
    flex-direction: column;
    gap: var(--sp-6);
  }

  /* ── Stats Grid ── */
  .stats-grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: var(--sp-4);
  }

  .stat-card {
    display: flex;
    align-items: center;
    gap: var(--sp-4);
    padding: var(--sp-5);
    background: var(--bg-surface);
    border: 1px solid var(--border-default);
    border-left: 3px solid var(--stat-accent);
    border-radius: var(--radius-lg);
    transition: box-shadow var(--ease-base);
  }

  .stat-card:hover {
    box-shadow: var(--shadow-md);
  }

  .stat-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 44px;
    height: 44px;
    border-radius: var(--radius-lg);
    background: var(--bg-elevated);
    color: var(--stat-accent);
    flex-shrink: 0;
  }

  .stat-info {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .stat-label {
    font-size: 0.8125rem;
    color: var(--text-secondary);
  }

  .stat-value {
    font-size: 1.5rem;
    font-weight: 700;
    color: var(--text-primary);
    font-family: var(--font-mono);
    letter-spacing: -0.02em;
  }

  /* ── Content Grid ── */
  .content-grid {
    display: grid;
    grid-template-columns: 1fr 320px;
    gap: var(--sp-4);
  }

  .section-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: var(--sp-5);
  }

  .section-header h3 {
    font-size: 1rem;
    font-weight: 600;
    color: var(--text-primary);
  }

  /* ── Empty State ── */
  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: var(--sp-10) var(--sp-4);
    text-align: center;
  }

  .empty-icon {
    color: var(--text-muted);
    margin-bottom: var(--sp-4);
    opacity: 0.5;
  }

  .empty-text {
    font-size: 0.875rem;
    color: var(--text-secondary);
    margin-bottom: var(--sp-1);
  }

  .empty-subtext {
    font-size: 0.75rem;
    color: var(--text-muted);
  }

  /* ── Quick Actions ── */
  .quick-actions { display: flex; flex-direction: column; gap: var(--sp-3); }
  .quick-action-btn {
    display: flex; align-items: center; gap: var(--sp-3);
    padding: var(--sp-4); background: var(--bg-primary);
    border: 1px solid var(--border-subtle); border-radius: var(--radius-md);
    color: var(--text-primary); text-decoration: none; font-weight: 500;
    transition: all var(--ease-fast);
  }
  .quick-action-btn:hover {
    border-color: var(--border-focus); background: var(--bg-hover);
    color: var(--accent-blue);
  }
  
  .alertas-list { display: flex; flex-direction: column; gap: var(--sp-3); }
  .alerta-card {
    display: flex; flex-direction: column; gap: var(--sp-1);
    padding: var(--sp-3) var(--sp-4);
    background: var(--bg-primary);
    border: 1px solid var(--border-subtle);
    border-left: 4px solid var(--accent-blue); /* Se sobreescribe inline */
    border-radius: var(--radius-md);
    text-decoration: none; color: inherit;
    transition: all var(--ease-base);
  }
  .alerta-card:hover {
    background: var(--bg-hover);
    transform: translateX(4px);
  }
  .alerta-header { display: flex; justify-content: space-between; align-items: center; }
  .alerta-header strong { font-size: 0.9375rem; color: var(--text-primary); }
  .alerta-meta { font-size: 0.75rem; color: var(--text-secondary); display: flex; gap: var(--sp-2); }
  .alerta-tipo { font-size: 0.75rem; color: var(--text-muted); font-weight: 600; margin-top: 4px; }
  .solventar-btn {
    background: var(--bg-secondary); border: 1px solid var(--border-subtle);
    color: var(--text-secondary); width: 24px; height: 24px;
    border-radius: var(--radius-sm); display: flex; align-items: center; justify-content: center;
    cursor: pointer; transition: all var(--ease-fast);
  }
  .solventar-btn:hover {
    background: var(--accent-green-soft); color: var(--accent-green); border-color: var(--accent-green-soft);
  }
</style>
