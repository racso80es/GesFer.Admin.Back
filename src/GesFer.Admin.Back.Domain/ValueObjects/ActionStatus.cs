using System.Text.Json;
using System.Text.Json.Serialization;

namespace GesFer.Admin.Back.Domain.ValueObjects;

[JsonConverter(typeof(ActionStatusJsonConverter))]
public readonly record struct ActionStatus
{
    public static readonly ActionStatus Pending = new("Pending");
    public static readonly ActionStatus InProgress = new("InProgress");
    public static readonly ActionStatus Completed = new("Completed");
    public static readonly ActionStatus Failed = new("Failed");

    public string Value { get; }

    private ActionStatus(string value) => Value = value;

    public static ActionStatus Create(string value)
    {
        if (string.IsNullOrWhiteSpace(value))
        {
            throw new ArgumentException("El estado de la acción no puede ser nulo o vacío.", nameof(value));
        }

        return value switch
        {
            "Pending" => Pending,
            "InProgress" => InProgress,
            "Completed" => Completed,
            "Failed" => Failed,
            _ => throw new ArgumentException($"El estado '{value}' no es válido.", nameof(value))
        };
    }

    public static implicit operator string(ActionStatus status) => status.Value;

    public override string ToString() => Value;

    public bool Equals(ActionStatus other) => string.Equals(Value, other.Value, StringComparison.OrdinalIgnoreCase);
    public override int GetHashCode() => Value?.GetHashCode() ?? 0;
}

public class ActionStatusJsonConverter : JsonConverter<ActionStatus>
{
    public override ActionStatus Read(ref Utf8JsonReader reader, Type typeToConvert, JsonSerializerOptions options)
    {
        var value = reader.GetString();
        if (string.IsNullOrWhiteSpace(value)) throw new JsonException("El ActionStatus no puede ser nulo o vacío.");
        return ActionStatus.Create(value);
    }

    public override void Write(Utf8JsonWriter writer, ActionStatus value, JsonSerializerOptions options)
    {
        writer.WriteStringValue(value.Value);
    }
}
