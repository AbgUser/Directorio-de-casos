// =============================================================================
// db/migrations.rs
// Módulo de migraciones y datos semilla para el Directorio General de Casos.
//
// Contiene:
// - SQL_SCHEMA: DDL completo (tablas, índices)
// - SQL_SEED_FERIADOS: Feriados colombianos 2024-2030
// - SQL_SEED_CONFIG: Configuración inicial del despacho
// =============================================================================

/// Esquema SQL completo de la base de datos.
///
/// Incluye todas las tablas del sistema:
/// - auth_config: configuración de autenticación (PIN/contraseña)
/// - clientes: datos de personas naturales y jurídicas
/// - casos: expedientes judiciales y administrativos
/// - actuaciones: registro de movimientos procesales
/// - documentos: archivos digitales asociados a casos
/// - eventos_terminos: calendario de términos y audiencias
/// - honorarios_cuentas: cuentas contables por caso
/// - honorarios_movimientos: movimientos financieros
/// - despacho_config: configuración general del despacho
/// - plantillas: plantillas de documentos
/// - feriados: días festivos para cálculo de términos
pub const SQL_SCHEMA: &str = r#"
-- =========================================================================
-- TABLA: auth_config
-- Almacena la configuración de autenticación del usuario.
-- Solo debe existir una fila (singleton).
-- =========================================================================
CREATE TABLE IF NOT EXISTS auth_config (
    id              INTEGER PRIMARY KEY CHECK (id = 1),
    pin_hash        TEXT,
    password_hash   TEXT,
    auth_method     TEXT NOT NULL DEFAULT 'none' CHECK (auth_method IN ('none', 'pin', 'password')),
    created_at      TEXT NOT NULL DEFAULT (datetime('now', 'localtime')),
    updated_at      TEXT NOT NULL DEFAULT (datetime('now', 'localtime'))
);

-- =========================================================================
-- TABLA: clientes
-- Registro de clientes del despacho (personas naturales o jurídicas).
-- =========================================================================
CREATE TABLE IF NOT EXISTS clientes (
    id                  INTEGER PRIMARY KEY AUTOINCREMENT,
    tipo_persona        TEXT NOT NULL DEFAULT 'natural' CHECK (tipo_persona IN ('natural', 'juridica')),
    nombre_completo     TEXT NOT NULL,
    tipo_identificacion TEXT NOT NULL CHECK (tipo_identificacion IN ('CC', 'NIT', 'CE', 'Pasaporte', 'Otro')),
    identificacion      TEXT NOT NULL,
    telefono            TEXT,
    email               TEXT,
    direccion           TEXT,
    -- Metadatos
    notas               TEXT,
    activo              INTEGER NOT NULL DEFAULT 1,
    created_at          TEXT NOT NULL DEFAULT (datetime('now', 'localtime')),
    updated_at          TEXT NOT NULL DEFAULT (datetime('now', 'localtime'))
);

CREATE INDEX IF NOT EXISTS idx_clientes_nombre ON clientes(nombre_completo);
CREATE INDEX IF NOT EXISTS idx_clientes_identificacion ON clientes(identificacion);
CREATE INDEX IF NOT EXISTS idx_clientes_activo ON clientes(activo);

-- =========================================================================
-- TABLA: casos
-- Expedientes judiciales, administrativos o de consulta.
-- =========================================================================
CREATE TABLE IF NOT EXISTS casos (
    id                  INTEGER PRIMARY KEY AUTOINCREMENT,
    cliente_id          INTEGER NOT NULL,
    -- Tipo de expediente
    categoria_expediente TEXT NOT NULL DEFAULT 'proceso',
    -- Identificación del caso
    radicado            TEXT, -- Idealmente 21 dígitos
    slug                TEXT NOT NULL,
    tipo_proceso        TEXT NOT NULL DEFAULT 'Ordinario Laboral',
    -- Despacho judicial
    juzgado             TEXT,
    -- Contraparte
    contraparte         TEXT,
    -- Estado
    estado              TEXT NOT NULL DEFAULT 'Activo',
    -- Descripción y fechas
    descripcion         TEXT,
    notas_internas      TEXT,
    fecha_inicio        TEXT,
    fecha_terminacion   TEXT,
    -- Carpeta de documentos (ruta relativa al directorio del app)
    carpeta_documentos  TEXT,
    -- Metadatos
    created_at          TEXT NOT NULL DEFAULT (datetime('now', 'localtime')),
    updated_at          TEXT NOT NULL DEFAULT (datetime('now', 'localtime')),
    FOREIGN KEY (cliente_id) REFERENCES clientes(id) ON DELETE RESTRICT
);

CREATE INDEX IF NOT EXISTS idx_casos_cliente ON casos(cliente_id);
CREATE INDEX IF NOT EXISTS idx_casos_radicado ON casos(radicado);
CREATE INDEX IF NOT EXISTS idx_casos_slug ON casos(slug);
CREATE INDEX IF NOT EXISTS idx_casos_estado ON casos(estado);
CREATE INDEX IF NOT EXISTS idx_casos_tipo_proceso ON casos(tipo_proceso);

-- =========================================================================
-- TABLA: actuaciones
-- Registro cronológico de movimientos procesales de cada caso.
-- =========================================================================
CREATE TABLE IF NOT EXISTS actuaciones (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    caso_id         INTEGER NOT NULL,
    fecha           TEXT NOT NULL,
    tipo            TEXT NOT NULL DEFAULT 'anotacion' CHECK (tipo IN (
        'auto', 'sentencia', 'notificacion', 'audiencia',
        'memorial', 'anotacion', 'otro'
    )),
    descripcion     TEXT NOT NULL,
    fecha_notificacion TEXT,
    -- Si esta actuación genera un término
    genera_termino  INTEGER NOT NULL DEFAULT 0,
    termino_vencimiento TEXT,
    termino_completado INTEGER DEFAULT 0,
    dias_termino    INTEGER,
    tipo_dias       TEXT CHECK (tipo_dias IN ('habiles', 'calendario')),
    archivos_vinculados TEXT DEFAULT '[]',
    -- Metadatos
    created_at      TEXT NOT NULL DEFAULT (datetime('now', 'localtime')),
    updated_at      TEXT NOT NULL DEFAULT (datetime('now', 'localtime')),
    FOREIGN KEY (caso_id) REFERENCES casos(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_actuaciones_caso ON actuaciones(caso_id);
CREATE INDEX IF NOT EXISTS idx_actuaciones_fecha ON actuaciones(fecha);
CREATE INDEX IF NOT EXISTS idx_actuaciones_tipo ON actuaciones(tipo);

-- =========================================================================
-- TABLA: documentos
-- Archivos digitales asociados a un caso.
-- =========================================================================
CREATE TABLE IF NOT EXISTS documentos (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    caso_id         INTEGER NOT NULL,
    actuacion_id    INTEGER,
    nombre          TEXT NOT NULL,
    nombre_archivo  TEXT NOT NULL,
    ruta_relativa   TEXT NOT NULL,
    tipo_archivo    TEXT,
    tamano_bytes    INTEGER,
    descripcion     TEXT,
    -- Metadatos
    created_at      TEXT NOT NULL DEFAULT (datetime('now', 'localtime')),
    FOREIGN KEY (caso_id) REFERENCES casos(id) ON DELETE CASCADE,
    FOREIGN KEY (actuacion_id) REFERENCES actuaciones(id) ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS idx_documentos_caso ON documentos(caso_id);
CREATE INDEX IF NOT EXISTS idx_documentos_actuacion ON documentos(actuacion_id);

-- =========================================================================
-- TABLA: eventos_terminos
-- Calendario de términos judiciales, audiencias y recordatorios.
-- =========================================================================
CREATE TABLE IF NOT EXISTS eventos_terminos (
    id                  INTEGER PRIMARY KEY AUTOINCREMENT,
    caso_id             INTEGER,
    actuacion_id        INTEGER,
    titulo              TEXT NOT NULL,
    tipo                TEXT NOT NULL DEFAULT 'termino' CHECK (tipo IN (
        'termino', 'audiencia', 'vencimiento', 'recordatorio', 'otro'
    )),
    fecha_inicio        TEXT NOT NULL,
    fecha_fin           TEXT,
    todo_el_dia         INTEGER NOT NULL DEFAULT 1,
    hora_inicio         TEXT,
    hora_fin            TEXT,
    -- Estado del evento
    estado              TEXT NOT NULL DEFAULT 'pendiente' CHECK (estado IN (
        'pendiente', 'cumplido', 'vencido', 'cancelado'
    )),
    -- Notificación
    notificar           INTEGER NOT NULL DEFAULT 1,
    dias_antes_notif    INTEGER DEFAULT 1,
    -- Descripción
    descripcion         TEXT,
    -- Metadatos
    created_at          TEXT NOT NULL DEFAULT (datetime('now', 'localtime')),
    updated_at          TEXT NOT NULL DEFAULT (datetime('now', 'localtime')),
    FOREIGN KEY (caso_id) REFERENCES casos(id) ON DELETE CASCADE,
    FOREIGN KEY (actuacion_id) REFERENCES actuaciones(id) ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS idx_eventos_caso ON eventos_terminos(caso_id);
CREATE INDEX IF NOT EXISTS idx_eventos_fecha ON eventos_terminos(fecha_inicio);
CREATE INDEX IF NOT EXISTS idx_eventos_estado ON eventos_terminos(estado);
CREATE INDEX IF NOT EXISTS idx_eventos_tipo ON eventos_terminos(tipo);



-- =========================================================================
-- TABLA: despacho_config
-- Configuración general del despacho jurídico (singleton).
-- =========================================================================
CREATE TABLE IF NOT EXISTS despacho_config (
    id                  INTEGER PRIMARY KEY CHECK (id = 1),
    nombre_abogado      TEXT,
    titulo_profesional  TEXT,
    tarjeta_profesional TEXT,
    direccion_oficina   TEXT,
    ciudad              TEXT,
    telefono            TEXT,
    celular             TEXT,
    email               TEXT,
    sitio_web           TEXT,
    logo_path           TEXT,
    firma_path          TEXT,
    -- Metadatos
    created_at          TEXT NOT NULL DEFAULT (datetime('now', 'localtime')),
    updated_at          TEXT NOT NULL DEFAULT (datetime('now', 'localtime'))
);

-- =========================================================================
-- TABLA: plantillas
-- Plantillas de documentos (poderes, memoriales, contratos, etc.).
-- =========================================================================
CREATE TABLE IF NOT EXISTS plantillas (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    nombre          TEXT NOT NULL,
    categoria       TEXT NOT NULL DEFAULT 'general' CHECK (categoria IN (
        'poder', 'memorial', 'contrato', 'acta', 'concepto', 'general'
    )),
    contenido       TEXT NOT NULL,
    variables       TEXT,
    descripcion     TEXT,
    activa          INTEGER NOT NULL DEFAULT 1,
    -- Metadatos
    created_at      TEXT NOT NULL DEFAULT (datetime('now', 'localtime')),
    updated_at      TEXT NOT NULL DEFAULT (datetime('now', 'localtime'))
);

CREATE INDEX IF NOT EXISTS idx_plantillas_categoria ON plantillas(categoria);
CREATE INDEX IF NOT EXISTS idx_plantillas_activa ON plantillas(activa);

-- =========================================================================
-- TABLA: feriados
-- Días festivos colombianos para cálculo de términos judiciales.
-- =========================================================================
CREATE TABLE IF NOT EXISTS feriados (
    id                  INTEGER PRIMARY KEY AUTOINCREMENT,
    fecha               TEXT NOT NULL UNIQUE,
    nombre              TEXT NOT NULL,
    tipo                TEXT NOT NULL DEFAULT 'fijo' CHECK (tipo IN (
        'fijo', 'trasladable', 'variable'
    )),
    recurrente_anual    INTEGER NOT NULL DEFAULT 0,
    origen              TEXT NOT NULL DEFAULT 'sistema' CHECK (origen IN ('sistema', 'usuario')),
    -- Metadatos
    created_at          TEXT NOT NULL DEFAULT (datetime('now', 'localtime'))
);

CREATE INDEX IF NOT EXISTS idx_feriados_fecha ON feriados(fecha);



-- =========================================================================
-- TABLA: modelos_legales
-- Bóveda global de plantillas y modelos físicos (Word, PDF, etc).
-- =========================================================================
CREATE TABLE IF NOT EXISTS modelos_legales (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    nombre          TEXT NOT NULL,
    tipo            TEXT NOT NULL,
    ruta_archivo    TEXT NOT NULL,
    -- Metadatos
    created_at      TEXT NOT NULL DEFAULT (datetime('now', 'localtime'))
);

CREATE INDEX IF NOT EXISTS idx_modelos_tipo ON modelos_legales(tipo);

"#;

/// Datos semilla: feriados colombianos 2024-2030.
///
/// Incluye:
/// - Feriados fijos (recurrente_anual = 1): Año Nuevo, Día del Trabajo,
///   Independencia, Batalla de Boyacá, Inmaculada Concepción, Navidad.
/// - Feriados trasladados al lunes por Ley 51/1983 (recurrente_anual = 0):
///   Reyes Magos, San José, San Pedro y San Pablo, Asunción, Día de la Raza,
///   Todos los Santos, Independencia de Cartagena.
/// - Feriados variables (recurrente_anual = 0): Jueves Santo, Viernes Santo,
///   Ascensión del Señor, Corpus Christi, Sagrado Corazón.
pub const SQL_SEED_FERIADOS: &str = r#"
-- =========================================================================
-- FERIADOS FIJOS (se repiten cada año en la misma fecha)
-- recurrente_anual = 1
-- =========================================================================

-- Año Nuevo - 1 de enero
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2024-01-01', 'Año Nuevo', 'fijo', 1, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2025-01-01', 'Año Nuevo', 'fijo', 1, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2026-01-01', 'Año Nuevo', 'fijo', 1, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2027-01-01', 'Año Nuevo', 'fijo', 1, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2028-01-01', 'Año Nuevo', 'fijo', 1, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2029-01-01', 'Año Nuevo', 'fijo', 1, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2030-01-01', 'Año Nuevo', 'fijo', 1, 'sistema');

-- Día del Trabajo - 1 de mayo
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2024-05-01', 'Día del Trabajo', 'fijo', 1, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2025-05-01', 'Día del Trabajo', 'fijo', 1, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2026-05-01', 'Día del Trabajo', 'fijo', 1, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2027-05-01', 'Día del Trabajo', 'fijo', 1, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2028-05-01', 'Día del Trabajo', 'fijo', 1, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2029-05-01', 'Día del Trabajo', 'fijo', 1, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2030-05-01', 'Día del Trabajo', 'fijo', 1, 'sistema');

-- Grito de Independencia - 20 de julio
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2024-07-20', 'Grito de Independencia', 'fijo', 1, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2025-07-20', 'Grito de Independencia', 'fijo', 1, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2026-07-20', 'Grito de Independencia', 'fijo', 1, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2027-07-20', 'Grito de Independencia', 'fijo', 1, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2028-07-20', 'Grito de Independencia', 'fijo', 1, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2029-07-20', 'Grito de Independencia', 'fijo', 1, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2030-07-20', 'Grito de Independencia', 'fijo', 1, 'sistema');

-- Batalla de Boyacá - 7 de agosto
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2024-08-07', 'Batalla de Boyacá', 'fijo', 1, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2025-08-07', 'Batalla de Boyacá', 'fijo', 1, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2026-08-07', 'Batalla de Boyacá', 'fijo', 1, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2027-08-07', 'Batalla de Boyacá', 'fijo', 1, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2028-08-07', 'Batalla de Boyacá', 'fijo', 1, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2029-08-07', 'Batalla de Boyacá', 'fijo', 1, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2030-08-07', 'Batalla de Boyacá', 'fijo', 1, 'sistema');

-- Inmaculada Concepción - 8 de diciembre
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2024-12-08', 'Inmaculada Concepción', 'fijo', 1, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2025-12-08', 'Inmaculada Concepción', 'fijo', 1, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2026-12-08', 'Inmaculada Concepción', 'fijo', 1, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2027-12-08', 'Inmaculada Concepción', 'fijo', 1, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2028-12-08', 'Inmaculada Concepción', 'fijo', 1, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2029-12-08', 'Inmaculada Concepción', 'fijo', 1, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2030-12-08', 'Inmaculada Concepción', 'fijo', 1, 'sistema');

-- Navidad - 25 de diciembre
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2024-12-25', 'Navidad', 'fijo', 1, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2025-12-25', 'Navidad', 'fijo', 1, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2026-12-25', 'Navidad', 'fijo', 1, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2027-12-25', 'Navidad', 'fijo', 1, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2028-12-25', 'Navidad', 'fijo', 1, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2029-12-25', 'Navidad', 'fijo', 1, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2030-12-25', 'Navidad', 'fijo', 1, 'sistema');

-- =========================================================================
-- FERIADOS TRASLADADOS AL LUNES (Ley 51 de 1983)
-- Fechas precalculadas para cada año. recurrente_anual = 0
-- =========================================================================

-- Día de los Reyes Magos (6 de enero → lunes siguiente)
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2024-01-08', 'Día de los Reyes Magos', 'trasladable', 0, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2025-01-06', 'Día de los Reyes Magos', 'trasladable', 0, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2026-01-12', 'Día de los Reyes Magos', 'trasladable', 0, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2027-01-11', 'Día de los Reyes Magos', 'trasladable', 0, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2028-01-10', 'Día de los Reyes Magos', 'trasladable', 0, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2029-01-08', 'Día de los Reyes Magos', 'trasladable', 0, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2030-01-07', 'Día de los Reyes Magos', 'trasladable', 0, 'sistema');

-- Día de San José (19 de marzo → lunes siguiente)
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2024-03-25', 'Día de San José', 'trasladable', 0, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2025-03-24', 'Día de San José', 'trasladable', 0, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2026-03-23', 'Día de San José', 'trasladable', 0, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2027-03-22', 'Día de San José', 'trasladable', 0, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2028-03-20', 'Día de San José', 'trasladable', 0, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2029-03-19', 'Día de San José', 'trasladable', 0, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2030-03-25', 'Día de San José', 'trasladable', 0, 'sistema');

-- San Pedro y San Pablo (29 de junio → lunes siguiente)
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2024-07-01', 'San Pedro y San Pablo', 'trasladable', 0, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2025-06-30', 'San Pedro y San Pablo', 'trasladable', 0, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2026-06-29', 'San Pedro y San Pablo', 'trasladable', 0, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2027-07-05', 'San Pedro y San Pablo', 'trasladable', 0, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2028-07-03', 'San Pedro y San Pablo', 'trasladable', 0, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2029-07-02', 'San Pedro y San Pablo', 'trasladable', 0, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2030-07-01', 'San Pedro y San Pablo', 'trasladable', 0, 'sistema');

-- Asunción de la Virgen (15 de agosto → lunes siguiente)
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2024-08-19', 'Asunción de la Virgen', 'trasladable', 0, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2025-08-18', 'Asunción de la Virgen', 'trasladable', 0, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2026-08-17', 'Asunción de la Virgen', 'trasladable', 0, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2027-08-16', 'Asunción de la Virgen', 'trasladable', 0, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2028-08-21', 'Asunción de la Virgen', 'trasladable', 0, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2029-08-20', 'Asunción de la Virgen', 'trasladable', 0, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2030-08-19', 'Asunción de la Virgen', 'trasladable', 0, 'sistema');

-- Día de la Raza (12 de octubre → lunes siguiente)
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2024-10-14', 'Día de la Raza', 'trasladable', 0, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2025-10-13', 'Día de la Raza', 'trasladable', 0, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2026-10-12', 'Día de la Raza', 'trasladable', 0, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2027-10-18', 'Día de la Raza', 'trasladable', 0, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2028-10-16', 'Día de la Raza', 'trasladable', 0, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2029-10-15', 'Día de la Raza', 'trasladable', 0, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2030-10-14', 'Día de la Raza', 'trasladable', 0, 'sistema');

-- Día de Todos los Santos (1 de noviembre → lunes siguiente)
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2024-11-04', 'Día de Todos los Santos', 'trasladable', 0, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2025-11-03', 'Día de Todos los Santos', 'trasladable', 0, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2026-11-02', 'Día de Todos los Santos', 'trasladable', 0, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2027-11-01', 'Día de Todos los Santos', 'trasladable', 0, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2028-11-06', 'Día de Todos los Santos', 'trasladable', 0, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2029-11-05', 'Día de Todos los Santos', 'trasladable', 0, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2030-11-04', 'Día de Todos los Santos', 'trasladable', 0, 'sistema');

-- Independencia de Cartagena (11 de noviembre → lunes siguiente)
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2024-11-11', 'Independencia de Cartagena', 'trasladable', 0, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2025-11-17', 'Independencia de Cartagena', 'trasladable', 0, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2026-11-16', 'Independencia de Cartagena', 'trasladable', 0, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2027-11-15', 'Independencia de Cartagena', 'trasladable', 0, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2028-11-13', 'Independencia de Cartagena', 'trasladable', 0, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2029-11-12', 'Independencia de Cartagena', 'trasladable', 0, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2030-11-11', 'Independencia de Cartagena', 'trasladable', 0, 'sistema');

-- =========================================================================
-- FERIADOS VARIABLES (dependen de la Pascua)
-- Fechas precalculadas para cada año. recurrente_anual = 0
-- =========================================================================

-- 2024: Pascua = 31 de marzo
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2024-03-28', 'Jueves Santo', 'variable', 0, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2024-03-29', 'Viernes Santo', 'variable', 0, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2024-05-13', 'Ascensión del Señor', 'variable', 0, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2024-06-03', 'Corpus Christi', 'variable', 0, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2024-06-10', 'Sagrado Corazón de Jesús', 'variable', 0, 'sistema');

-- 2025: Pascua = 20 de abril
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2025-04-17', 'Jueves Santo', 'variable', 0, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2025-04-18', 'Viernes Santo', 'variable', 0, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2025-06-02', 'Ascensión del Señor', 'variable', 0, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2025-06-23', 'Corpus Christi', 'variable', 0, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2025-06-30', 'Sagrado Corazón de Jesús', 'variable', 0, 'sistema');

-- 2026: Pascua = 5 de abril
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2026-04-02', 'Jueves Santo', 'variable', 0, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2026-04-03', 'Viernes Santo', 'variable', 0, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2026-05-18', 'Ascensión del Señor', 'variable', 0, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2026-06-08', 'Corpus Christi', 'variable', 0, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2026-06-15', 'Sagrado Corazón de Jesús', 'variable', 0, 'sistema');

-- 2027: Pascua = 28 de marzo
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2027-03-25', 'Jueves Santo', 'variable', 0, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2027-03-26', 'Viernes Santo', 'variable', 0, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2027-05-10', 'Ascensión del Señor', 'variable', 0, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2027-05-31', 'Corpus Christi', 'variable', 0, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2027-06-07', 'Sagrado Corazón de Jesús', 'variable', 0, 'sistema');

-- 2028: Pascua = 16 de abril
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2028-04-13', 'Jueves Santo', 'variable', 0, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2028-04-14', 'Viernes Santo', 'variable', 0, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2028-05-29', 'Ascensión del Señor', 'variable', 0, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2028-06-19', 'Corpus Christi', 'variable', 0, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2028-06-26', 'Sagrado Corazón de Jesús', 'variable', 0, 'sistema');

-- 2029: Pascua = 1 de abril
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2029-03-29', 'Jueves Santo', 'variable', 0, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2029-03-30', 'Viernes Santo', 'variable', 0, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2029-05-14', 'Ascensión del Señor', 'variable', 0, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2029-06-04', 'Corpus Christi', 'variable', 0, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2029-06-11', 'Sagrado Corazón de Jesús', 'variable', 0, 'sistema');

-- 2030: Pascua = 21 de abril
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2030-04-17', 'Jueves Santo', 'variable', 0, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2030-04-18', 'Viernes Santo', 'variable', 0, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2030-06-03', 'Ascensión del Señor', 'variable', 0, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2030-06-24', 'Corpus Christi', 'variable', 0, 'sistema');
INSERT OR IGNORE INTO feriados (fecha, nombre, tipo, recurrente_anual, origen)
VALUES ('2030-07-01', 'Sagrado Corazón de Jesús', 'variable', 0, 'sistema');
"#;

/// Datos semilla: configuración inicial del despacho jurídico.
pub const SQL_SEED_CONFIG: &str = r#"
INSERT OR IGNORE INTO despacho_config (id, titulo_profesional)
VALUES (1, 'Abogado y Profesor');
"#;

// =============================================================================
// Funciones públicas para obtener las cadenas SQL
// =============================================================================

/// Retorna el esquema SQL completo para crear todas las tablas e índices.
pub fn get_schema() -> &'static str {
    SQL_SCHEMA
}

/// Retorna las sentencias SQL para insertar los feriados colombianos (2024-2030).
pub fn get_seed_feriados() -> &'static str {
    SQL_SEED_FERIADOS
}

/// Retorna la sentencia SQL para insertar la configuración inicial del despacho.
pub fn get_seed_config() -> &'static str {
    SQL_SEED_CONFIG
}
