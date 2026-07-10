import { PDFDocument, rgb } from 'pdf-lib';
import { invoke } from '@tauri-apps/api/core';
import { documentDir, join, tempDir } from '@tauri-apps/api/path';
import { getConfig } from './configService.js';

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
      console.error('Error crítico al leer o procesar el archivo de plantilla con pdf-lib:', e);
      throw new Error('No se encontró el template base. Ve a Configuración y carga el PDF del membrete del despacho.');
    }

    const pages = pdfDoc.getPages();
    const page = pages[0];
    const { width, height } = page.getSize();
    
    // Configuración de tipografía
    let yOffset = height - 120; // Iniciar debajo del membrete (suponiendo que ocupa ~100px)
    const margin = 50;
    const sizeTitle = 14;
    const sizeText = 11;

    // 2. Escribir Datos del Cliente
    page.drawText(`CLIENTE: ${cliente.nombre_completo || cliente.razon_social}`, { x: margin, y: yOffset, size: sizeTitle });
    yOffset -= 20;
    page.drawText(`Identificación: ${cliente.identificacion || ''}`, { x: margin, y: yOffset, size: sizeText });
    yOffset -= 20;
    page.drawText(`Contacto: ${cliente.telefono || ''} | ${cliente.email || ''}`, { x: margin, y: yOffset, size: sizeText });
    
    yOffset -= 40;

    // 3. Escribir Datos del Caso
    page.drawText(`EXPEDIENTE: ${caso.radicado || caso.slug}`, { x: margin, y: yOffset, size: sizeTitle });
    yOffset -= 20;
    page.drawText(`Tipo de Proceso: ${caso.tipo_proceso || 'N/A'}`, { x: margin, y: yOffset, size: sizeText });
    yOffset -= 20;
    page.drawText(`Despacho / Juzgado: ${caso.juzgado || 'N/A'}`, { x: margin, y: yOffset, size: sizeText });
    yOffset -= 20;
    page.drawText(`Contraparte: ${caso.contraparte || 'N/A'}`, { x: margin, y: yOffset, size: sizeText });
    yOffset -= 20;
    page.drawText(`Estado Actual: ${caso.estado || 'Activo'}`, { x: margin, y: yOffset, size: sizeText });

    yOffset -= 40;

    // 4. Historial de Actuaciones (Resumen)
    if (actuaciones && actuaciones.length > 0) {
      page.drawText(`ÚLTIMAS ACTUACIONES`, { x: margin, y: yOffset, size: sizeTitle });
      yOffset -= 25;
      
      const maxAct = Math.min(actuaciones.length, 5); // Mostrar hasta 5
      for (let i = 0; i < maxAct; i++) {
        const act = actuaciones[i];
        if (yOffset < 50) {
           // Si se acaba la página, agregar otra (solo básico para este ejemplo)
           const newPage = pdfDoc.addPage();
           yOffset = height - 50;
        }
        page.drawText(`• ${act.fecha}: ${act.descripcion.substring(0, 80)}${act.descripcion.length > 80 ? '...' : ''}`, { 
          x: margin + 10, y: yOffset, size: sizeText 
        });
        yOffset -= 20;
      }
    }

    // Guardar el PDF generado
    const pdfBytes = await pdfDoc.save();
    
    const osTempDir = await tempDir();
    const fileName = `Reporte_Expediente_${caso.radicado || caso.id}.pdf`;
    const outputPath = await join(osTempDir, fileName);

    await invoke('write_file_bytes', { path: outputPath, bytes: Array.from(pdfBytes) });

    return outputPath;

  } catch (err) {
    console.error('Error generando PDF:', err);
    throw err;
  }
}
