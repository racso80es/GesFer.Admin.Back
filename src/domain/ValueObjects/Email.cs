using System.ComponentModel;
using System.Globalization;
using System.Text.Json;
using System.Text.Json.Serialization;
using System.Text.RegularExpressions;

namespace GesFer.Admin.Domain.ValueObjects;

[TypeConverter(typeof(EmailTypeConverter))]
[JsonConverter(typeof(EmailJsonConverter))]
public readonly record struct Email : IFormattable
{
    private static readonly Regex EmailRegex = new(
        @"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$",
        RegexOptions.Compiled | RegexOptions.IgnoreCase,
        TimeSpan.FromMilliseconds(250));
    private readonly string _value;
    public string? Value => _value;

    private Email(string value) => _value = value;

    public static Email Create(string? value)
    {
        if (string.IsNullOrWhiteSpace(value))
            throw new ArgumentException("El email no puede ser nulo o vacío.", nameof(value));
        var trimmedValue = value.Trim();
        if (!EmailRegex.IsMatch(trimmedValue))
            throw new ArgumentException($"El formato del email '{trimmedValue}' no es válido.", nameof(value));
        return new Email(trimmedValue);
    }

    public static bool TryCreate(string? value, out Email email)
    {
        email = default;
        if (string.IsNullOrWhiteSpace(value)) return false;
        var trimmedValue = value.Trim();
        if (!EmailRegex.IsMatch(trimmedValue)) return false;
        email = new Email(trimmedValue);
        return true;
    }

    public static implicit operator Email(string? value) => value == null ? throw new ArgumentNullException(nameof(value)) : Create(value);
    public static implicit operator string?(Email email) => email._value;
    public string ToString(string? format, IFormatProvider? formatProvider) => _value ?? string.Empty;
    public override string ToString() => _value ?? string.Empty;
    public bool Equals(Email other) => string.Equals(_value, other._value, StringComparison.OrdinalIgnoreCase);
    public override int GetHashCode() => _value?.ToLowerInvariant().GetHashCode() ?? 0;
}

public class EmailTypeConverter : TypeConverter
{
    public override bool CanConvertFrom(ITypeDescriptorContext? context, Type sourceType) => sourceType == typeof(string) || base.CanConvertFrom(context, sourceType);
    public override object? ConvertFrom(ITypeDescriptorContext? context, CultureInfo? culture, object value)
    {
        if (value is string stringValue && Email.TryCreate(stringValue, out var email)) return email;
        return base.ConvertFrom(context, culture, value);
    }
    public override bool CanConvertTo(ITypeDescriptorContext? context, Type? destinationType) => destinationType == typeof(string) || base.CanConvertTo(context, destinationType);
    public override object? ConvertTo(ITypeDescriptorContext? context, CultureInfo? culture, object? value, Type destinationType)
    {
        if (destinationType == typeof(string) && value is Email email) return email.Value;
        return base.ConvertTo(context, culture, value, destinationType);
    }
}

public class EmailJsonConverter : JsonConverter<Email>
{
    public override Email Read(ref Utf8JsonReader reader, Type typeToConvert, JsonSerializerOptions options)
    {
        var value = reader.GetString();
        if (string.IsNullOrWhiteSpace(value)) throw new JsonException("El email no puede ser nulo o vacío.");
        return Email.Create(value);
    }
    public override void Write(Utf8JsonWriter writer, Email value, JsonSerializerOptions options) => writer.WriteStringValue(value.Value);
}
