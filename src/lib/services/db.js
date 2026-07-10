import { invoke } from '@tauri-apps/api/core';

let initialized = false;

export async function initDb(password) {
  if (initialized) {
    // If already initialized but we are trying to login again, we should probably reset or at least re-check
    // Wait, let's just let Rust handle it. 
    // Actually, we shouldn't rely on `initialized` for password checking.
  }
  const success = await invoke('db_init', { password });
  if (success) {
    initialized = true;
    
    // Parche seguro: Intentar añadir la columna si no existe en bases de datos antiguas
    try {
      await invoke('db_execute', { query: "ALTER TABLE actuaciones ADD COLUMN archivos_vinculados TEXT DEFAULT '[]'", params: [] });
    } catch(e) {
      // Ignorar error. Significa que la columna ya existe.
    }
  }
  return success;
}

export async function resetDbState() {
  initialized = false;
  try {
    await invoke('db_close_connection');
  } catch(e) {}
}

export async function getDb() {
  if (!initialized) {
    throw new Error('La base de datos no ha sido inicializada/desencriptada aún. Llama a initDb primero.');
  }
  return {
    select: async (query, params = []) => {
      return await invoke('db_select', { query, params });
    },
    execute: async (query, params = []) => {
      return await invoke('db_execute', { query, params });
    }
  };
}
