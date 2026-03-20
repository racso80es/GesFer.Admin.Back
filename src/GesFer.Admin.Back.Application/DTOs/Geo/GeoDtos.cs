namespace GesFer.Admin.Back.Application.DTOs.Geo;

public class CountryGeoReadDto
{
    public Guid Id { get; set; }
    public string Name { get; set; } = string.Empty;
    public string Code { get; set; } = string.Empty;
}

public class StateGeoReadDto
{
    public Guid Id { get; set; }
    public Guid CountryId { get; set; }
    public string Name { get; set; } = string.Empty;
    public string? Code { get; set; }
}

public class CityGeoReadDto
{
    public Guid Id { get; set; }
    public Guid StateId { get; set; }
    public string Name { get; set; } = string.Empty;
}

public class PostalCodeGeoReadDto
{
    public Guid Id { get; set; }
    public Guid CityId { get; set; }
    public string Code { get; set; } = string.Empty;
}
