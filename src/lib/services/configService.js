import { getDb } from './db.js';
import { documentDir, join } from '@tauri-apps/api/path';
import { copyFile, exists, mkdir } from '@tauri-apps/plugin-fs';

export async function getConfig() {
  const db = await getDb();
  const rows = await db.select('SELECT * FROM despacho_config WHERE id = 1');
  return rows[0] || {};
}

export async function updateConfig(data) {
  const db = await getDb();
  const keys = Object.keys(data);
  if (keys.length === 0) return;
  
  const setParts = keys.map((k, i) => `${k} = $${i + 1}`).join(', ');
  const values = Object.values(data);
  
  await db.execute(
    `UPDATE despacho_config SET ${setParts}, updated_at = datetime('now', 'localtime') WHERE id = 1`,
    values
  );
}
