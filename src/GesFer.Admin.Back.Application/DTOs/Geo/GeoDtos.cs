namespace GesFer.Admin.Back.Application.DTOs.Geo;

public class CountryDto
{
    public Guid Id { get; set; }
    public string Name { get; set; } = string.Empty;
    public string Code { get; set; } = string.Empty;
    public Guid LanguageId { get; set; }
}

public class StateDto
{
    public Guid Id { get; set; }
    public Guid CountryId { get; set; }
    public string Name { get; set; } = string.Empty;
    public string? Code { get; set; }
}

public class CityDto
{
    public Guid Id { get; set; }
    public Guid StateId { get; set; }
    public string Name { get; set; } = string.Empty;
}
