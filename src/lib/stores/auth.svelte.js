import { invoke } from '@tauri-apps/api/core';
import { initDb, resetDbState } from '$lib/services/db.js';

let isAuthenticated = $state(false);
let isLoading = $state(true);
let isFirstTime = $state(false);
let error = $state('');

export function getAuth() {
  return {
    get isAuthenticated() { return isAuthenticated; },
    get isLoading() { return isLoading; },
    get isFirstTime() { return isFirstTime; },
    get error() { return error; },

    async checkAuthExists() {
      try {
        const dbExists = await invoke('db_exists');
        isFirstTime = !dbExists;
      } catch (e) {
        console.error('Error verificando existencia de base de datos:', e);
        isFirstTime = true;
      } finally {
        isLoading = false;
      }
    },

    async setupPassword(password) {
      try {
        error = '';
        const success = await initDb(password);
        if (success) {
          isAuthenticated = true;
          isFirstTime = false;
          // Guardar en keychain por defecto
          try { await invoke('save_to_keychain', { password }); } catch (e) { console.warn('Error guardando en keychain:', e); }
          return { success: true };
        } else {
          const msg = 'No se pudo inicializar la base de datos segura.';
          error = msg;
          return { success: false, error: msg };
        }
      } catch (e) {
        const msg = 'Error al configurar contraseña: ' + String(e);
        error = msg;
        return { success: false, error: msg };
      }
    },

    async login(password) {
      try {
        error = '';
        const success = await initDb(password);
        if (success) {
          isAuthenticated = true;
          return { success: true };
        } else {
          error = 'Contraseña incorrecta o archivo de base de datos corrupto.';
          return { success: false, error: error };
        }
      } catch (e) {
        const msg = typeof e === 'string' ? e : 'Error de autenticación: ' + String(e);
        error = msg;
        return { success: false, error: msg };
      }
    },

    async changePassword(newPassword) {
      try {
        await invoke('db_change_password', { newPassword });
        return { success: true };
      } catch (e) {
        return { success: false, error: String(e) };
      }
    },



    async resetApp() {
      try {
        await invoke('db_reset');
        await resetDbState();
        isAuthenticated = false;
        isFirstTime = true;
        error = '';
        return { success: true };
      } catch (e) {
        return { success: false, error: String(e) };
      }
    },

    logout() {
      resetDbState();
      isAuthenticated = false;
    }
  };
}

