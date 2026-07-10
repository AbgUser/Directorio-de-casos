<script>
  import Button from '$lib/components/ui/Button.svelte';
  import Badge from '$lib/components/ui/Badge.svelte';
  import ConfirmDialog from '$lib/components/ui/ConfirmDialog.svelte';
  import EmptyState from '$lib/components/ui/EmptyState.svelte';
  import { listen } from '@tauri-apps/api/event';
  import * as documentoService from '$lib/services/documentoService.js';
  import { formatDate, formatFileSize } from '$lib/utils/formatters.js';
  import { open as openDialog } from '@tauri-apps/plugin-dialog';

  let { casoId, carpetaDocumentos = '' } = $props();

  let documentos = $state([]);
  let loading = $state(true);
  let uploading = $state(false);
  let deleteTarget = $state(null);
  let isDragging = $state(false);

  const fileIcons = {
    'application/pdf': { color: 'var(--accent-red)', label: 'PDF' },
    'application/msword': { color: 'var(--accent-blue)', label: 'DOC' },
    'application/vnd.openxmlformats-officedocument.wordprocessingml.document': { color: 'var(--accent-blue)', label: 'DOCX' },
    'application/vnd.ms-excel': { color: 'var(--accent-green)', label: 'XLS' },
    'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet': { color: 'var(--accent-green)', label: 'XLSX' },
    'image/jpeg': { color: 'var(--accent-purple)', label: 'JPG' },
    'image/png': { color: 'var(--accent-purple)', label: 'PNG' },
    'text/plain': { color: 'var(--text-secondary)', label: 'TXT' },
  };

  function getFileInfo(tipo) {
    return fileIcons[tipo] || { color: 'var(--text-muted)', label: 'FILE' };
  }

  $effect(() => { if (casoId) loadDocuments(); });

  $effect(() => {
    let unlistenEnter, unlistenLeave, unlistenDrop;

    async function setupDragAndDrop() {
      unlistenEnter = await listen('tauri://drag-enter', () => {
        isDragging = true;
      });
      unlistenLeave = await listen('tauri://drag-leave', () => {
        isDragging = false;
      });
      unlistenDrop = await listen('tauri://drag-drop', async (event) => {
        isDragging = false;
        const paths = event.payload?.paths || event.payload;
        if (Array.isArray(paths) && paths.length > 0) {
          uploading = true;
          try {
            await documentoService.upload(casoId, paths);
            await loadDocuments();
          } catch (e) {
            console.error('Error al subir documentos soltados:', e);
          } finally {
            uploading = false;
          }
        }
      });
    }

    setupDragAndDrop();

    return () => {
      if (unlistenEnter) unlistenEnter();
      if (unlistenLeave) unlistenLeave();
      if (unlistenDrop) unlistenDrop();
    };
  });

  export async function refresh() {
    await loadDocuments();
  }

  async function loadDocuments() {
    loading = true;
    try { documentos = await documentoService.getByCaso(casoId); }
    catch (e) { console.error(e); }
    finally { loading = false; }
  }

  async function handleUpload() {
    try {
      const selected = await openDialog({
        multiple: true,
        title: 'Seleccionar documentos para agregar al expediente'
      });
      if (!selected || selected.length === 0) return;
      uploading = true;
      const paths = Array.isArray(selected) ? selected.map(f => typeof f === 'string' ? f : f.path) : [typeof selected === 'string' ? selected : selected.path];
      await documentoService.upload(casoId, paths);
      await loadDocuments();
    } catch (e) {
      console.error('Error al subir documentos:', e);
    } finally {
      uploading = false;
    }
  }

  async function handleOpenFolder() {
    try {
      if (carpetaDocumentos) {
        await documentoService.openInFinder(carpetaDocumentos + '/.');
      }
    } catch (e) { console.error(e); }
  }

  async function handleOpenFile(doc) {
    try { 
      await documentoService.openFile(doc.ruta_absoluta); 
    } catch (e) { 
      console.error('Error al abrir archivo:', e);
      alert(e);
    }
  }

  async function handleShowInFinder(doc) {
    try { await documentoService.openInFinder(doc.ruta_absoluta); }
    catch (e) { 
      console.error(e);
    }
  }

  async function handlePrint(doc) {
    try { await documentoService.printFile(doc.ruta_absoluta); }
    catch (e) { 
      console.error('Error al imprimir:', e);
    }
  }

  async function handleDelete() {
    if (!deleteTarget) return;
    try {
      await documentoService.removeFile(deleteTarget.ruta_absoluta);
      deleteTarget = null;
      await loadDocuments();
    } catch (e) { console.error(e); }
  }
</script>

<div class="docs-container" style="position: relative; min-height: 200px;">
  {#if isDragging}
    <div class="drag-overlay">
      <div class="drag-content">
        <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="17 8 12 3 7 8"/><line x1="12" y1="3" x2="12" y2="15"/></svg>
        <p>Suelta los documentos aquí para anexarlos al expediente</p>
      </div>
    </div>
  {/if}

  <div class="docs-header">
    <h3>Expediente Digital</h3>
    <div class="docs-actions">
      <Button variant="ghost" size="sm" onclick={handleOpenFolder}>
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/></svg>
        Abrir en Finder
      </Button>
      <Button variant="primary" size="sm" onclick={handleUpload} disabled={uploading}>
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="17 8 12 3 7 8"/><line x1="12" y1="3" x2="12" y2="15"/></svg>
        {uploading ? 'Subiendo...' : 'Agregar Documento'}
      </Button>
    </div>
  </div>

  {#if loading}
    <div class="loading-state"><div class="spinner"></div></div>
  {:else if documentos.length === 0}
    <EmptyState title="Sin documentos" message="Agregue documentos PDF, escritos y anexos al expediente digital de este caso." />
  {:else}
    <div class="docs-grid">
      {#each documentos as doc}
        {@const info = getFileInfo(doc.tipo_archivo)}
        <div class="doc-card" ondblclick={() => handleOpenFile(doc)} role="button" tabindex="0">
          <div class="doc-icon" style="color: {info.color}">
            <svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8"/></svg>
            <span class="doc-ext">{info.label}</span>
          </div>
          <div class="doc-info">
            <span class="doc-name" title={doc.nombre}>{doc.nombre}</span>
            <span class="doc-meta">{formatFileSize(doc.tamano_bytes)} · {formatDate(doc.created_at)}</span>
          </div>
          <div class="doc-toolbar">
            <button class="tool-btn" onclick={() => handleOpenFile(doc)} title="Abrir archivo">
              <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"/><polyline points="15 3 21 3 21 9"/><line x1="10" y1="14" x2="21" y2="3"/></svg>
            </button>
            <button class="tool-btn" onclick={() => handleShowInFinder(doc)} title="Mostrar en Finder">
              <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/></svg>
            </button>
            <button class="tool-btn" onclick={() => handlePrint(doc)} title="Imprimir">
              <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="6 9 6 2 18 2 18 9"/><path d="M6 18H4a2 2 0 0 1-2-2v-5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v5a2 2 0 0 1-2 2h-2"/><rect x="6" y="14" width="12" height="8"/></svg>
            </button>
            <button class="tool-btn tool-delete" onclick={() => deleteTarget = doc} title="Quitar del expediente">
              <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="3 6 5 6 21 6"/><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/></svg>
            </button>
          </div>
        </div>
      {/each}
    </div>
    <div class="docs-footer">
      <span class="count">{documentos.length} documento{documentos.length !== 1 ? 's' : ''}</span>
    </div>
  {/if}
</div>

<ConfirmDialog open={!!deleteTarget} title="Mover a la Papelera" message="¿Eliminar definitivamente este archivo del disco? Esta acción borrará el archivo de su carpeta física." confirmLabel="Eliminar" onconfirm={handleDelete} oncancel={() => deleteTarget = null} />

<style>
  .docs-container { display: flex; flex-direction: column; gap: var(--sp-4); }
  .docs-header { display: flex; align-items: center; justify-content: space-between; }
  .docs-header h3 { font-size: 1rem; font-weight: 600; }
  .docs-actions { display: flex; gap: var(--sp-2); }
  .loading-state { display: flex; justify-content: center; padding: var(--sp-10); }
  .spinner { width: 24px; height: 24px; border: 3px solid var(--border-default); border-top-color: var(--accent-blue); border-radius: 50%; animation: spin 0.8s linear infinite; }

  .docs-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(280px, 1fr)); gap: var(--sp-3); }
  
  .drag-overlay {
    position: absolute; top: 0; left: 0; right: 0; bottom: 0;
    background: rgba(0, 0, 0, 0.7); z-index: 50;
    display: flex; align-items: center; justify-content: center;
    border-radius: var(--radius-lg);
    border: 4px dashed var(--accent-blue);
    backdrop-filter: blur(2px);
  }
  .drag-content {
    display: flex; flex-direction: column; align-items: center; gap: var(--sp-4);
    color: white; font-weight: 500; font-size: 1.125rem;
    pointer-events: none;
  }

  .doc-card {
    display: flex; align-items: center; gap: var(--sp-3);
    padding: var(--sp-3) var(--sp-4); background: var(--bg-surface);
    border: 1px solid var(--border-default); border-radius: var(--radius-lg);
    transition: all var(--ease-fast);
    cursor: pointer;
    user-select: none;
  }
  .doc-card:hover { border-color: var(--border-focus); box-shadow: var(--shadow-sm); }
  .doc-icon {
    position: relative; flex-shrink: 0;
    width: 40px; height: 40px; display: flex; align-items: center; justify-content: center;
  }
  .doc-ext {
    position: absolute; bottom: -2px; right: -4px;
    font-size: 0.5625rem; font-weight: 700; letter-spacing: 0.02em;
    background: var(--bg-elevated); padding: 1px 3px; border-radius: 2px;
    color: inherit;
  }
  .doc-info { flex: 1; min-width: 0; display: flex; flex-direction: column; gap: 2px; }
  .doc-name {
    font-size: 0.8125rem; font-weight: 500; color: var(--text-primary);
    overflow: hidden; text-overflow: ellipsis; white-space: nowrap;
  }
  .doc-meta { font-size: 0.6875rem; color: var(--text-muted); }
  .doc-toolbar { display: flex; gap: var(--sp-1); flex-shrink: 0; }
  .tool-btn {
    display: flex; align-items: center; justify-content: center;
    width: 28px; height: 28px; background: none;
    border: 1px solid transparent; border-radius: var(--radius-sm);
    color: var(--text-muted); cursor: pointer; transition: all var(--ease-fast);
  }
  .tool-btn:hover { color: var(--accent-blue); border-color: var(--border-default); background: var(--bg-elevated); }
  .tool-delete:hover { color: var(--accent-red); }
  .docs-footer { display: flex; justify-content: flex-end; }
  .count { font-size: 0.75rem; color: var(--text-muted); }
</style>
