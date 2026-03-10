# Clarificaciones: Proceso audit-tool

**Tarea:** create-process-audit-tool  
**Fecha:** 2026-03-10  
**Estado:** Resuelto

---

## Dudas y gaps identificados

A continuación se listan las dudas surgidas durante la especificación. Se requiere decisión del usuario para continuar.

---

### Q1: Invocación del ejecutable — ¿parámetros por defecto?

**Contexto:** El proceso indica invocar el `.exe` de la herramienta. Sin embargo, algunas herramientas pueden requerir parámetros específicos.

**Opciones:**

| Opción | Descripción |
|--------|-------------|
| A | Invocar siempre sin parámetros (configuración por defecto) |
| B | Leer `<tool-id>-config.json` para extraer parámetros de prueba |
| C | Definir en `objectives.md` de cada auditoría los parámetros a usar |

**Recomendación:** Opción B (usar config si existe) con fallback a sin parámetros.

---

### Q2: Resultado "PARTIAL" — ¿cuándo aplica?

**Contexto:** El esquema de `audit-result.json` incluye tres resultados posibles: PASS, FAIL, PARTIAL.

**Pregunta:** ¿Cuándo debe marcarse como PARTIAL?

**Propuesta:**
- **PASS:** Todas las validaciones (JSON + funcional) correctas.
- **FAIL:** Alguna validación crítica falla (no arranca, JSON inválido, objetivo funcional no cumplido).
- **PARTIAL:** La herramienta arranca y JSON es válido, pero hay warnings o validaciones opcionales fallidas.

**¿Aceptas esta definición o prefieres otra semántica?**

---

### Q3: Endpoint de health — ¿cómo identificarlo?

**Contexto:** Para `start-api`, se debe validar que el endpoint de health responde correctamente.

**Pregunta:** ¿Cómo determinar la URL del health endpoint?

**Opciones:**

| Opción | Descripción |
|--------|-------------|
| A | Hardcoded por herramienta en la auditoría (`http://localhost:5000/health`) |
| B | Leer de `start-api-config.json` si tiene campo `healthEndpoint` |
| C | Convención: siempre `http://localhost:<port>/health` |
| D | El manifest.json de la herramienta debe declarar `validation.health_endpoint` |

**Recomendación:** Opción B con fallback a convención (opción C).

---

### Q4: Validación funcional — ¿qué respuesta HTTP es aceptable?

**Contexto:** El health endpoint puede devolver distintos formatos.

**Pregunta:** ¿Qué constituye una respuesta válida?

**Propuesta:**
- HTTP Status: 200 OK
- Body: Cualquiera que indique estado saludable (JSON con `status: "healthy"`, texto "OK", etc.)

**¿Es suficiente con HTTP 200 o se requiere validar el body?**

---

### Q5: ¿Debe el proceso detener la herramienta tras la auditoría?

**Contexto:** `start-api` levanta la API y la deja corriendo. Tras validar el health, la API sigue activa.

**Pregunta:** ¿El proceso audit-tool debe:

| Opción | Descripción |
|--------|-------------|
| A | Dejar la herramienta/proceso corriendo tras la auditoría |
| B | Detener/kill el proceso tras completar la validación |
| C | Configurable por herramienta (flag en config o manifest) |

**Recomendación:** Opción C (configurable), con default B (cleanup).

---

### Q6: Ubicación del informe — ¿versionado por fecha?

**Contexto:** El informe se guarda en `paths.auditsPath/tools/<tool-id>/`.

**Pregunta:** Si se ejecutan múltiples auditorías de la misma herramienta, ¿cómo nombrar los informes?

**Opciones:**

| Opción | Descripción |
|--------|-------------|
| A | Sobrescribir siempre `audit-report.md` y `audit-result.json` |
| B | Versionado por fecha: `audit-report-2026-03-10.md` |
| C | Mantener histórico: `audit-report-<timestamp>.md` y un `latest/` con el más reciente |

**Recomendación:** Opción B (versionado por fecha) para trazabilidad sin acumulación excesiva.

---

### Q7: ¿Registrar la auditoría en Evolution Log?

**Contexto:** Las auditorías pueden generar hallazgos relevantes.

**Pregunta:** ¿Siempre registrar en Evolution Log o solo si hay FAIL/hallazgos?

**Opciones:**

| Opción | Descripción |
|--------|-------------|
| A | Siempre registrar (trazabilidad completa) |
| B | Solo si resultado es FAIL o PARTIAL |
| C | Solo si hay recomendaciones o hallazgos |

**Recomendación:** Opción B (registrar solo anomalías).

---

## Decisiones confirmadas

| Q# | Tema | Decisión | Detalle |
|----|------|----------|---------|
| Q1 | Parámetros de invocación | **B** | Leer config, informar al usuario de los parámetros antes de ejecutar |
| Q2 | Semántica de PARTIAL | **Aceptada** | PASS/FAIL/PARTIAL como propuesto |
| Q3 | Health endpoint | **B + C** | Leer de config, fallback a convención `/health` |
| Q4 | Respuesta HTTP válida | **HTTP 200** | Sin validar body |
| Q5 | Detener proceso tras auditoría | **C** | Configurable, default cleanup |
| Q6 | Nombrado de informes | **B** | Versionado por fecha |
| Q7 | Evolution Log | **B** | Solo si FAIL o PARTIAL |

---

**Clarificaciones resueltas.** Siguiente fase: planificación.
