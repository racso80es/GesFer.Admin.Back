# Plan de Migración: Tooling a Rust

**Fecha:** 2026-02-26
**Autor:** Jules (Executor)
**Contexto:** KAIZEN-3, Auditoría 2026-02-26

## 1. Introducción

El presente documento analiza el estado actual de los scripts de mantenimiento y operación del proyecto `GesFer.Admin.Back` y propone una estrategia de migración hacia herramientas desarrolladas en Rust, cumpliendo con la directiva de estandarización de tooling interno.

## 2. Inventario de Scripts Actuales

| Archivo | Propósito | Complejidad | Prioridad Migración |
| :--- | :--- | :---: | :---: |
| `Run-E2ELocal.ps1` | Orquesta la ejecución local de tests E2E (BD, API, Tests). | Alta | Alta |
| `audit_frontend_daily.py` | Script de auditoría para frontend (Python). | Media | Baja (Fuera de scope Backend puro, pero necesario) |
| `cerrar-procesos-servicios.ps1` | Limpieza de puertos y procesos zombies. | Baja | Media |
| `ejecutar-tests.ps1` | Wrapper para `dotnet test` con filtros. | Baja | Alta |
| `install-front-dependencies.ps1` | Instalación de dependencias (npm/bun). | Baja | Baja |
| `run-service-with-log.ps1` | Ejecución de servicios en background con logging. | Media | Media |
| `validate-nomenclatura.ps1` | Validación estática de nombres de archivos/carpetas. | Media | Crítica (CI/CD) |
| `validate-services-and-health.ps1` | Healthcheck de servicios docker/locales. | Media | Alta |

## 3. Estrategia de Migración

La migración se realizará creando herramientas binarias en `scripts/tools-rs/` (Cargo workspace).

### 3.1 Fase 1: Críticos y CI (Semana 1)
Migrar aquellos scripts que impactan directamente en el pipeline de Integración Continua y la validación de PRs.

*   **`validate-nomenclatura`** -> `tools-rs/validate_naming`
    *   *Beneficio:* Mayor velocidad de ejecución en CI, reglas de validación más estrictas y tipadas.
*   **`ejecutar-tests`** -> `skills-rs/run_tests` (Ya existe parcialmente, extender funcionalidad).

### 3.2 Fase 2: Operación Local (Semana 2)
Herramientas que facilitan la vida del desarrollador.

*   **`Run-E2ELocal`** + **`run-service-with-log`** + **`cerrar-procesos-servicios`** -> `tools-rs/dev_runner`
    *   *Concepto:* Una sola herramienta CLI (`gesfer-dev`) con subcomandos:
        *   `gesfer-dev start` (levanta servicios)
        *   `gesfer-dev test e2e` (ejecuta E2E)
        *   `gesfer-dev stop` (limpia procesos)
        *   `gesfer-dev health` (valida estado)

### 3.3 Fase 3: Mantenimiento y Legacy (Semana 3+)
Scripts de menor uso o de otros dominios.

*   `install-front-dependencies` -> Integrar en `dev_runner` (`gesfer-dev setup`).
*   `audit_frontend_daily.py` -> Evaluar si mover a Rust o mantener como excepción si usa librerías específicas de Python para análisis complejo.

## 4. Estructura Propuesta (Rust Workspace)

```text
scripts/tools-rs/
├── Cargo.toml (Workspace)
├── validate_naming/ (Binary)
├── dev_runner/ (Binary - "gesfer-dev")
└── common/ (Library - Shared logic for paths, logging, shell execution)
```

## 5. Acciones Inmediatas

1.  Inicializar workspace en `scripts/tools-rs/` si no existe o no está configurado como tal.
2.  Crear ticket/feature para **Fase 1**.
