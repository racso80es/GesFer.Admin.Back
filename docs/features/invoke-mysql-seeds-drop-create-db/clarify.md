---
feature: invoke-mysql-seeds-drop-create-db
date: 2026-04-28
status: pending
questions:
  - id: Q1
    title: ¿DropCreateDb debe ser siempre default?
    detail: "El requisito indica eliminar datos previos con estrategia B. Implementaremos el `.bat` para pasar `-DropCreateDb` por defecto. En CLI/envelope, `DropCreateDb` será opcional."
    default_assumption: "Sí, el `.bat` siempre pasa `-DropCreateDb` salvo que el usuario use `-NoDropCreateDb` (no se añadirá si no se pide)."
---

## Clarify

No quedan dudas bloqueantes: se implementa `DropCreateDb` y se configura el `.bat` para activarlo por defecto.

