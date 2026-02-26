using System.ComponentModel;
using System.Globalization;
using System.Text.Json;
using System.Text.Json.Serialization;

namespace GesFer.Admin.Back.Domain.ValueObjects;

[TypeConverter(typeof(ProcessIdTypeConverter))]
[JsonConverter(typeof(ProcessIdJsonConverter))]
public readonly record struct ProcessId(Guid Value) : IComparable<ProcessId>
{
    public static ProcessId New() => new(Guid.NewGuid());
    public static ProcessId Empty => new(Guid.Empty);

    public static ProcessId Create(Guid value)
    {
        if (value == Guid.Empty)
        {
            throw new ArgumentException("El ID del proceso no puede ser Guid.Empty.", nameof(value));
        }
        return new ProcessId(value);
    }

    public static ProcessId Create(string value)
    {
        if (string.IsNullOrWhiteSpace(value))
        {
            throw new ArgumentException("El ID del proceso no puede ser nulo o vacío.", nameof(value));
        }
        if (!Guid.TryParse(value, out var guid))
        {
            throw new ArgumentException($"El valor '{value}' no es un GUID válido.", nameof(value));
        }
        return new ProcessId(guid);
    }

    public static bool TryCreate(string? value, out ProcessId processId)
    {
        processId = default;
        if (string.IsNullOrWhiteSpace(value)) return false;
        if (!Guid.TryParse(value, out var guid)) return false;
        if (guid == Guid.Empty) return false;

        processId = new ProcessId(guid);
        return true;
    }

    public override string ToString() => Value.ToString();

    public int CompareTo(ProcessId other) => Value.CompareTo(other.Value);

    public static implicit operator Guid(ProcessId id) => id.Value;
    public static implicit operator ProcessId(Guid id) => new(id);
}

public class ProcessIdTypeConverter : TypeConverter
{
    public override bool CanConvertFrom(ITypeDescriptorContext? context, Type sourceType) => sourceType == typeof(string) || base.CanConvertFrom(context, sourceType);
    public override object? ConvertFrom(ITypeDescriptorContext? context, CultureInfo? culture, object value)
    {
        if (value is string stringValue && ProcessId.TryCreate(stringValue, out var processId)) return processId;
        return base.ConvertFrom(context, culture, value);
    }
    public override bool CanConvertTo(ITypeDescriptorContext? context, Type? destinationType) => destinationType == typeof(string) || base.CanConvertTo(context, destinationType);
    public override object? ConvertTo(ITypeDescriptorContext? context, CultureInfo? culture, object? value, Type destinationType)
    {
        if (destinationType == typeof(string) && value is ProcessId processId) return processId.ToString();
        return base.ConvertTo(context, culture, value, destinationType);
    }
}

public class ProcessIdJsonConverter : JsonConverter<ProcessId>
{
    public override ProcessId Read(ref Utf8JsonReader reader, Type typeToConvert, JsonSerializerOptions options)
    {
        var value = reader.GetString();
        if (string.IsNullOrWhiteSpace(value)) throw new JsonException("El ProcessId no puede ser nulo o vacío.");
        return ProcessId.Create(value);
    }
    public override void Write(Utf8JsonWriter writer, ProcessId value, JsonSerializerOptions options) => writer.WriteStringValue(value.Value.ToString());
}
