# Especificación de Corrección - 2026-03-19

## 1. Contexto SddIA
Este documento formaliza el resultado de la auditoría diaria reportada en `docs/audits/AUDITORIA_2026_03_19.md`.

## 2. Alcance
* **Nuevas Funcionalidades:** Ninguna.
* **Correcciones:** Conversión de DTOs y Request/Responses a tipo `record` para reducir la deuda técnica y mejorar inmutabilidad.
* **Proceso Activo:** `SddIA/process/correccion-auditorias`.

## 3. Hoja de Ruta del Ejecutor
1. Identificar y modificar las declaraciones `public class` por `public record` en todos los archivos del directorio `src/GesFer.Admin.Back.Application/DTOs/`.
2. Verificar que el código compila con `dotnet build`.
3. Validar las pruebas integrales de los controladores que usan estos DTOs con `dotnet test`.

## 4. Definition of Done
* El proyecto compila sin alertas ni errores tras los cambios.
* Los tests pasan al 100%, incluyendo IntegrationTests.
* El informe de validación en `validacion.json` refleja métricas de salud al 100%.