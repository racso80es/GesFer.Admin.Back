using System.ComponentModel;
using System.Globalization;
using System.Text.Json;
using System.Text.Json.Serialization;

namespace GesFer.Admin.Back.Domain.ValueObjects;

[TypeConverter(typeof(ProcessIdTypeConverter))]
[JsonConverter(typeof(ProcessIdJsonConverter))]
public readonly record struct ProcessId : IFormattable
{
    private readonly Guid _value;
    public Guid Value => _value;

    private ProcessId(Guid value) => _value = value;

    public static ProcessId Create(Guid value)
    {
        if (value == Guid.Empty)
            throw new ArgumentException("El ID del proceso no puede ser Guid.Empty.", nameof(value));
        return new ProcessId(value);
    }

    public static ProcessId Create(string? value)
    {
        if (string.IsNullOrWhiteSpace(value) || !Guid.TryParse(value, out var guid))
            throw new ArgumentException($"Formato de ProcessId inválido: {value}", nameof(value));
        return Create(guid);
    }

    public static bool TryCreate(Guid value, out ProcessId result)
    {
        result = default;
        if (value == Guid.Empty) return false;
        result = new ProcessId(value);
        return true;
    }

    public static bool TryCreate(string? value, out ProcessId result)
    {
        result = default;
        if (string.IsNullOrWhiteSpace(value) || !Guid.TryParse(value, out var guid))
            return false;
        return TryCreate(guid, out result);
    }

    public static implicit operator Guid(ProcessId id) => id.Value;
    public static implicit operator string(ProcessId id) => id.ToString();

    public override string ToString() => _value.ToString();
    public string ToString(string? format, IFormatProvider? formatProvider) => _value.ToString(format, formatProvider);
}

public class ProcessIdTypeConverter : TypeConverter
{
    public override bool CanConvertFrom(ITypeDescriptorContext? context, Type sourceType) => sourceType == typeof(string) || sourceType == typeof(Guid) || base.CanConvertFrom(context, sourceType);
    public override object? ConvertFrom(ITypeDescriptorContext? context, CultureInfo? culture, object value)
    {
        if (value is string stringValue && ProcessId.TryCreate(stringValue, out var id)) return id;
        if (value is Guid guidValue && ProcessId.TryCreate(guidValue, out id)) return id;
        return base.ConvertFrom(context, culture, value);
    }
    public override bool CanConvertTo(ITypeDescriptorContext? context, Type? destinationType) => destinationType == typeof(string) || destinationType == typeof(Guid) || base.CanConvertTo(context, destinationType);
    public override object? ConvertTo(ITypeDescriptorContext? context, CultureInfo? culture, object? value, Type destinationType)
    {
        if (value is ProcessId id)
        {
            if (destinationType == typeof(string)) return id.ToString();
            if (destinationType == typeof(Guid)) return id.Value;
        }
        return base.ConvertTo(context, culture, value, destinationType);
    }
}

public class ProcessIdJsonConverter : JsonConverter<ProcessId>
{
    public override ProcessId Read(ref Utf8JsonReader reader, Type typeToConvert, JsonSerializerOptions options)
    {
        if (reader.TokenType == JsonTokenType.String && Guid.TryParse(reader.GetString(), out var guid))
            return ProcessId.Create(guid);
        throw new JsonException("Formato JSON inválido para ProcessId.");
    }
    public override void Write(Utf8JsonWriter writer, ProcessId value, JsonSerializerOptions options) => writer.WriteStringValue(value.Value);
}
