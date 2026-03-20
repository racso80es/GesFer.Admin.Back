---
description: Objetivos de la refactorización de DTOs en base a los hallazgos de auditoría.
status: active
type: refactorization
---

# Objetivos de Corrección - 2026-03-20

1. Refactorizar todos los Data Transfer Objects (DTOs), Requests y Responses en `src/GesFer.Admin.Back.Application/DTOs/` para que sean tipos `record` en lugar de `class`, alineándose con las directrices de inmutabilidad y SddIA para C# moderno.
2. Comprobar que el proyecto compila, confirmando 100% de métricas (Arquitectura, Nomenclatura, y Estabilidad Async) tras aplicar la corrección.
3. Asegurar que los Integration Tests, Unit Tests y E2E Tests pasan satisfactoriamente.