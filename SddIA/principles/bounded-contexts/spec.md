# Bounded Contexts

**principle_id:** `bounded-contexts`

## Resumen

Los Bounded Contexts conforman límites lógicos y organizacionales dentro del diseño guiado por el dominio (DDD) que sirven para delimitar distintas áreas y departamentos de la empresa. Facilitan que un mismo concepto comercial disponga de una representación y un modelo independientes según la mirada del equipo que lo utilice (por ejemplo, el departamento de plataforma frente al departamento de backoffice), previniendo fricciones semánticas y un alto acoplamiento interdepartamental.

## Objetivo

Delimitar fronteras claras donde un mismo término (ej. "Cliente", "Pedido") puede tener significado, reglas y modelos distintos, evitando un modelo único forzado que genere acoplamiento y conflictos.

## Aplicación para Arquitecto

- Identificar y documentar los bounded contexts del producto (ej. Gestión Ferial, Backoffice, Notificaciones) y sus relaciones (ACL, eventos, APIs).
- Definir contratos entre contextos (eventos de dominio, DTOs de API) y evitar que un contexto importe entidades de otro; comunicar por eventos o servicios anticorrupción.

## Aplicación para Tekton

- Respetar los límites de módulos o bounded contexts: no referenciar entidades de otro contexto directamente; usar IDs, eventos o DTOs de integración.
- Al añadir una funcionalidad, ubicarla en el contexto correcto y exponer integración vía eventos o APIs bien definidas, no mediante referencias cruzadas a base de datos o modelos internos.

## Referencias

[19], [20] — DDD, Bounded Context, modelo por contexto.

---
*Definición en paths.principlesPath/bounded-contexts/ (contrato paths.principlesPath/principles-contract.md).*
