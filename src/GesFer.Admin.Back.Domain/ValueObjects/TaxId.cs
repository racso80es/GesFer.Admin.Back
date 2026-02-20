using System.ComponentModel;
using System.Globalization;
using System.Text.Json;
using System.Text.Json.Serialization;
using System.Text.RegularExpressions;

namespace GesFer.Admin.Back.Domain.ValueObjects;

[TypeConverter(typeof(TaxIdTypeConverter))]
[JsonConverter(typeof(TaxIdJsonConverter))]
public readonly record struct TaxId : IFormattable
{
    private static readonly Regex CifPattern = new(@"^[ABCDEFGHJNPQRSUVW][0-9]{7}[0-9A-J]$", RegexOptions.Compiled);
    private static readonly Regex NifPattern = new(@"^[0-9]{8}[TRWAGMYFPDXBNJZSQVHLCKE]$", RegexOptions.Compiled);
    private static readonly Regex NiePattern = new(@"^[XYZ][0-9]{7}[TRWAGMYFPDXBNJZSQVHLCKE]$", RegexOptions.Compiled);
    private static readonly string NifLetters = "TRWAGMYFPDXBNJZSQVHLCKE";
    private static readonly string CifLetters = "JABCDEFGHI";
    private readonly string _value;

    public string? Value => _value;
    public TaxIdType Type { get; init; }

    private TaxId(string value, TaxIdType type)
    {
        _value = value;
        Type = type;
    }

    public static TaxId Create(string? value)
    {
        if (string.IsNullOrWhiteSpace(value))
            throw new ArgumentException("El identificador fiscal no puede ser nulo o vacío.", nameof(value));
        var trimmedValue = value.Trim().ToUpperInvariant();
        if (CifPattern.IsMatch(trimmedValue))
        {
            if (!ValidateCif(trimmedValue))
                throw new ArgumentException($"El CIF '{trimmedValue}' no es válido según el algoritmo oficial.", nameof(value));
            return new TaxId(trimmedValue, TaxIdType.CIF);
        }
        if (NifPattern.IsMatch(trimmedValue))
        {
            if (!ValidateNif(trimmedValue))
                throw new ArgumentException($"El NIF '{trimmedValue}' no es válido según el algoritmo oficial.", nameof(value));
            return new TaxId(trimmedValue, TaxIdType.NIF);
        }
        if (NiePattern.IsMatch(trimmedValue))
        {
            if (!ValidateNie(trimmedValue))
                throw new ArgumentException($"El NIE '{trimmedValue}' no es válido según el algoritmo oficial.", nameof(value));
            return new TaxId(trimmedValue, TaxIdType.NIE);
        }
        throw new ArgumentException($"El formato del identificador fiscal '{trimmedValue}' no es válido. Debe ser CIF, NIF o NIE.", nameof(value));
    }

    public static bool TryCreate(string? value, out TaxId taxId)
    {
        taxId = default;
        if (string.IsNullOrWhiteSpace(value)) return false;
        try { taxId = Create(value); return true; } catch { return false; }
    }

    private static bool ValidateCif(string cif)
    {
        if (cif.Length != 9) return false;
        var numberPart = cif.Substring(1, 7);
        var controlChar = cif[8];
        var sumPairs = 0; var sumOdds = 0;
        for (int i = 0; i < 7; i++)
        {
            var digit = int.Parse(numberPart[i].ToString());
            if ((i + 1) % 2 == 0) sumPairs += digit;
            else { var doubleValue = digit * 2; sumOdds += doubleValue / 10 + doubleValue % 10; }
        }
        var total = sumPairs + sumOdds;
        var unitDigit = total % 10;
        var checkDigit = unitDigit == 0 ? 0 : 10 - unitDigit;
        if (char.IsDigit(controlChar))
            return int.Parse(controlChar.ToString()) == checkDigit;
        return CifLetters[checkDigit] == controlChar;
    }

    private static bool ValidateNif(string nif)
    {
        if (nif.Length != 9) return false;
        var numberPart = nif.Substring(0, 8);
        var letterPart = nif[8];
        if (!int.TryParse(numberPart, out var number)) return false;
        return NifLetters[number % 23] == letterPart;
    }

    private static bool ValidateNie(string nie)
    {
        if (nie.Length != 9) return false;
        var firstChar = nie[0];
        var numberPart = nie.Substring(1, 7);
        var letterPart = nie[8];
        var replacementChar = firstChar switch { 'X' => '0', 'Y' => '1', 'Z' => '2', _ => firstChar };
        if (!int.TryParse(replacementChar + numberPart, out var number)) return false;
        return NifLetters[number % 23] == letterPart;
    }

    public static implicit operator TaxId(string? value) => value == null ? throw new ArgumentNullException(nameof(value)) : Create(value);
    public static implicit operator string?(TaxId taxId) => taxId._value;
    public string ToString(string? format, IFormatProvider? formatProvider) => _value ?? string.Empty;
    public override string ToString() => _value ?? string.Empty;
    public bool Equals(TaxId other) => string.Equals(_value, other._value, StringComparison.OrdinalIgnoreCase);
    public override int GetHashCode() => _value?.ToUpperInvariant().GetHashCode() ?? 0;
}

public enum TaxIdType { CIF, NIF, NIE }

public class TaxIdTypeConverter : TypeConverter
{
    public override bool CanConvertFrom(ITypeDescriptorContext? context, Type sourceType) => sourceType == typeof(string) || base.CanConvertFrom(context, sourceType);
    public override object? ConvertFrom(ITypeDescriptorContext? context, CultureInfo? culture, object value)
    {
        if (value is string stringValue && TaxId.TryCreate(stringValue, out var taxId)) return taxId;
        return base.ConvertFrom(context, culture, value);
    }
    public override bool CanConvertTo(ITypeDescriptorContext? context, Type? destinationType) => destinationType == typeof(string) || base.CanConvertTo(context, destinationType);
    public override object? ConvertTo(ITypeDescriptorContext? context, CultureInfo? culture, object? value, Type destinationType)
    {
        if (destinationType == typeof(string) && value is TaxId taxId) return taxId.Value;
        return base.ConvertTo(context, culture, value, destinationType);
    }
}

public class TaxIdJsonConverter : JsonConverter<TaxId>
{
    public override TaxId Read(ref Utf8JsonReader reader, Type typeToConvert, JsonSerializerOptions options)
    {
        var value = reader.GetString();
        if (string.IsNullOrWhiteSpace(value)) throw new JsonException("El identificador fiscal no puede ser nulo o vacío.");
        return TaxId.Create(value);
    }
    public override void Write(Utf8JsonWriter writer, TaxId value, JsonSerializerOptions options) => writer.WriteStringValue(value.Value);
}
