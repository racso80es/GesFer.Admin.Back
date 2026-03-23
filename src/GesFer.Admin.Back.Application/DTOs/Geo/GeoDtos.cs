namespace GesFer.Admin.Back.Application.DTOs.Geo;

public record CountryGeoReadDto
{
    public Guid Id { get; set; }
    public string Name { get; set; } = string.Empty;
    public string Code { get; set; } = string.Empty;
}

public record StateGeoReadDto
{
    public Guid Id { get; set; }
    public Guid CountryId { get; set; }
    public string Name { get; set; } = string.Empty;
    public string? Code { get; set; }
}

public record CityGeoReadDto
{
    public Guid Id { get; set; }
    public Guid StateId { get; set; }
    public string Name { get; set; } = string.Empty;
}

public record PostalCodeGeoReadDto
{
    public Guid Id { get; set; }
    public Guid CityId { get; set; }
    public string Code { get; set; } = string.Empty;
}
