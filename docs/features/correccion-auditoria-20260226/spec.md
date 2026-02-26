# Especificación Técnica: Corrección Auditoría 2026-02-26

## 1. Value Objects

Se crearán los siguientes archivos en `src/GesFer.Admin.Back.Domain/ValueObjects/`:

### 1.1 `ProcessId.cs`

Debe ser un `readonly record struct` que envuelva un `Guid`.
Debe implementar `IComparable<ProcessId>`.
Debe incluir `TypeConverter` para conversión desde/hacia string.
Debe incluir `JsonConverter` para serialización JSON como string (GUID).

**Interfaz Pública:**
```csharp
public readonly record struct ProcessId(Guid Value) : IComparable<ProcessId>
{
    public static ProcessId New() => new(Guid.NewGuid());
    public static ProcessId Empty => new(Guid.Empty);
    public static ProcessId Create(Guid value) => new(value);
    public static ProcessId Create(string value) => new(Guid.Parse(value));
    // ... TypeConverter y JsonConverter
}
```

### 1.2 `ActionStatus.cs`

Debe ser un `readonly record struct` que envuelva un `string`.
Debe restringir los valores a un conjunto cerrado: `Pending`, `InProgress`, `Completed`, `Failed`.

**Interfaz Pública:**
```csharp
public readonly record struct ActionStatus
{
    public static readonly ActionStatus Pending = new("Pending");
    public static readonly ActionStatus InProgress = new("InProgress");
    public static readonly ActionStatus Completed = new("Completed");
    public static readonly ActionStatus Failed = new("Failed");

    public string Value { get; }
    // ...
}
```

## 2. Corrección de Seeds (AdminJsonDataSeeder)

El problema reside en el método constructor o de inicialización de `AdminJsonDataSeeder` que intenta localizar la ruta de los seeds.
Actualmente busca `GesFer.sln` ascendiendo directorios.

**Cambio Propuesto:**
Modificar la lógica de búsqueda para:
1.  Buscar `GesFer.Admin.Back.sln` en lugar de `GesFer.sln`.
2.  Si no encuentra la solución, intentar localizar la ruta relativa `src/GesFer.Admin.Back.Infrastructure/Data/Seeds` desde el directorio base de ejecución.
3.  Asegurar que funcione tanto en ejecución local (`dotnet run`) como en tests (`dotnet test`).

## 3. Plan de Migración Rust

El documento `docs/analysis/MIGRACION_RUST_SCRIPTS.md` debe contener:
- Inventario de scripts actuales (`.ps1`, `.sh`, `.bat`).
- Propuesta de herramientas Rust equivalentes (e.g., `cargo run --bin <tool>`).
- Estimación de esfuerzo y prioridad.
