# GesFer.Admin.Back.ArchitectureTests

Pruebas de **arquitectura** (límites entre capas): comprueban referencias entre ensamblados del backend para que no se introduzcan dependencias prohibidas (estilo Clean / DDD ligero del repo).

## Qué protege

- **Domain** no depende de Application, Infrastructure ni Api.
- **Application** no depende de Infrastructure ni Api (solo Domain y bibliotecas neutras).
- **Infrastructure** puede depender de Application y Domain; no debe depender de Api.
- **Api** puede depender de Application e Infrastructure (y Domain vía transitivas), coherente con la solución actual.

Ampliar reglas aquí si el grafo permitido del solution cambia de forma explícita.
