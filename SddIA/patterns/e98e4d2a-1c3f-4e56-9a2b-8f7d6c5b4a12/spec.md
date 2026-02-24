# Patrón: Value Object (Objeto de Valor)

## Definición

Un **Value Object (VO)** es un objeto que no tiene identidad conceptual. Se define únicamente por sus atributos. Es inmutable y se considera igual a otro objeto si todos sus atributos son iguales.

### Características Clave
1.  **Inmutabilidad:** Una vez creado, su estado no puede cambiar.
2.  **Igualdad por Valor:** Dos VOs son idénticos si sus propiedades tienen los mismos valores.
3.  **Auto-Validación:** Un VO nunca puede existir en un estado inválido. La validación ocurre en el momento de la construcción.
4.  **Sin Ciclo de Vida:** No tiene historia independiente; se crea y se destruye según sea necesario.

---

## Implementación Estándar (.NET 8)

En el ecosistema SddIA, los Value Objects se implementan utilizando `readonly record struct` para maximizar el rendimiento y la inmutabilidad.

### Estructura Canónica

```csharp
public readonly record struct Email : IFormattable
{
    private readonly string _value;
    public string Value => _value;

    // Constructor privado para forzar el uso de Factory Methods
    private Email(string value) => _value = value;

    // Factory Method: Garantiza validación
    public static Email Create(string? value)
    {
        if (string.IsNullOrWhiteSpace(value))
            throw new ArgumentException("El email no puede ser nulo o vacío.", nameof(value));

        // Validación de dominio
        if (!IsValidFormat(value))
            throw new ArgumentException($"Formato inválido: {value}", nameof(value));

        return new Email(value.Trim());
    }

    // TryFactory Pattern: Para validaciones sin excepciones (Performance)
    public static bool TryCreate(string? value, out Email result)
    {
        result = default;
        if (string.IsNullOrWhiteSpace(value) || !IsValidFormat(value))
            return false;

        result = new Email(value.Trim());
        return true;
    }

    // Operadores implícitos (opcional, usar con precaución)
    public static implicit operator string(Email email) => email.Value;

    // Override ToString para facilitar depuración y logs
    public override string ToString() => _value;
}
```

---

## Uso Obligatorio: Interacción Procesos-Acciones

Para garantizar la integridad del sistema SddIA, la comunicación entre **Procesos** (e.g., `feature`, `bug-fix`) y **Acciones** (e.g., `spec`, `clarify`) **DEBE** realizarse mediante Value Objects, eliminando la "Obsesión por Primitivos".

### Regla de Oro
> "Ningún identificador de proceso, estado de acción o configuración crítica debe pasarse como `string` o `int` crudo."

### Ejemplos de Refactorización

#### ❌ Incorrecto (Primitive Obsession)
```csharp
public void ExecuteAction(string actionName, string status) { ... }
```

#### ✅ Correcto (Value Objects)
```csharp
public void ExecuteAction(ActionName action, ActionStatus status) { ... }
```

### Guía de Migración (Reproduction Steps)

Para adaptar entidades existentes a este patrón:

1.  **Identificar Primitivos:** Buscar argumentos `string`, `int`, `Guid` que representen conceptos de dominio (e.g., `processId`, `actionType`).
2.  **Definir VO:** Crear el `record struct` correspondiente en `Domain/ValueObjects/`.
3.  **Implementar Validación:** Mover la lógica de validación dispersa (en Services o Controllers) al método `Create()` del VO.
4.  **Sustituir en Firmas:** Reemplazar los tipos primitivos en los métodos de `Process` y `Action` por los nuevos VOs.
5.  **Añadir Conversores:** Registrar `JsonConverter` y `TypeConverter` para que la serialización (API/DB) sea transparente.

---

## Persistencia (EF Core & JSON)

### Entity Framework Core
Utilizar `HasConversion` en `OnModelCreating` para mapear el VO a la columna de base de datos.

```csharp
builder.Property(x => x.Email)
       .HasConversion(v => v.Value, v => Email.Create(v))
       .HasColumnName("Email");
```

### System.Text.Json
Implementar `JsonConverter<T>` para serializar como el valor primitivo subyacente.

```csharp
public class EmailJsonConverter : JsonConverter<Email>
{
    public override Email Read(ref Utf8JsonReader reader, Type typeToConvert, JsonSerializerOptions options)
        => Email.Create(reader.GetString());

    public override void Write(Utf8JsonWriter writer, Email value, JsonSerializerOptions options)
        => writer.WriteStringValue(value.Value);
}
```

---

## Beneficios
*   **Seguridad de Tipos:** Imposible confundir un `UserId` con un `ProductId` (ambos `Guid`, pero distintos VOs).
*   **Lógica Centralizada:** Las reglas de validación viven en un solo lugar.
*   **Código Expresivo:** `Execute(ActionStatus.Pending)` es más claro que `Execute(1)`.
