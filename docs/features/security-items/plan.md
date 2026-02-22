# Plan de Ejecución: Creación de Items de Seguridad

Este documento describe el plan de acción para crear 20 items de seguridad en `SddIA/security/`, siguiendo el patrón de diseño SddIA y el modelo de seguridad `Karma2Token`.

## 1. Establecer Contrato del Módulo de Seguridad
*   Crear el directorio `SddIA/security/`.
*   Crear `SddIA/security/security-contract.json`: Definir el esquema JSON obligatorio para cada item.
*   Crear `SddIA/security/security-contract.md`: Documentar el contrato y la estructura de carpetas.

## 2. Implementar Items de Seguridad (Lote 1: 1-10)
*   **SQL Injection (Inyección SQL)**
*   **Consultas Parametrizadas (Prepared Statements)**
*   **JSON Web Tokens (JWT) y su Integridad**
*   **Criptografía Asimétrica en JWT**
*   **El Usuario como Vulnerabilidad Principal**
*   **Centro de Operaciones de Seguridad (SOC)**
*   **Red Team vs Pen Testing**
*   **Bug Bounty Programs**
*   **Formación mediante CTF (Capture The Flag)**
*   **Ataques de Supply Chain en Dependencias (npm)**

*   **Entregables por Item:**
    *   Carpeta UUID única en `SddIA/security/`.
    *   `spec.json`: Metadatos estructurados.
    *   `spec.md`: Contenido Markdown formateado (Descripción, Riesgo, Mitigación, Referencias).

## 3. Implementar Items de Seguridad (Lote 2: 11-20)
*   **Ejecución de malware vía scripts postinstall**
*   **Bash Injection en GitHub Actions**
*   **Fuga de Secretos en Repositorios y ficheros .env**
*   **Gestión de Secretos con Vaults**
*   **Cifrado Físico de Dispositivos**
*   **Rate Limiting y Prevención de Scraping**
*   **Firmas de Commits con GPG**
*   **Modo Vigilante (Vigilant Mode) en GitHub**
*   **Asegurando APIs con Helmet y CORS**
*   **Actualizaciones Automáticas con Dependabot**

*   **Entregables por Item:**
    *   Carpeta UUID única en `SddIA/security/`.
    *   `spec.json`: Metadatos estructurados.
    *   `spec.md`: Contenido Markdown formateado.

## 4. Generar Artefactos de Ejecución y Validación
*   Generar `docs/features/security-items/execution.json`: Registro de archivos y acciones realizadas.
*   Generar `docs/features/security-items/validacion.json`: Confirmación de existencia, integridad y validez de los 20 items.

## 5. Verificación y Entrega
*   Verificar que `Karma2Token` esté referenciado en todos los contratos.
*   Correr verificaciones pre-commit.
*   Entregar cambios en rama `feat/create-security-items` (o similar).
