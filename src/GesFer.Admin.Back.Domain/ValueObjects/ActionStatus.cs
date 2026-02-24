using System.ComponentModel;
using System.Globalization;
using System.Text.Json;
using System.Text.Json.Serialization;

namespace GesFer.Admin.Back.Domain.ValueObjects;

[TypeConverter(typeof(ActionStatusTypeConverter))]
[JsonConverter(typeof(ActionStatusJsonConverter))]
public readonly record struct ActionStatus : IFormattable
{
    private readonly string _value;
    public string Value => _value;

    private ActionStatus(string value) => _value = value;

    public static readonly ActionStatus Pending = new("Pending");
    public static readonly ActionStatus Running = new("Running");
    public static readonly ActionStatus Completed = new("Completed");
    public static readonly ActionStatus Failed = new("Failed");
    public static readonly ActionStatus Cancelled = new("Cancelled");

    private static readonly HashSet<string> ValidValues = new(StringComparer.OrdinalIgnoreCase)
    {
        "Pending", "Running", "Completed", "Failed", "Cancelled"
    };

    public static ActionStatus Create(string? value)
    {
        if (string.IsNullOrWhiteSpace(value) || !ValidValues.Contains(value.Trim()))
            throw new ArgumentException($"Estado de acción inválido: {value}", nameof(value));
        return new ActionStatus(value.Trim());
    }

    public static bool TryCreate(string? value, out ActionStatus result)
    {
        result = default;
        if (string.IsNullOrWhiteSpace(value) || !ValidValues.Contains(value.Trim()))
            return false;
        result = new ActionStatus(value.Trim());
        return true;
    }

    public static implicit operator string(ActionStatus status) => status.Value;
    public override string ToString() => _value ?? "Pending"; // Default to Pending if empty (should not happen via Create)
    public string ToString(string? format, IFormatProvider? formatProvider) => ToString();
}

public class ActionStatusTypeConverter : TypeConverter
{
    public override bool CanConvertFrom(ITypeDescriptorContext? context, Type sourceType) => sourceType == typeof(string) || base.CanConvertFrom(context, sourceType);
    public override object? ConvertFrom(ITypeDescriptorContext? context, CultureInfo? culture, object value)
    {
        if (value is string stringValue && ActionStatus.TryCreate(stringValue, out var status)) return status;
        return base.ConvertFrom(context, culture, value);
    }
    public override bool CanConvertTo(ITypeDescriptorContext? context, Type? destinationType) => destinationType == typeof(string) || base.CanConvertTo(context, destinationType);
    public override object? ConvertTo(ITypeDescriptorContext? context, CultureInfo? culture, object? value, Type destinationType)
    {
        if (destinationType == typeof(string) && value is ActionStatus status) return status.Value;
        return base.ConvertTo(context, culture, value, destinationType);
    }
}

public class ActionStatusJsonConverter : JsonConverter<ActionStatus>
{
    public override ActionStatus Read(ref Utf8JsonReader reader, Type typeToConvert, JsonSerializerOptions options) => ActionStatus.Create(reader.GetString());
    public override void Write(Utf8JsonWriter writer, ActionStatus value, JsonSerializerOptions options) => writer.WriteStringValue(value.Value);
}
