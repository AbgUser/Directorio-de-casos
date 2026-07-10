<script>
  import '../app.css';
  import LoginScreen from '$lib/components/auth/LoginScreen.svelte';
  import AppShell from '$lib/components/layout/AppShell.svelte';
  import { getAuth } from '$lib/stores/auth.svelte.js';
  import { page } from '$app/stores';
  import { isPermissionGranted, requestPermission, sendNotification } from '@tauri-apps/plugin-notification';
  import { getAlertas } from '$lib/services/dashboardService.js';

  let { children } = $props();

  const auth = getAuth();

  const pageTitles = {
    '/': 'Inicio',
    '/clientes': 'Directorio de Clientes',
    '/casos': 'Directorio General de Casos',
    '/configuracion': 'Configuración del Despacho'
  };

  let currentTitle = $derived(() => {
    const path = $page.url.pathname;
    if (path === '/') return 'Inicio';
    if (path.startsWith('/clientes')) return 'Directorio de Clientes';
    if (path.startsWith('/casos')) return 'Directorio General de Casos';
    if (path.startsWith('/calendario')) return 'Calendario';
    if (path.startsWith('/configuracion')) return 'Configuración del Despacho';
    return 'Inicio';
  });

  let notificationsChecked = false;

  $effect(() => {
    auth.checkAuthExists();
  });

  $effect(() => {
    if (auth.isAuthenticated && !notificationsChecked) {
      notificationsChecked = true;
      checkNotifications();
    }
  });

  async function checkNotifications() {
    try {
      let permissionGranted = await isPermissionGranted();
      if (!permissionGranted) {
        const permission = await requestPermission();
        permissionGranted = permission === 'granted';
      }
      
      if (permissionGranted) {
        const res = await getAlertas();
        const urgentes = res.alertas.filter(a => a.dias <= 2 && a.dias >= 0);
        
        for (const alert of urgentes) {
          sendNotification({
            title: 'Término por vencer',
            body: `Caso: ${alert.radicado || 'General'} - ${alert.titulo}`
          });
        }
      }
    } catch (e) {
      console.error('Error checking notifications:', e);
    }
  }
</script>

{#if auth.isLoading}
  <div class="loading-screen">
    <div class="loading-spinner"></div>
  </div>
{:else if !auth.isAuthenticated}
  <LoginScreen
    {auth}
    isFirstTime={auth.isFirstTime}
  />
{:else}
  <AppShell title={currentTitle()}>
    {@render children()}
  </AppShell>
{/if}

<style>
  .loading-screen {
    position: fixed;
    inset: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--bg-primary);
  }

  .loading-spinner {
    width: 32px;
    height: 32px;
    border: 3px solid var(--border-default);
    border-top-color: var(--accent-blue);
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }
</style>
