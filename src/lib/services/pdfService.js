import { PDFDocument, rgb } from 'pdf-lib';
import { invoke } from '@tauri-apps/api/core';
import { documentDir, join, tempDir } from '@tauri-apps/api/path';
import { getConfig } from './configService.js';

/**
 * Limpia un texto para la fuente estándar de pdf-lib (WinAnsi):
 * quita saltos de línea y caracteres que la fuente no puede dibujar
 * (emojis, flechas, etc.), que de lo contrario hacen fallar todo el PDF.
 * @param {any} text
 * @returns {string}
 */
function pdfSafe(text) {
  return String(text ?? '')
    .replace(/[\r\n\t]+/g, ' ')
    .replace(/[\u2018\u2019]/g, "'")
    .replace(/[\u201C\u201D]/g, '"')
    .replace(/[\u2013\u2014]/g, '-')
    .replace(/[^\x20-\x7E\u00A0-\u00FF\u2022\u20AC]/g, '');
}

/**
 * Genera un PDF de reporte utilizando un template.pdf existente.
 * Si no encuentra el template, genera uno simple.
 *
 * @param {Object} caso - Información completa del caso.
 * @param {Object} cliente - Información completa del cliente.
 * @param {Array} actuaciones - Historial de actuaciones.
 * @param {Array} terminos - Términos o eventos del caso.
 * @param {Object} config - Configuración de la app.
 * @returns {Promise<string>} Ruta absoluta del archivo generado.
 */
export async function generateReportePDF(caso, cliente, actuaciones = [], terminos = [], config = {}) {
  try {
    const basePath = await documentDir();
    const appDir = `${basePath}/Directorio_Casos`;

    // 1. Cargar el template si existe
    // Fetch config internally if not provided or empty
    if (!config || Object.keys(config).length === 0) {
      config = await getConfig();
    }

    let pdfDoc;
    let templatePath = await join(basePath, 'Directorio_Casos', 'Configuracion', 'template_membrete.pdf');

    try {
      const pdfBytesArray = await invoke('read_file_bytes', { path: templatePath });
      const existingPdfBytes = new Uint8Array(pdfBytesArray);
      pdfDoc = await PDFDocument.load(existingPdfBytes);
    } catch (e) {
      // Sin membrete: generar el reporte sobre una hoja carta en blanco.
      console.warn('No se encontró template_membrete.pdf; se usa una hoja en blanco.', e);
      pdfDoc = await PDFDocument.create();
      pdfDoc.addPage([612, 792]);
    }

    const pages = pdfDoc.getPages();
    let page = pages[0];
    const { width, height } = page.getSize();
    
    // Configuración de tipografía
    let yOffset = height - 120; // Iniciar debajo del membrete (suponiendo que ocupa ~100px)
    const margin = 50;
    const sizeTitle = 14;
    const sizeText = 11;

    // 2. Escribir Datos del Cliente
    page.drawText(`CLIENTE: ${pdfSafe(cliente.nombre_completo || cliente.razon_social)}`, { x: margin, y: yOffset, size: sizeTitle });
    yOffset -= 20;
    page.drawText(`Identificación: ${pdfSafe(cliente.identificacion || '')}`, { x: margin, y: yOffset, size: sizeText });
    yOffset -= 20;
    page.drawText(`Contacto: ${pdfSafe(cliente.telefono || '')} | ${pdfSafe(cliente.email || '')}`, { x: margin, y: yOffset, size: sizeText });
    
    yOffset -= 40;

    // 3. Escribir Datos del Caso
    page.drawText(`EXPEDIENTE: ${pdfSafe(caso.radicado || caso.slug)}`, { x: margin, y: yOffset, size: sizeTitle });
    yOffset -= 20;
    page.drawText(`Tipo de Proceso: ${pdfSafe(caso.tipo_proceso || 'N/A')}`, { x: margin, y: yOffset, size: sizeText });
    yOffset -= 20;
    page.drawText(`Despacho / Juzgado: ${pdfSafe(caso.juzgado || 'N/A')}`, { x: margin, y: yOffset, size: sizeText });
    yOffset -= 20;
    page.drawText(`Contraparte: ${pdfSafe(caso.contraparte || 'N/A')}`, { x: margin, y: yOffset, size: sizeText });
    yOffset -= 20;
    page.drawText(`Estado Actual: ${pdfSafe(caso.estado || 'Activo')}`, { x: margin, y: yOffset, size: sizeText });

    yOffset -= 40;

    // 4. Historial de Actuaciones (Resumen)
    if (actuaciones && actuaciones.length > 0) {
      page.drawText(`ÚLTIMAS ACTUACIONES`, { x: margin, y: yOffset, size: sizeTitle });
      yOffset -= 25;
      
      const maxAct = Math.min(actuaciones.length, 5); // Mostrar hasta 5
      for (let i = 0; i < maxAct; i++) {
        const act = actuaciones[i];
        if (yOffset < 50) {
           // Si se acaba la página, continuar en una nueva
           page = pdfDoc.addPage([width, height]);
           yOffset = height - 50;
        }
        const desc = pdfSafe(act.descripcion);
        page.drawText(`• ${pdfSafe(act.fecha)}: ${desc.substring(0, 80)}${desc.length > 80 ? '...' : ''}`, { 
          x: margin + 10, y: yOffset, size: sizeText 
        });
        yOffset -= 20;
      }
    }

    // Guardar el PDF generado
    const pdfBytes = await pdfDoc.save();
    
    const osTempDir = await tempDir();
    const fileName = `Reporte_Expediente_${String(caso.radicado || caso.id).replace(/[\\/:*?"<>|]/g, '_')}.pdf`;
    const outputPath = await join(osTempDir, fileName);

    await invoke('write_file_bytes', { path: outputPath, bytes: Array.from(pdfBytes) });

    return outputPath;

  } catch (err) {
    console.error('Error generando PDF:', err);
    throw err;
  }
}
